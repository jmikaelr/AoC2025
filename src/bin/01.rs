advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut pos: i32 = 50;
    let size: i32 = 100;
    let mut count: u64 = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        let (dir, num_str) = line.split_at(1);
        let dist: i32 = num_str.parse().ok()?;

        match dir {
            "R" => {
                pos = (pos + dist) % size;
            }
            "L" => {
                pos = (pos - dist).rem_euclid(size);
            }
            _ => {
                return None;
            }
        }

        if pos == 0 {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pos: i32 = 50;
    let size: i32 = 100;
    let mut count: u64 = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        let (dir, num_str) = line.split_at(1);
        let dist: i32 = num_str.parse().ok()?;

        let s = pos;
        let d = dist;

        let k0 = match dir {
            "R" => {
                if s == 0 {
                    size
                } else {
                    size - s
                }
            }
            "L" => {
                if s == 0 {
                    size
                } else {
                    s
                }
            }
            _ => return None,
        };

        if d >= k0 {
            count += 1 + ((d - k0) / size) as u64;
        }
        pos = match dir {
            "R" => (s + d) % size,
            "L" => (s - d).rem_euclid(size),
            _ => unreachable!(),
        };
    }
    Some(count)
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
