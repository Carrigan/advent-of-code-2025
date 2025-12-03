advent_of_code::solution!(3);

fn find_largest(slice: &[u32]) -> (usize, u32) {
    let mut largest = slice[0];
    let mut index = 0;

    for (i, digit) in slice[1..].iter().enumerate() {
        if *digit > largest {
            index = i + 1;
            largest = *digit;
        }
    }

    (index, largest)
}

fn line_to_joltage(line: &Vec<u32>, n_digits: usize) -> u64 {
    let line_length = line.len();
    let mut largest_array: Vec<u32> = Vec::new();
    let mut current_index = 0;

    // Build up an array in least significant ordering
    for n in 0..n_digits {
        let reserved_chars = line_length - n_digits + n + 1;
        let current_search = &line.as_slice()[current_index..reserved_chars];
        let (index, largest) = find_largest(current_search);

        current_index = current_index + index + 1;
        largest_array.push(largest);
    }

    // Turn the array into a number
    let mut output: u64 = 0;
    for digit in largest_array {
        output = (output * 10) + digit as u64;
    }

    output
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .map(|line| line_to_joltage(&line, 2))
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .map(|line| line_to_joltage(&line, 12))
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
