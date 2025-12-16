use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::thread;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    n_lights: usize,
    goal_mask: u64,
    button_masks: Vec<u64>,
    jolts: Vec<u32>,
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
        let presses = min_presses_part_two(m).unwrap();
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
    let mut jolts: Vec<u32> = Vec::new();

    for tok in tokens {
        if tok.starts_with('{') {
            let js: Vec<isize> = parse_joltage(tok);
            jolts = js.into_iter().map(|x| x as u32).collect();
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

fn min_presses_part_two(machine: &Machine) -> Option<u64> {
    let n = machine.n_lights;
    let m = machine.button_masks.len();

    if m > 62 {
        return None;
    }
    let total_subsets = 1usize.checked_shl(m as u32)?;

    let (by_parity, cost, incs) = build_parity_patterns(machine, total_subsets)?;

    let target: Vec<u16> = machine.jolts.iter().map(|&x| x as u16).collect();
    let mut memo: HashMap<Vec<u16>, u32> = HashMap::new();

    const INF: u32 = 1_000_000_000;
    let ans = solve_jolts(&target, n, &by_parity, &cost, &incs, &mut memo, INF);

    if ans >= INF {
        None
    } else {
        Some(ans as u64)
    }
}

fn build_parity_patterns(
    machine: &Machine,
    total_subsets: usize,
) -> Option<(HashMap<u64, Vec<u32>>, Vec<u16>, Vec<u8>)> {
    let n = machine.n_lights;
    let m = machine.button_masks.len();

    let mut by_parity: HashMap<u64, Vec<u32>> = HashMap::new();
    let mut cost: Vec<u16> = vec![0; total_subsets];
    let mut incs: Vec<u8> = vec![0; total_subsets * n];

    let mut prev_gray: u64 = 0;
    let mut parity_mask: u64 = 0;
    let mut cur_inc: Vec<u16> = vec![0; n];

    for idx in 0..total_subsets {
        let gray = (idx as u64) ^ ((idx as u64) >> 1);

        if idx > 0 {
            let flip = gray ^ prev_gray;
            let bit = flip.trailing_zeros() as usize;

            let bm = machine.button_masks[bit];

            let adding = ((gray >> bit) & 1) == 1;

            parity_mask ^= bm;

            let mut mask = bm;
            while mask != 0 {
                let i = mask.trailing_zeros() as usize;
                mask &= mask - 1;

                if adding {
                    cur_inc[i] += 1;
                } else {
                    cur_inc[i] -= 1;
                }
            }
        }

        prev_gray = gray;

        let subset = gray as usize;

        let c = gray.count_ones() as u16;
        cost[subset] = c;

        let base = subset * n;
        for i in 0..n {
            incs[base + i] = cur_inc[i] as u8;
        }

        by_parity.entry(parity_mask).or_default().push(subset as u32);
    }

    Some((by_parity, cost, incs))
}

fn solve_jolts(
    target: &[u16],
    n: usize,
    by_parity: &HashMap<u64, Vec<u32>>,
    cost: &[u16],
    incs: &[u8],
    memo: &mut HashMap<Vec<u16>, u32>,
    inf: u32,
) -> u32 {
    if target.iter().all(|&x| x == 0) {
        return 0;
    }

    if let Some(&v) = memo.get(target) {
        return v;
    }

    let mut p: u64 = 0;
    for i in 0..n {
        if (target[i] & 1) == 1 {
            p |= 1u64 << i;
        }
    }

    let Some(subsets) = by_parity.get(&p) else {
        memo.insert(target.to_vec(), inf);
        return inf;
    };

    let mut best = inf;

    for &s_u32 in subsets {
        let s = s_u32 as usize;
        let c = cost[s] as u32;

        if c >= best {
            continue;
        }

        let base = s * n;

        let mut next: Vec<u16> = Vec::with_capacity(n);
        let mut ok = true;

        for i in 0..n {
            let inc = incs[base + i] as u16;
            if inc > target[i] {
                ok = false;
                break;
            }
            let rem = target[i] - inc;
            if (rem & 1) == 1 {
                ok = false;
                break;
            }
            next.push(rem / 2);
        }

        if !ok {
            continue;
        }

        let sub = solve_jolts(&next, n, by_parity, cost, incs, memo, inf);
        if sub >= inf {
            continue;
        }

        let cand = c.saturating_add(2u32.saturating_mul(sub));
        if cand < best {
            best = cand;
        }
    }

    memo.insert(target.to_vec(), best);
    best
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
