use itertools::Itertools;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.split(",").map(|n| n.parse().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap())
        })
        .collect()
}

fn distance(pt1: (usize, usize, usize), pt2: (usize, usize, usize)) -> f64 {
    let sum_of_powers =
        ((pt1.0).abs_diff(pt2.0)).pow(2) +
        ((pt1.1).abs_diff(pt2.1)).pow(2) +
        ((pt1.2).abs_diff(pt2.2)).pow(2);

    (sum_of_powers as f64).powf(0.5)
}

fn compute_all_distances(points: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize, f64)> {
    let mut result = Vec::new();
    for first_index in 0..points.len() - 1 {
        for second_index in first_index + 1..points.len() {
            result.push(
                (first_index, second_index, distance(points[first_index], points[second_index]))
            );
        }
    }

    // Sort this vector by distance, descending
    result.sort_by(|&a, &b| (a.2).total_cmp(&b.2));

    result
}

fn try_make_connection(distance: (usize, usize, f64), connections: &mut Vec<Vec<usize>>) {
    // Start by finding all current connections that touch one of the junctions
    // being looked at
    let connection_indeces_found: Vec<usize> = connections
        .iter()
        .enumerate()
        .filter_map(|(index, connection_graph)|
            if connection_graph.iter().filter(|&&c| c == distance.0 || c == distance.1).count() > 0 {
                Some(index)
            } else {
                None
            }
        )
        .collect();

    // Based on the number of touched connections, add the new connection to the
    // graphs
    match connection_indeces_found.len() {
        // For 0, create a whole new connection graph
        0 => connections.push(vec![distance.0, distance.1]),

        // For 1, see if it connects with just one or both points of the connection
        // graph and connect accordingly.
        1 => {
            let connection_index = connection_indeces_found[0];
            let left_match = connections[connection_index].iter().find(|&&n| n == distance.0);
            let right_match = connections[connection_index].iter().find(|&&n| n == distance.1);

            match (left_match, right_match) {
                (None, None) => panic!(),
                (None, Some(_)) => connections[connection_index].push(distance.0),
                (Some(_), None) => connections[connection_index].push(distance.1),
                _ => {}

            }
        },

        // For 2, merge the two connection indeces
        2 => {
            let &lower_index = [connection_indeces_found[0], connection_indeces_found[1]].iter().min().unwrap();
            let &higher_index = [connection_indeces_found[0], connection_indeces_found[1]].iter().max().unwrap();
            let removed_connection = connections.remove(higher_index);
            let connection_to_add_to = &mut connections[lower_index];

            connection_to_add_to.extend(removed_connection.iter());
        },

        // If there are more than 2, uh oh!
        _ => panic!()
    }
}

fn part_one_partial(input: &str, n_connections: usize) -> u64 {
    let points = parse_input(input);
    let distances = compute_all_distances(&points);

    // Grab the nearest N connections and make some graphs
    let mut connections: Vec<Vec<usize>> = Vec::new();

    for distance in distances.iter().take(n_connections) {
        try_make_connection(*distance, &mut connections);
    }

    // Finally, make a product of the count of each connection graph
    connections
        .iter()
        .map(|graph| graph.len() as u64)
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(part_one_partial(input, 1000))
}

fn all_connections_made(connections: &Vec<Vec<usize>>, point_count: usize) -> bool {
    connections
        .iter()
        .map(|c| c.len())
        .sum::<usize>() == point_count
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let point_count = points.len();
    let distances = compute_all_distances(&points);

    let mut connections: Vec<Vec<usize>> = Vec::new();
    let mut last_connection_index = 0;
    while !all_connections_made(&connections, point_count) {
        try_make_connection(distances[last_connection_index], &mut connections);
        last_connection_index += 1;
    }

    let last_point_1 = points[distances[last_connection_index - 1].0];
    let last_point_2 = points[distances[last_connection_index - 1].1];

    Some(last_point_1.0 as u64 * last_point_2.0 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_partial(
            &advent_of_code::template::read_file("examples", DAY),
            10
        );
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
