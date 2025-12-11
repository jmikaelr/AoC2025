use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);

    let max_end = ranges.iter().map(|&(_, b)| b).max().unwrap_or(0);
    let max_digits = max_end.to_string().len();

    let mut pow10 = vec![1u64; max_digits + 1];
    for i in 1..max_digits {
        pow10[i] = pow10[i - 1].saturating_mul(10);
    }

    let mut sum = 0u64;

    for half_len in 1..=max_digits / 2 {
        let pow_len = pow10[half_len];
        let start = pow_len / 10;
        let end = pow_len - 1;

        for k in start..=end {
            let candidate = match k.checked_mul(pow_len).and_then(|x| x.checked_add(k)) {
                Some(v) => v,
                None => break,
            };

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
    let max_digits = max_end.to_string().len();

    let mut pow10 = vec![1u64; max_digits + 1];
    for i in 1..max_digits {
        pow10[i] = pow10[i - 1].saturating_mul(10);
    }

    let mut sum: u64 = 0;
    let mut seen: HashSet<u64> = HashSet::new();

    for half_len in 1..=max_digits / 2 {
        let pow_len = pow10[half_len];
        let start = pow_len / 10;
        let end = pow_len - 1;

        for k in start..=end {
            let mut candidate = 0u64;
            let mut m = 0usize;

            loop {
                candidate = match candidate
                    .checked_mul(pow_len)
                    .and_then(|x| x.checked_add(k))
                {
                    Some(v) => v,
                    None => break,
                };

                m += 1;

                if half_len * m > max_digits {
                    break;
                }

                if m < 2 {
                    continue;
                }

                if candidate > max_end {
                    break;
                }

                if ranges
                    .iter()
                    .any(|&(lo, hi)| candidate >= lo && candidate <= hi)
                    && seen.insert(candidate)
                {
                    sum += candidate;
                }
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
