advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy)]
struct Range {
    lower: u64,
    upper: u64
}

impl Range {
    pub fn contains(&self, other: u64) -> bool {
        other >= self.lower && other <= self.upper
    }

    pub fn overlapping_range(&self, other: &Range) -> Option<Range> {
        if self.lower <= other.upper && self.upper >= other.lower {
            let lower = if self.lower < other.lower { self.lower } else { other.lower };
            let upper = if self.upper > other.upper { self.upper } else { other.upper };

            return Some(Range {lower, upper});
        }

        None
    }

    pub fn size(&self) -> u64 {
        self.upper - self.lower + 1
    }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    let mut past_blank = false;
    for line in input.lines() {
        if line == "" {
            past_blank = true
        } else {
            if past_blank {
                ingredients.push(line.parse().unwrap())
            } else {
                let mut parts = line.split("-");
                let range = Range {
                    lower: parts.next().unwrap().parse().unwrap(),
                    upper: parts.next().unwrap().parse().unwrap()
                };
                ranges.push(range)
            }
        }
    }

    (ranges, ingredients)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = parse_input(input);

    let count = ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(**ingredient)))
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _ingredients) = parse_input(input);

    let mut ranges = ranges;
    let mut simplifications_happened = true;
    while simplifications_happened {
        let mut simplified_ranges: Vec<Range> = Vec::new();
        simplifications_happened = false;

        for range in ranges.iter() {
            // Check to see if it can meld with any current ranges
            let current_range_result =
                simplified_ranges
                    .iter()
                    .enumerate()
                    .find(|(_i, r)| r.overlapping_range(&range).is_some());

            match current_range_result {
                Some((index, matched_range)) => {
                    simplifications_happened = true;
                    simplified_ranges[index] = range.overlapping_range(&matched_range).unwrap();
                },
                None => { simplified_ranges.push(*range); }
            }
        }

        ranges = simplified_ranges;
    }

    // Simplify our ranges
    let count: u64 = ranges.iter().map(|range| range.size()).sum();

    Some(count as u64)
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
        assert_eq!(result, Some(14));
    }
}
