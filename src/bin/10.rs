use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    light_state: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>
}

fn strip_outer(token: &str) -> &str {
    &token[1..token.len()-1]
}

fn line_to_machine(line: &str) -> Machine {
    let mut line_iterator = line.split(" ").peekable();

    // The first item is the light array
    let light_state = strip_outer(line_iterator.next().unwrap())
        .chars()
        .map(|c| c == '#')
        .collect();

    let mut buttons = Vec::new();
    let mut joltages = Vec::new();
    while let Some(item) = line_iterator.next() {
        if line_iterator.peek().is_some() {
            // All middle items are the buttons
            buttons.push(
                strip_outer(item)
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect()
            )
        } else {
            // The last button is the joltages
            joltages = strip_outer(item)
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
        }
    }

    Machine { light_state, buttons, joltages }
}

// A useful button changes at least one light to the desired state
fn is_useful_button(button: &Vec<usize>, desired_lights: &Vec<bool>, current_lights: &Vec<bool>) -> bool {
    let lights_diff: Vec<bool> = (0..desired_lights.len())
        .map(|i| desired_lights[i] ^ current_lights[i])
        .collect();

    button.iter().any(|&l_index| lights_diff[l_index])
}


// For part 2, a useful button is one that does not go above the desired numbers
fn is_useful_button_p2(button: &Vec<usize>, desired_joltages: &Vec<usize>, current_joltages: &Vec<usize>) -> bool {
    for joltage_index in 0..desired_joltages.len() {
        if desired_joltages[joltage_index] != current_joltages[joltage_index] { continue; }

        if let Some(_) = button.iter().find(|&&button_index| button_index == joltage_index) {
            return false;
        }
    }

    true
}

fn recurse_option_pt1(machine: &Machine, current_lights: &Vec<bool>, buttons_pressed: &Vec<usize>, best_solution: &mut Option<Vec<usize>>) {
    // If we have a match, add it to the working solutions and return
    let lights_match = (0..current_lights.len())
        .all(|index| current_lights[index] == machine.light_state[index]);

    if lights_match {
        *best_solution = Some(buttons_pressed.clone());
        return
    }

    // If another button press gives a solution longer than an existing one, don't bother
    if let Some(shortest_working_solution) = best_solution {
        if buttons_pressed.len() >= shortest_working_solution.len() {
            return
        }
    }

    // Find all valid buttons that could be pushed
    let potential_presses = machine
        .buttons
        .iter()
        .enumerate()
        .filter(|(button_index, button)|
            is_useful_button(button, &machine.light_state, current_lights) &&
            !buttons_pressed.iter().any(|pressed_index| pressed_index == button_index)
        );

    // If there are no options, dead end
    if potential_presses.clone().count() == 0 {
        return
    }

    // For each one, press it and then recurse
    for (button_index, button) in potential_presses {
        // Update the state after the button press
        let updated_lights = (0..current_lights.len())
            .map(|i| current_lights[i] ^ button.iter().find(|&&light_index| light_index == i).is_some())
            .collect();

        // Clone buttons pressed and pass it along
        let mut pressed_clone = buttons_pressed.clone();
        pressed_clone.push(button_index);
        recurse_option_pt1(machine, &updated_lights, &pressed_clone, best_solution);
    }
}

fn recurse_option_pt2(machine: &Machine, button_indeces: &Vec<usize>, current_joltages: &Vec<usize>, count: usize, best_solution: &mut Option<usize>, cache: &mut HashMap<Vec<usize>, usize>) {
    // If we have been to this state before in fewer buttons, return
    if let Some(&best_count) = cache.get(current_joltages) {
        if best_count <= count {
            return;
        }
    }

    // If we have a match, add it to the working solutions and return
    let joltages_match = (0..current_joltages.len())
        .all(|index| current_joltages[index] == machine.joltages[index]);

    if joltages_match {
        println!("Found solution: {:?}", count);
        *best_solution = Some(count);
        return
    }

    // If another button press gives a solution longer than an existing one, don't bother
    if let Some(shortest_working_solution) = best_solution {
        if count >= *shortest_working_solution {
            return
        }
    }

    // Otherwise, we are going to try pressing all the buttons. Add this to the cache so that
    // we don't try it again
    cache.insert(current_joltages.clone(), count);

    // Find all valid buttons that could be pushed, sorted by most effects first (i.e. prefer more impactful presses)
    let mut indeces_copy = button_indeces.clone();
    indeces_copy.retain(|index| is_useful_button_p2(&machine.buttons[*index], &machine.joltages, current_joltages));

    if indeces_copy.len() == 0 {
        return;
    }

    // For each one, press it and then recurse
    for button_index in indeces_copy.iter() {
        let button = &machine.buttons[*button_index];

        // Update the state after the button press
        let updated_joltages = (0..current_joltages.len())
            .map(|i| {
                let adder = match button.iter().find(|&&joltage_index| joltage_index == i) {
                    None => 0,
                    Some(_) => 1
                };

                current_joltages[i] + adder
            })
            .collect();

        // Try this path
        recurse_option_pt2(machine, &indeces_copy, &updated_joltages, count + 1, best_solution, cache);
    }
}

fn solve_machine_part_1(machine: &Machine) -> u64 {
    // Set up the initial state
    let mut current_lights = Vec::new();
    for _ in 0..machine.light_state.len() { current_lights.push(false); }

    let mut best_solution = None;
    recurse_option_pt1(machine, &current_lights, &Vec::new(), &mut best_solution);

    // Find the fewest button presses
    best_solution.unwrap().len() as u64
}

fn solve_machine_part_2(machine: &mut Machine) -> u64 {
    // Set up the initial state
    let mut current_joltages = Vec::new();
    for _ in 0..machine.joltages.len() { current_joltages.push(0); }

    let mut best_solution = None;
    machine.buttons.sort_by(|a, b| a.len().cmp(&b.len()));
    let button_indeces = (0..machine.buttons.len()).collect();
    let mut cache = HashMap::new();

    recurse_option_pt2(machine, &button_indeces, &current_joltages, 0, &mut best_solution, &mut cache);

    // Find the fewest button presses
    best_solution.unwrap() as u64
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(line_to_machine)
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let result = input
        .iter()
        .map(|m| solve_machine_part_1(m))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = parse_input(input);

    let result = input
        .iter_mut()
        .map(|m| {
            println!("Solving: {:?}", m.light_state);
            solve_machine_part_2(m)
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
