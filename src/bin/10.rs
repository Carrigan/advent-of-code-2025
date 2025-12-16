use std::collections::HashMap;

use good_lp::*;

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
    let mut vars = ProblemVariables::new();
    let num_buttons = machine.buttons.len();

    // Create variables dynamically
    let button_vars: Vec<Variable> = (0..num_buttons)
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    // Objective: minimize sum of all button presses
    let objective: Expression = button_vars.iter()
        .map(|&v| Expression::from(v))
        .sum();

    // Build constraints dynamically
    let mut problem = vars.minimise(objective).using(default_solver);

    for (counter_idx, &target) in machine.joltages.iter().enumerate() {
        // Sum up effects of all buttons on this counter
        let constraint_expr: Expression = button_vars
            .iter()
            .enumerate()
            .map(|(btn_idx, &var)| {
                match machine.buttons[btn_idx].contains(&counter_idx) {
                    true => 1 * var,
                    false => 0 * var
                }
            })
            .sum();

        problem = problem.with(constraint!(constraint_expr == target as f64));
    }

    // Solve
    let solution = problem.solve()
        .map_err(|e| format!("Solver error: {:?}", e)).unwrap();

    // Extract results
    let total_presses: i32 = button_vars.iter()
        .map(|&v| solution.value(v) as i32)
        .sum();

    total_presses as u64
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
