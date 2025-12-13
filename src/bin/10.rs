use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    n_lights: usize,
    goal_mask: u64,
    button_masks: Vec<u64>,
    jolts: Vec<u8>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);

    let mut total = 0u64;

    for m in &machines {
        let presses = min_presses(m).unwrap();
        total += presses;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(123456789)
}

fn min_presses(machine: &Machine) -> Option<u64> {
    let n = machine.n_lights;

    let total_states = 1usize << n; // 2^n
    let mut dist = vec![u32::MAX; total_states];
    let mut q = VecDeque::new();

    let start = 0u64;
    let goal = machine.goal_mask;

    dist[start as usize] = 0;
    q.push_back(start);

    while let Some(state) = q.pop_front() {
        let d = dist[state as usize];

        if state == goal {
            return Some(d as u64);
        }

        for &bm in &machine.button_masks {
            let next = state ^ bm;
            let idx = next as usize;

            if dist[idx] == u32::MAX {
                // check if unvisited
                dist[idx] = d + 1;
                q.push_back(next);
            }
        }
    }
    None
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| parse_machine(line))
        .collect()
}

fn parse_machine(line: &str) -> Machine {
    let mut tokens = line.split_whitespace();

    let lights = tokens.next().unwrap();
    let (n_lights, goal_mask) = parse_lights(lights);

    let mut button_masks = Vec::new();
    let mut jolts: Vec<u8> = Vec::new();

    for tok in tokens {
        if tok.starts_with('{') {
            let js: Vec<isize> = parse_joltage(tok);
            jolts = js.into_iter().map(|x| x as u8).collect();
            break;
        }
        if tok.starts_with('(') {
            button_masks.push(parse_button(tok))
        }
    }

    Machine {
        n_lights,
        goal_mask,
        button_masks,
        jolts,
    }
}

fn parse_lights(s: &str) -> (usize, u64) {
    let inner = s.trim_matches(|c| c == '[' || c == ']');
    let mut mask = 0u64;
    for (i, ch) in inner.chars().enumerate() {
        if ch == '#' {
            mask |= 1 << i;
        }
    }
    (inner.len(), mask)
}

fn parse_button(s: &str) -> u64 {
    let inner = s.trim_matches(|c| c == '(' || c == ')');
    let mut mask = 0u64;
    if !inner.is_empty() {
        for part in inner.split(',') {
            let idx: usize = part.parse().unwrap();
            mask |= 1 << idx;
        }
    }
    mask
}

fn parse_joltage(s: &str) -> Vec<isize> {
    s.trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .map(|n| n.parse().unwrap())
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
