advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(u64),
    Multiplication,
    Addition
}

impl Token {
    fn as_number(&self) -> u64 {
        match self {
            Token::Number(i) => *i,
            _ => panic!()
        }
    }

    fn perform<'a>(&self, numbers: impl Iterator<Item = &'a Token>) -> u64 {
        match self {
            Token::Multiplication => numbers.map(|n| n.as_number()).product(),
            Token::Addition => numbers.map(|n| n.as_number()).sum(),
            Token::Number(_) => panic!()
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "*" => Ok(Token::Multiplication),
            "+" => Ok(Token::Addition),
            _ => {
                match s.parse() {
                    Ok(num) => Ok(Token::Number(num)),
                    Err(_) => Err(())
                }
            }
        }
    }
}

impl TryFrom<char> for Token {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '*' => Ok(Token::Multiplication),
            '+' => Ok(Token::Addition),
            _ => Err(())
        }
    }
}

// Format: 2d array that matches the input
fn parse_input_p1(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|line| line.split(" ").filter_map(|t| Token::try_from(t).ok()).collect())
        .collect()
}

// Format: Each output is the operator followed by the other token types
fn parse_input_p2(input: &str) -> Vec<Vec<Token>> {
    let lines: Vec<&str> = input.lines().collect();
    let longest_line = lines.iter().map(|l| l.len()).max().unwrap();
    let mut output = Vec::new();

    // Temp variables to hold each of the numbers and the operator
    let mut tokens = Vec::new();
    let mut first_column = true;

    // Scan the index left to right
    for index in 0..longest_line {
        // If we are at the column gap, process the buffer and rest
        if lines.iter().all(|line| line.chars().nth(index).unwrap_or(' ') == ' ') {
            output.push(tokens);
            first_column = true;
            tokens = Vec::new();
            continue;
        }

        // Otherwise add to the buffer
        if first_column {
            tokens.push(Token::try_from(lines.last().unwrap().chars().nth(index).unwrap()).unwrap());
            first_column = false;
        }

        let mut current_number = 0;
        for row_index in 0..lines.len() - 1 {
            let current_character = lines[row_index].chars().nth(index).unwrap_or(' ');
            if let Some(i) = current_character.to_digit(10) {
                current_number = current_number * 10 + i as u64;
            };
        }

        tokens.push(Token::Number(current_number));
    }

    // Flush the last buffer
    output.push(tokens);

    output
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut tokens = parse_input_p1(input);
    let col_count = tokens[0].len();

    // Lop off the last row
    let operations = tokens.pop().unwrap();

    let result = (0..col_count)
        .map(|col_i|
            operations[col_i].perform(tokens.iter().map(|row| row.get(col_i).unwrap()))
        )
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let tokens = parse_input_p2(input);

    let result = tokens
        .iter()
        .map(|t_row| {
            let mut row_iterator = t_row.iter();
            let operator = row_iterator.next().unwrap();
            operator.perform(row_iterator)
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
