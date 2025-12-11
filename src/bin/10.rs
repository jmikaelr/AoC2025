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
    let machines = parse_input(input);

    let mut total = 0u64;

    for m in &machines {
        let presses = min_presses_jolts(m)?;
        total += presses;
    }
    Some(total)
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

fn min_presses_jolts(machine: &Machine) -> Option<u64> {
    let target: Vec<u16> = machine.jolts.iter().map(|&x| x as u16).collect();
    let k = target.len();
    let b = machine.button_masks.len();

    let button_indices: Vec<Vec<usize>> = machine
        .button_masks
        .iter()
        .map(|&bm| {
            let mut idxs = Vec::new();
            for i in 0..k {
                if (bm & (1 << i)) != 0 {
                    idxs.push(i);
                }
            }
            idxs
        })
        .collect();

    let mut best: Option<u64> = None;
    let mut current = vec![0u16; k];

    fn dfs_button(
        j: usize,
        b: usize,
        button_indices: &Vec<Vec<usize>>,
        target: &Vec<u16>,
        current: &mut [u16],
        presses_so_far: u64,
        best: &mut Option<u64>,
    ) {
        if let Some(bst) = *best {
            if presses_so_far >= bst {
                return;
            }
        }

        if j == b {
            for i in 0..target.len() {
                if current[i] != target[i] {
                    return;
                }
            }
            *best = Some(presses_so_far);
            return;
        }

        let idxs = &button_indices[j];

        if idxs.is_empty() {
            dfs_button(j + 1, b, button_indices, target, current, presses_so_far, best);
            return;
        }

        let mut max_x: u16 = u16::MAX;
        for &i in idxs {
            if current[i] > target[i] {
                return; // already impossible
            }
            let cap = target[i].saturating_sub(current[i]); // how much more we can add on this counter
            if cap < max_x {
                max_x = cap;
            }
        }

        // try x presses for button j, from 0 up to max_x
        for x in 0..=max_x {
            // apply x
            if x > 0 {
                for &i in idxs {
                    current[i] += x;
                }
            }

            dfs_button(
                j + 1,
                b,
                button_indices,
                target,
                current,
                presses_so_far + x as u64,
                best,
            );

            // undo x
            if x > 0 {
                for &i in idxs {
                    current[i] -= x;
                }
            }
        }
    }

    dfs_button(0, b, &button_indices, &target, &mut current, 0, &mut best);
    best
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
