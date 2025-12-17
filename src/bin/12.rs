advent_of_code::solution!(12);

// All pieces must be 3x3 in format:
// 0 1 2
// 3 4 5
// 6 7 8
type Piece = [bool; 9];

struct Placement {
    x: usize,
    y: usize,
    orientation_index: usize
}

fn orientations(piece: &Piece) -> [Piece; 4] {
    [
        [
            piece[0], piece[1], piece[2],
            piece[3], piece[4], piece[5],
            piece[6], piece[7], piece[8],
        ],
        [
            piece[6], piece[3], piece[0],
            piece[7], piece[4], piece[1],
            piece[8], piece[5], piece[2],
        ],
        [
            piece[8], piece[7], piece[6],
            piece[5], piece[4], piece[3],
            piece[2], piece[1], piece[0],
        ],
        [
            piece[2], piece[5], piece[8],
            piece[1], piece[4], piece[7],
            piece[0], piece[3], piece[6],
        ]
    ]
}

fn valid_placements(board_state: &Vec<bool>, width: usize, height: usize, piece: &Piece) -> Vec<Placement> {
    let mut placements = Vec::new();

    for (orientation_index, oriented_piece) in orientations(piece).iter().enumerate() {
        if board_state.iter().all(|p| !p) {
            // Placement of the first one will be upper left
            placements.push(Placement{ x: 0, y: 0, orientation_index });
        } else {
            for row_i in 0..(height - 2) {
                for col_i in 0..(width - 2) {
                    if board_state[width * row_i + col_i] { continue; }

                    // Otherwise we are in the first place we might fit this; check if it fits
                    let mut collision = false;
                    for piece_index in 0..9 {
                        if !oriented_piece[piece_index] { continue; }

                        let x_offset = piece_index % 3;
                        let y_offset = piece_index / 3;

                        if board_state[(width * (row_i + y_offset)) + col_i + x_offset] {
                            collision = true;
                            break;
                        }
                    }

                    // If there was no collision, it is valid
                    if !collision {
                        placements.push(Placement { x: col_i, y: row_i, orientation_index });
                    }

                    // Either way, break to the next line
                    break;
                }
            }
        }
    }

    placements
}


#[derive(Debug)]
struct Puzzle {
    width: usize,
    height: usize,
    piece_counts: Vec<usize>,
    board_state: Vec<bool>
}

fn recurse_puzzle(board_state: &Vec<bool>, piece_counts: &Vec<usize>, pieces: &[Piece], width: usize, height: usize) -> bool {
    // Iterate through all available piece types
    let available_piece_indeces = piece_counts
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p { 0 => None, _ => Some(i) });

    for piece_index in available_piece_indeces {
        let piece = &pieces[piece_index];

        for valid_placement in valid_placements(&board_state, width, height, piece) {
            // We are going to move on with a new board state. Clone the current board state and counts
            let mut new_counts = piece_counts.clone();

            // Subtract one from the counts
            new_counts[piece_index] -= 1;

            // If this was the last piece and there is a valid placement, we are done
            if new_counts.iter().all(|&c| c == 0) {
                return true;
            }

            // Apply the piece (this could probably be more efficient)
            let mut new_state = board_state.clone();
            let oriented_piece = orientations(piece)[valid_placement.orientation_index];
            for piece_index in 0..9 {
                if !oriented_piece[piece_index] { continue; }

                let x_offset = piece_index % 3;
                let y_offset = piece_index / 3;
                new_state[(valid_placement.y + y_offset) * width + valid_placement.x + x_offset] = true;
            }

            // Otherwise, recurse and return if a true is found
            let any_trues_in_this_path = recurse_puzzle(
                &new_state, &new_counts, pieces, width, height
            );

            if any_trues_in_this_path {
                return true;
            }
        }
    }

    false
}

impl Puzzle {
    // Naive implementation: try every orientation until something works
    fn solve(&self, pieces: &[Piece]) -> bool {
        println!("Solving puzzle {:?}", self.piece_counts);
        recurse_puzzle(&self.board_state, &self.piece_counts, pieces, self.width, self.height)
    }
}

fn parse_input(input: &str) -> ([Piece; 6], Vec<Puzzle>) {
    let mut lines = input.lines();
    let mut pieces: [Piece; 6] = [[false; 9]; 6];
    let mut puzzles = Vec::new();

    // There are always 6 pieces
    for piece_index in 0..6 {
        // Burn the first line
        lines.next().unwrap();

        for line_index in 0..3 {
            let line = lines.next().unwrap();
            for (char_index, c) in line.chars().enumerate() {
                pieces[piece_index][line_index * 3 + char_index] = c == '#';
            }
        }

        // Burn the last line
        lines.next().unwrap();
    }

    // Everything else is puzzles
    for line in lines {
        let mut sides = line.split(":");
        let mut w_h = sides.next().unwrap().split("x");
        let piece_counts_str = sides.next().unwrap();
        let width = w_h.next().unwrap().parse().unwrap();
        let height = w_h.next().unwrap().parse().unwrap();
        let piece_counts = piece_counts_str
            .split(" ")
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();

        // Initialize board state to empty
        let board_state = vec![false; width * height];

        puzzles.push(Puzzle { width, height, piece_counts, board_state });
    }

    (pieces, puzzles)
}



pub fn part_one(input: &str) -> Option<u64> {
    let (pieces, puzzles) = parse_input(input);

    let can_fit = puzzles
        .iter()
        .filter(|puzzle| puzzle.solve(&pieces))
        .count() as u64;

    Some(can_fit)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
