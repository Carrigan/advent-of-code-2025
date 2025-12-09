use itertools::Itertools;

advent_of_code::solution!(9);

struct Point {
    x: u64,
    y: u64
}

fn intersects_right(point: &Point, edge_point_1: &Point, edge_point_2: &Point) -> bool {
    // Do not consider horizontal lines for this
    if edge_point_1.y == edge_point_2.y {
        return false;
    }

    // Check the Y intercept
    let min_y = [edge_point_1, edge_point_2].iter().map(|p| p.y).min().unwrap();
    let max_y = [edge_point_1, edge_point_2].iter().map(|p| p.y).max().unwrap();
    let is_within_y = point.y >= min_y && point.y < max_y;

    // Check the X. Since we know this is not a horizontal line, it must be vertical which
    // means we can assume the two X points are equal
    let is_left_x = point.x <= edge_point_1.x;

    is_left_x && is_within_y
}

fn on_edge(point: &Point, edge_point_1: &Point, edge_point_2: &Point) -> bool {
    if edge_point_1.x == edge_point_2.x {
        if point.x != edge_point_1.x {
            return false;
        }

        let min_y = edge_point_1.y.min(edge_point_2.y);
        let max_y = edge_point_1.y.max(edge_point_2.y);
        point.y >= min_y && point.y <= max_y
    } else {
        if point.y != edge_point_1.y {
            return false;
        }

        let min_x = edge_point_1.x.min(edge_point_2.x);
        let max_x = edge_point_1.x.max(edge_point_2.x);
        point.x >= min_x && point.x <= max_x
    }
}

fn edges_properly_intersect(r1: &Point, r2: &Point, p1: &Point, p2: &Point) -> bool {
    // Skip if edges share an endpoint (touching is OK)
    if (r1.x == p1.x && r1.y == p1.y) || (r1.x == p2.x && r1.y == p2.y) ||
       (r2.x == p1.x && r2.y == p1.y) || (r2.x == p2.x && r2.y == p2.y) {
        return false;
    }

    // Check if rectangle edge is horizontal
    if r1.y == r2.y {
        // Check if polygon edge is vertical
        if p1.x == p2.x {
            let rx_min = r1.x.min(r2.x);
            let rx_max = r1.x.max(r2.x);
            let py_min = p1.y.min(p2.y);
            let py_max = p1.y.max(p2.y);

            // They intersect if polygon's x is between rect's x range
            // and rect's y is between polygon's y range
            return p1.x > rx_min && p1.x < rx_max &&
                   r1.y > py_min && r1.y < py_max;
        }
    } else {
        // Rectangle edge is vertical
        // Check if polygon edge is horizontal
        if p1.y == p2.y {
            let ry_min = r1.y.min(r2.y);
            let ry_max = r1.y.max(r2.y);
            let px_min = p1.x.min(p2.x);
            let px_max = p1.x.max(p2.x);

            return p1.y > ry_min && p1.y < ry_max &&
                   r1.x > px_min && r1.x < px_max;
        }
    }

    false
}

impl Point {
    fn rectangle_area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    fn is_on(&self, polygon: &Vec<Point>) -> bool {
        polygon
            .iter()
            .circular_tuple_windows::<(_, _)>()
            .any(|(ep1, ep2)| on_edge(self, ep1, ep2))
    }

    fn is_inside_or_on(&self, polygon: &Vec<Point>) -> bool {
        // Check if the point is on an edge directly
        if self.is_on(polygon) {
            return true;
        }

        // If not, we need to raycast, making the end points of each edge
        // inclusive. Lets shoot just one ray to the right
        let intersection_count = polygon
            .iter()
            .circular_tuple_windows::<(_, _)>()
            .filter(|(ep1, ep2)| intersects_right(self, ep1, ep2))
            .count();

        intersection_count % 2 == 1
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(",");
            Point {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap()
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let points_count = points.len();
    let mut possibilities: Vec<u64> = Vec::new();

    for index_1 in 0..points_count - 1 {
        for index_2 in index_1 + 1..points_count {
            possibilities.push(points[index_1].rectangle_area(&points[index_2]))
        }
    }

    let best = possibilities.iter().max().unwrap();

    Some(*best)
}

fn connected_by_red_tiles(polygon: &Vec<Point>, point_1: &Point, point_2: &Point) -> bool {
    if (point_1.x == point_2.x) || (point_1.y == point_2.y) {
        return true;
    }

    let other_points = [
        Point { x: point_1.x, y: point_2.y },
        Point { x: point_2.x, y: point_1.y },
    ];

    let all_points_inside_or_on =
        other_points
        .iter()
        .all(|p| p.is_inside_or_on(polygon));

    if !all_points_inside_or_on { return false; }

    // Check that rectangle edges don't cross polygon edges
    let rect_edges = [
        (point_1, &other_points[0]),
        (&other_points[0], point_2),
        (point_2, &other_points[1]),
        (&other_points[1], point_1)
    ];

    for (r1, r2) in rect_edges.iter() {
        for (p1, p2) in polygon.iter().circular_tuple_windows::<(_, _)>() {
            if edges_properly_intersect(r1, r2, p1, p2) {
                return false;
            }
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let points_count = points.len();
    let mut possibilities: Vec<u64> = Vec::new();

    for index_1 in 0..points_count - 1 {
        for index_2 in index_1 + 1..points_count {
            let p1 = &points[index_1];
            let p2 = &points[index_2];

            if connected_by_red_tiles(&points, p1, p2) {
                possibilities.push(p1.rectangle_area(p2))
            }
        }
    }

    let best = possibilities.iter().max().unwrap();

    Some(*best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
