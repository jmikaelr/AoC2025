use std::collections::HashMap;
advent_of_code::solution!(11);

struct Path<'a> {
    input: &'a str,
    output: Vec<&'a str>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let paths = parse_input(input);

    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for p in &paths {
        adj.entry(p.input)
            .or_default()
            .extend(p.output.iter().copied());
    }

    fn count_paths<'a>(
        node: &'a str,
        target: &'a str,
        adj: &HashMap<&'a str, Vec<&'a str>>,
        memo: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if node == target {
            return 1;
        }

        if let Some(&cached) = memo.get(node) {
            return cached;
        }

        let mut total = 0;

        if let Some(nexts) = adj.get(node) {
            for &n in nexts {
                total += count_paths(n, target, adj, memo);
            }
        }

        memo.insert(node, total);
        total
    }

    let mut memo: HashMap<&str, u64> = HashMap::new();
    let count = count_paths("you", "out", &adj, &mut memo);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let paths = parse_input(input);

    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for p in &paths {
        adj.entry(p.input)
            .or_default()
            .extend(p.output.iter().copied());
    }

    fn count_paths<'a>(
        node: &'a str,
        target: &'a str,
        adj: &HashMap<&'a str, Vec<&'a str>>,
        memo: &mut HashMap<(&'a str, &'a str), u64>,
    ) -> u64 {
        if node == target {
            return 1;
        }

        if let Some(&cached) = memo.get(&(node, target)) {
            return cached;
        }

        let mut total = 0;
        if let Some(nexts) = adj.get(node) {
            for &n in nexts {
                total += count_paths(n, target, adj, memo);
            }
        }

        memo.insert((node, target), total);
        total
    }

    let mut memo: HashMap<(&str, &str), u64> = HashMap::new();

    let dac_out = count_paths("dac", "out", &adj, &mut memo);
    let fft_dac = count_paths("fft", "dac", &adj, &mut memo);
    let svr_fft = count_paths("svr", "fft", &adj, &mut memo);
    let fft_out = count_paths("fft", "out", &adj, &mut memo);
    let dac_fft = count_paths("dac", "fft", &adj, &mut memo);
    let svr_dac = count_paths("svr", "dac", &adj, &mut memo);

    let count = (svr_dac * dac_fft * fft_out) + (svr_fft * fft_dac * dac_out);
    Some(count)
}

fn parse_input(input: &str) -> Vec<Path> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();
            let input = left.trim();
            let output: Vec<&str> = right.split_whitespace().collect();

            Path { input, output }
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
