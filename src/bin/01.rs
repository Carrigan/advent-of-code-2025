advent_of_code::solution!(1);

enum Direction {
    LEFT,
    RIGHT
}

struct Instruction {
    direction: Direction,
    amount: u16
}

impl Instruction {
    // Parse a single instruction from a &str
    fn from_str(s: &str) -> Option<Instruction> {
        let (dir, amt) = s.split_at(1);
        let direction = match dir {
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => return None
        };
        let amount = amt.parse::<u16>().ok()?;
        Some(Instruction { direction, amount })
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_str = match self.direction {
            Direction::LEFT => "L",
            Direction::RIGHT => "R"
        };
        write!(f, "{}{}", dir_str, self.amount)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // For each line, create an instruction
    let instructions: Vec<Instruction> = input.lines()
        .filter_map(|line| Instruction::from_str(line))
        .collect();

    let mut count_zero = 0;
    let mut current_value = 50;

    for instruction in instructions {
        let amount = instruction.amount % 100;

        match instruction.direction {
            Direction::LEFT => current_value -= amount as i32,
            Direction::RIGHT => current_value += amount as i32,
        }

        if current_value < 0 {
            current_value = 100 + current_value;
        }

        if current_value > 99 {
            current_value = current_value - 100;
        }

        if current_value == 0 {
            count_zero += 1;
        }
    }

    Some(count_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    // For each line, create an instruction
    let instructions: Vec<Instruction> = input.lines()
        .filter_map(|line| Instruction::from_str(line))
        .collect();

    let mut count_zero: u64 = 0;
    let mut current_value = 50;

    for instruction in instructions {
        let amount = instruction.amount % 100;
        count_zero += (instruction.amount / 100) as u64;
        let start_value = current_value;

        match instruction.direction {
            Direction::LEFT => current_value -= amount as i32,
            Direction::RIGHT => current_value += amount as i32,
        }

        if current_value < 0 {
            current_value = 100 + current_value;

            if start_value != 0 {
                count_zero += 1;
            }
        }

        else if current_value > 99 {
            current_value = current_value - 100;

            if start_value != 0 {
                count_zero += 1;
            };
        }

        else if current_value == 0 {
            count_zero += 1;
        }
    }

    Some(count_zero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
