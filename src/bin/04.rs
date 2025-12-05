advent_of_code::solution!(4);

type Grid = Vec<Vec<char>>;
const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_one(input: &str) -> Option<u64> {
    let mut neighbour_count = 0;
    let grid = parse_grid(input);
    // dbg!(grid);
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' {
                let n = count_neighbours(&grid, row, col);
                if n < 4 {
                    neighbour_count += 1;
                }
            }
        }
    }

    Some(neighbour_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);
    let mut removed_papers = 0;
    loop {
        let mut neighbour_count = 0;
        // dbg!(grid);
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == '@' {
                    let n = count_neighbours(&grid, row, col);
                    if n < 4 {
                        neighbour_count += 1;
                        grid[row][col] = '.';
                    }
                }
            }
        }
        // println!("{}", neighbour_count);
        removed_papers += neighbour_count;
        if neighbour_count == 0 {
            break;
        }
    }
    Some(removed_papers)
}

fn parse_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_neighbours(grid: &Grid, row: usize, col: usize) -> usize {
    let mut count = 0;
    for (dr, dc) in DIRS {
        let nr = row as isize + dr;
        let nc = col as isize + dc;
        if nr >= 0
            && (nr as usize) < grid.len()
            && nc >= 0
            && (nc as usize) < grid[0].len()
            && grid[nr as usize][nc as usize] == '@'
        {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
