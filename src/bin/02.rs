advent_of_code::solution!(2);

fn is_valid_pt1(num: u64) -> bool {
    // See how many digits the number has
    let digits = (num as f64).log10().floor() as u64 + 1;

    if digits % 2 == 1 { return true };

    // Split the number
    let splitter = (10 as f64).powi((digits / 2) as i32) as u64;
    let first = num / splitter;
    let second = num % splitter;

    first != second
}

fn is_valid_pt2(num: u64) -> bool {
    // Start by breaking apart the number into an array of digits
    let mut digits: Vec<u64> = Vec::new();
    let mut num_copy = num;
    while num_copy > 0 {
        digits.push(num_copy % 10);
        num_copy /= 10;
    }

    // Flip it so most significant is first
    digits.reverse();

    let digit_count = digits.len();

    // digit_block here gives how large the grouping of digits we are
    // looking at is
    for digit_block in 1..=digit_count / 2 {
        // If we cannot evenly divide, keep going
        if digit_count % digit_block > 0 { continue; }

        let is_match = digits
            .iter()
            .enumerate()
            .all(|(i, digit)| digits[i % digit_block] == *digit);

        if is_match {
            return false;
        }
    };

    true
}

fn solve_with_fn(input: &str, validator: &dyn Fn(u64) -> bool) -> u64{
    input
        .split(",")
        .map(|range_str| {
            let parts = range_str.split("-").collect::<Vec<&str>>();
            let lower =  parts[0].parse::<u64>().unwrap();
            let upper =  parts[1].parse::<u64>().unwrap();

            (lower..=upper)
                .filter(|&num| !validator(num))
                .sum::<u64>()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_with_fn(input, &is_valid_pt1))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve_with_fn(input, &is_valid_pt2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(is_valid_pt1(1221));
        assert!(!is_valid_pt1(123123));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
