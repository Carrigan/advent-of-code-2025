use std::{collections::HashMap};

advent_of_code::solution!(11);

fn index_or_create(nodes: &mut Vec<String>, this_node: String) -> usize {
    match nodes.iter().enumerate().find(|(_, n)| *n == &this_node) {
        Some((i, _)) => i,
        None => {
            nodes.push(this_node);
            nodes.len() - 1
        }
    }
}

pub fn parse_input(input: &str) -> (Vec<String>, HashMap<usize, Vec<usize>>) {
    let mut nodes = Vec::new();
    let mut edges = HashMap::new();

    for line in input.lines() {
        let mut connections = Vec::new();
        let this_node = line[0..3].to_string();
        let this_node_index = index_or_create(&mut nodes, this_node);

        let connection_nodes = line[5..]
            .split(" ")
            .map(|n| n.to_string());

        for connection_node in connection_nodes {
            let connection_node_index = index_or_create(&mut nodes, connection_node);
            connections.push(connection_node_index);
        }

        edges.insert(this_node_index, connections);
    }

    (nodes, edges)
}

fn start_recurse(start_index: usize, end_index: usize, nodes: &Vec<String>, edges: &HashMap<usize, Vec<usize>>) -> u64 {
    let mut cache = HashMap::new();
    recurse_path_count(
        start_index,
        end_index,
        nodes,
        edges,
        &mut cache
    )
}

fn recurse_path_count(start_index: usize, end_index: usize, nodes: &Vec<String>, edges: &HashMap<usize, Vec<usize>>, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(count) = cache.get(&(start_index, end_index)) {
        return *count;
    }

    let mut count = 0;

    let maybe_outputs = edges.get(&start_index);
    if maybe_outputs.is_none() {
        return 0;
    }

    for edge in maybe_outputs.unwrap() {
        if *edge == end_index {
            count += 1;
        } else {
            //let mut new_nodes_visited = nodes_visited.clone();
            //new_nodes_visited.push(*edge);

            let result = recurse_path_count(
                //new_nodes_visited,
                *edge,
                end_index,
                nodes,
                edges,
                cache
            );

            cache.insert((*edge, end_index), result);
            count += result;
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let (nodes, edges) = parse_input(input);

    // Find the indeces for "you" and "out"
    let you = nodes.iter().position(|n| n == "you").unwrap();
    let out = nodes.iter().position(|n| n == "out").unwrap();

    // Lets go
    Some(start_recurse(you, out, &nodes, &edges))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (nodes, edges) = parse_input(input);

    // Find the indeces for relevant parts
    let svr = nodes.iter().position(|n| n == "svr").unwrap();
    let fft = nodes.iter().position(|n| n == "fft").unwrap();
    let dac = nodes.iter().position(|n| n == "dac").unwrap();
    let out = nodes.iter().position(|n| n == "out").unwrap();

    let svr_to_fft = start_recurse(svr, fft, &nodes, &edges);
    let fft_to_dac = start_recurse(fft, dac, &nodes, &edges);
    let dac_to_out = start_recurse(dac, out, &nodes, &edges);

    Some(svr_to_fft * fft_to_dac * dac_to_out)
}

#[cfg(test)]
mod tests {
    use advent_of_code::template::Day;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", Day::new(13).unwrap()));
        assert_eq!(result, Some(2));
    }
}
