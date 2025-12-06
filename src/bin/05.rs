advent_of_code::solution!(5);

#[derive(Debug, Eq, PartialEq, Hash)]
struct Range {
    low: i64,
    high: i64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_input(input);

    let fresh_count = ids.into_iter().filter(|id| is_fresh(*id, &ranges)).count() as u64;
    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse_input(input);

    ranges.sort_by_key(|r| r.low);

    let mut merged: Vec<Range> = Vec::new();

    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r.low <= last.high {
                if r.high > last.high {
                    last.high = r.high;
                }
            } else {
                merged.push(r);
            }
        } else {
            merged.push(r);
        }
    }

    let total = merged.iter().map(|r| (r.high - r.low + 1) as u64).sum();

    Some(total)
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<i64>) {
    let (ranges_part, ids_part) = input.split_once("\n\n").unwrap();

    let ranges = ranges_part
        .lines()
        .map(|line| {
            let (low_str, high_str) = line.split_once("-").unwrap();
            Range {
                low: low_str.parse().unwrap(),
                high: high_str.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let ids = ids_part
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    (ranges, ids)
}

fn is_fresh(id: i64, ranges: &[Range]) -> bool {
    ranges.iter().any(|r| r.low <= id && id <= r.high)
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
