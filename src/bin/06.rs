advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let rows = lines.len();
    let cols = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    let mut grid = vec![vec![' '; cols]; rows];
    for (r, line) in lines.iter().enumerate() {
        // println!("r: {}, line {}", r, line);
        for (c, ch) in line.chars().enumerate() {
            // println!("c {}, ch {}", c, ch);
            grid[r][c] = ch;
        }
    }

    let mut col_is_blank = vec![false; cols];
    for c in 0..cols {
        col_is_blank[c] = (0..rows).all(|r| grid[r][c] == ' ');
    }

    let mut total = 0u64;
    let mut c = 0;
    while c < cols {
        if col_is_blank[c] {
            c += 1;
            continue;
        }

        let start = c;
        while c < cols && !col_is_blank[c] {
            c += 1;
        }
        let end = c - 1;

        let op_row = rows - 1;

        let mut op = None;
        for (_, ch) in grid[op_row]
            .iter()
            .enumerate()
            .take(end + 1)
            .skip(start)
        {
            if *ch == '+' || *ch == '*' {
                op = Some(ch);
                break;
            }
        }

        let op = op.unwrap();

        let mut nums: Vec<u64> = Vec::new();
        for (r, _) in grid.iter().enumerate().take(op_row) {
            let slice: String = (start..=end).map(|col| grid[r][col]).collect();
            let s = slice.trim();
            if !s.is_empty() {
                let val: u64 = s.parse().unwrap();
                nums.push(val);
            }
        }

        let value = match op {
            '+' => nums.iter().copied().sum::<u64>(),
            '*' => nums.iter().copied().product::<u64>(),
            _ => unreachable!(),
        };

        total += value;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let rows = lines.len();
    let cols = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    let mut grid = vec![vec![' '; cols]; rows];
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid[r][c] = ch;
        }
    }

    let mut col_is_blank = vec![false; cols];
    for c in 0..cols {
        col_is_blank[c] = (0..rows).all(|r| grid[r][c] == ' ');
    }

    let mut total = 0u64;
    let mut c = 0;
    let op_row = rows - 1;

    while c < cols {
        if col_is_blank[c] {
            c += 1;
            continue;
        }

        let start = c;
        while c < cols && !col_is_blank[c] {
            c += 1;
        }
        let end = c - 1;

        let mut op = None;
        for (_, ch) in grid[op_row]
            .iter()
            .enumerate()
            .take(end + 1)
            .skip(start)
        {
            if *ch == '+' || *ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.unwrap();

        let mut nums: Vec<u64> = Vec::new();

        for (col, _) in grid[0]
            .iter()
            .enumerate()
            .skip(start)
            .take(end - start + 1)
            .rev()
        {
            let mut s = String::new();

            for row in grid.iter().take(op_row) {
                let ch = row[col];
                if ch.is_ascii_digit() {
                    s.push(ch);
                }
            }

            if !s.is_empty() {
                let val: u64 = s.parse().unwrap();
                nums.push(val);
            }
        }

        let value: u64 = match op {
            '+' => nums.iter().copied().sum::<u64>(),
            '*' => nums.iter().copied().product::<u64>(),
            _ => unreachable!(),
        };

        total += value;
    }
    Some(total)
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
