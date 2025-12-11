use std::cmp::{max, min};
advent_of_code::solution!(9);

#[derive(Clone, Copy)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn calculate_area(&self, other: &Point) -> isize {
        ((self.row - other.row).abs() + 1) * ((self.col - other.col).abs() + 1)
    }
}

struct Candidate {
    corner1: Point,
    corner2: Point,
    area: isize,
}

fn calculate_all_area(points: &[Point]) -> Vec<Candidate> {
    let mut candidates = Vec::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let corner1 = points[i];
            let corner2 = points[j];
            let area = points[i].calculate_area(&corner2);

            candidates.push(Candidate {
                corner1,
                corner2,
                area,
            });
        }
    }
    candidates
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);

    let max_area = calculate_all_area(&points)
        .iter()
        .map(|candidate| candidate.area.max(0) as u64)
        .max()
        .unwrap_or(0);

    Some(max_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);

    let mut candidates = calculate_all_area(&points);

    candidates.sort_by_key(|a| -a.area);

    let mut edges: Vec<(Point, Point)> = points.windows(2).map(|w| (w[0], w[1])).collect();

    edges.push((points[points.len() - 1].clone(), points[0].clone()));

    let ans = candidates
        .iter()
        .find(|candidate| is_valid_rectangle(candidate, &edges))
        .map(|c| c.area)
        .unwrap_or(0);
    Some(ans as u64)
}

fn is_valid_rectangle(candidate: &Candidate, edges: &[(Point, Point)]) -> bool {
    let min_row = min(candidate.corner1.row, candidate.corner2.row);
    let max_row = max(candidate.corner1.row, candidate.corner2.row);
    let min_col = min(candidate.corner1.col, candidate.corner2.col);
    let max_col = max(candidate.corner1.col, candidate.corner2.col);

    for (p1, p2) in edges {
        let e_min_row = min(p1.row, p2.row);
        let e_max_row = max(p1.row, p2.row);
        let e_min_col = min(p1.col, p2.col);
        let e_max_col = max(p1.col, p2.col);

        if min_row < e_max_row && max_row > e_min_row && min_col < e_max_col && max_col > e_min_col
        {
            return false;
        }
    }
    true
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point::new(x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None); // you probably want to change this to Some(50)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None); // and this to Some(24) when you're ready
    }
}
