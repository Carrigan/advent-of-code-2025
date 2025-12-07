use std::collections::HashMap;

advent_of_code::solution!(7);

struct State {
    start: (usize, usize),
    row_count: usize,
    splitters: Vec<(usize, usize)>
}

impl State {
    fn hit_splitter(&self, position: (usize, usize)) -> bool {
        self
            .splitters
            .iter()
            .find(|(s_x, s_y)| *s_x == position.0 && *s_y == position.1)
            .is_some()
    }
}

fn parse_input(input: &str) -> State {
    let row_count = input.lines().count();
    let mut start = (0, 0);
    let mut splitters = Vec::new();

    for (y, row) in input.lines().enumerate() {
        for (x, character) in row.chars().enumerate() {
            if character == 'S' {
                start = (x, y);
            } else if character == '^' {
                splitters.push((x, y));
            }
        }
    }

    State { row_count, start, splitters }
}

fn push_location_if_unique(positions: &mut Vec<(usize, usize)>, position: (usize, usize)) {
    let exists = positions
        .iter()
        .find(|(s_x, s_y)| *s_x == position.0 && *s_y == position.1)
        .is_some();

    if exists { return; }

    positions.push(position);
}

fn solve_pt1(state: &State) -> u64 {
    let mut beam_positions = Vec::new();
    let mut split_count = 0;

    // Start at the start position
    beam_positions.push(state.start);

    for row_index in 0..state.row_count {
        let mut temp_positions = Vec::new();

        for (beam_x, _beam_y) in beam_positions {
            // Check if we hit a splitter
            if state.hit_splitter((beam_x, row_index)) {
                split_count += 1;
                push_location_if_unique(&mut temp_positions, (beam_x - 1, row_index));
                push_location_if_unique(&mut temp_positions, (beam_x + 1, row_index));
            } else {
                push_location_if_unique(&mut temp_positions, (beam_x, row_index));
            }
        }

        beam_positions = temp_positions;
    }

    split_count
}

fn solve_pt2(state: &State) -> u64 {
    let current_position = state.start;
    let mut cached_answers: HashMap<(usize, usize), u64> = HashMap::new();
    recurse_pt2(state, &mut cached_answers, current_position)
}

fn recurse_pt2(state: &State, cached_answers: &mut HashMap<(usize, usize), u64>, current_position: (usize, usize)) -> u64 {
    // Progress once
    let next_position = (current_position.0, current_position.1 + 1);

    // If we are at the end, return one
    if next_position.1 == state.row_count - 1 {
        return 1;
    }

    // If we have been here before, return the cached answer
    if let Some(answer) = cached_answers.get(&next_position) {
        return *answer;
    }

    // If we hit a splitter, recurse both options
    if state.hit_splitter(next_position) {
        let left = recurse_pt2(state, cached_answers, (next_position.0 - 1, next_position.1));
        cached_answers.insert((next_position.0 - 1, next_position.1), left);

        let right = recurse_pt2(state, cached_answers, (next_position.0 + 1, next_position.1));
        cached_answers.insert((next_position.0 + 1, next_position.1), right);

        cached_answers.insert(next_position, left + right);
        return left + right;
    }

    // Otherwise continue down
    let result = recurse_pt2(state, cached_answers, next_position);
    cached_answers.insert(next_position, result);

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let state = parse_input(input);
    Some(solve_pt1(&state))
}

pub fn part_two(input: &str) -> Option<u64> {
    let state = parse_input(input);
    Some(solve_pt2(&state))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
