advent_of_code::solution!(7);

type Grid = Vec<Vec<char>>;

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let h = grid.len();
    let w = grid.first()?.len();

    let mut start_beam = None;
    'outer: for (r, _) in grid.iter().enumerate().take(h) {
        for (c, _) in grid.iter().enumerate().take(w) {
            if grid[r][c] == 'S' {
                if r + 1 < h {
                    start_beam = Some((r + 1, c))
                }
                break 'outer;
            }
        }
    }
    let (sr, sc) = start_beam?;

    let mut splits = 0u64;

    let mut current = vec![false; w];
    current[sc] = true;

    for row in grid.iter().take(h).skip(sr + 1) {
        let mut next = vec![false; w];

        for c in 0..w {
            if !current[c] {
                continue;
            }

            match row[c] {
                '.' | 'S' | '|' => {
                    next[c] = true;
                }
                '^' => {
                    splits += 1;

                    if c > 0 {
                        next[c - 1] = true;
                    }
                    if c + 1 < w {
                        next[c + 1] = true;
                    }
                }
                _ => {}
            }
        }
        current = next;
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let h = grid.len();
    let w = grid.first()?.len();

    let mut start_beam = None;
    'outer: for (r, _) in grid.iter().enumerate().take(h) {
        for (c, _) in grid.iter().enumerate().take(w) {
            if grid[r][c] == 'S' {
                if r + 1 < h {
                    start_beam = Some((r + 1, c))
                }
                break 'outer;
            }
        }
    }
    let (sr, sc) = start_beam?;

    let mut ways = vec![vec![0u64; w]; h];
    ways[sr][sc] = 1;

    let mut total = 0u64;

    for (r, row) in grid.iter().enumerate().take(h) {
        for c in 0..w {
            let k = ways[r][c];
            if k == 0 {
                continue;
            }

            match row[c] {
                '.' | 'S' | '|' => {
                    if r + 1 < h {
                        ways[r + 1][c] += k;
                    } else {
                        total += k;
                    }
                }
                '^' => {
                    if r + 1 < h {
                        if c > 0 {
                            ways[r + 1][c - 1] += k;
                        } else {
                            total += k;
                        }

                        if c + 1 < w {
                            ways[r + 1][c + 1] += k;
                        } else {
                            total += k;
                        }
                    } else {
                        total += k;
                    }
                }
                _ => {
                    total += k;
                }
            }
        }
    }
    Some(total)
}

fn parse_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
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
