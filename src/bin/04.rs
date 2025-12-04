advent_of_code::solution!(4);

fn line_to_vector(line: &str) -> Vec<bool> {
    line
        .chars()
        .map(|c| c == '@')
        .collect()
}

fn input_to_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line_to_vector(&line))
        .collect()
}

fn is_accessible(row_n: usize, col_n: usize, row_count: usize, col_count: usize, grid: &Vec<Vec<bool>>) -> bool {
    // Allow overflow
    let row_n = row_n as i32;
    let col_n = col_n as i32;


    let adjacent_items: [(i32, i32); 8] = [
        (col_n - 1, row_n - 1), (col_n, row_n - 1), (col_n + 1, row_n - 1),
        (col_n - 1, row_n),                         (col_n + 1, row_n),
        (col_n - 1, row_n + 1), (col_n, row_n + 1), (col_n + 1, row_n + 1)
    ];

    let adjacent_count = adjacent_items
        .iter()
        .filter(|(col, row)|
            *col >= 0 && *col < (col_count as i32) && *row >= 0 && *row < (row_count as i32)
        )
        .filter(|(col, row)|
            grid[*row as usize][*col as usize]
        )
        .count();

    adjacent_count < 4
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input_to_grid(input);
    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut viable_spaces = 0;
    for row in 0..row_count {
        for col in 0..col_count {
            if grid[row][col] && is_accessible(row, col, row_count, col_count, &grid) {
                viable_spaces += 1
            }
        }
    }

    Some(viable_spaces)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = input_to_grid(input);
    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut removed = 0;
    let mut temp_grid: Vec<Vec<bool>>;
    loop {
        let mut removed_this_iteration = 0;
        temp_grid = Vec::new();

        for row in 0..row_count {
            let mut row_vec = Vec::new();

            for col in 0..col_count {
                if grid[row][col] {
                    if is_accessible(row, col, row_count, col_count, &grid) {
                        removed_this_iteration += 1;
                        row_vec.push(false);
                    } else {
                        row_vec.push(true);
                    }
                } else {
                    row_vec.push(false);
                }
            }

            temp_grid.push(row_vec);
        }

        removed += removed_this_iteration;
        if removed_this_iteration == 0 { break; }
        grid = temp_grid;
    }

    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
