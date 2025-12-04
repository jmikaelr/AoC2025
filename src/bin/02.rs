use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);

    let max_end = ranges.iter().map(|&(_, b)| b).max().unwrap_or(0);
    let max_digits = max_end.to_string().len();

    let mut sum = 0;

    for half_len in 1..=max_digits / 2 {
        let start = 10u64.pow(half_len as u32 - 1); // 10^0 ... 10^1 ... 10^(n-1)
        let end = 10u64.pow(half_len as u32) - 1; // 10^1 - 1 ... 10^2 ... 10^n - 1
        // dbg!(half_len);
        // dbg!(start);
        // dbg!(end);

        for k in start..=end {
            let s = k.to_string();
            let candidate = format!("{s}{s}").parse::<u64>().unwrap(); // concat string

            if candidate > max_end {
                break;
            }

            if ranges
                .iter()
                .any(|&(lo, hi)| candidate >= lo && candidate <= hi)
            {
                sum += candidate;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);

    let max_end = ranges.iter().map(|&(_, b)| b).max().unwrap_or(0);
    let max_digits = max_end.to_string().len() as u64;

    let mut seen: HashSet<u64> = HashSet::new();
    let mut sum: u64 = 0;

    for half_len in 1..=max_digits / 2 {
        let start = 10u64.pow(half_len as u32 - 1); // 10^0 ... 10^1 ... 10^(n-1)
        let end = 10u64.pow(half_len as u32) - 1; // 10^1 - 1 ... 10^2 ... 10^n - 1
        // dbg!(half_len);
        // dbg!(start);
        // dbg!(end);

        for k in start..=end {
            let s = k.to_string();

            let mut m = 2;
            loop {
                let total_len = half_len * m;
                if total_len > max_digits {
                    break;
                }

                let candidate_str = s.repeat(m as usize);
                let candidate: u64 = candidate_str.parse().unwrap();

                if candidate > max_end {
                    break;
                }

                if ranges
                    .iter()
                    .any(|&(lo, hi)| candidate >= lo && candidate <= hi)
                {
                    if seen.insert(candidate) {
                        sum += candidate;
                    }
                }

                m += 1;
            }
        }
    }
    Some(sum)
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|r| {
            let (a, b) = r.split_once('-').unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect()
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
