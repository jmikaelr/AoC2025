advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: Vec<u64> = Vec::new();
    for bank in input.lines().filter(|l| !l.is_empty()) {
        let chars: Vec<char> = bank.chars().collect();
        let mut best = 0u64;

        for i in 0..chars.len() {
            for j in i + 1..chars.len() {
                let a = chars[i].to_digit(10).unwrap() as u64;
                let b = chars[j].to_digit(10).unwrap() as u64;
                let val = a * 10 + b;
                if val > best {
                    best = val;
                }
            }
        }
        total.push(best);
    }
    Some(total.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: Vec<u64> = Vec::new();
    let k = 12;

    for bank in input.lines().filter(|l| !l.is_empty()) {
        let chars: Vec<char> = bank.chars().collect();
        let n = chars.len();
        let mut start = 0usize;
        let mut remaining = k;
        let mut best = 0u64;

        while remaining > 0 {
            let end = n - remaining;

            let mut best_char = '0';
            let mut best_index = start;

            for (i, &c) in chars.iter().enumerate().take(end + 1).skip(start) {
                if c > best_char {
                    best_char = c;
                    best_index = i;

                    if c == '9' {
                        break;
                    }
                }
            }
            let d = best_char.to_digit(10).unwrap() as u64;
            best = best * 10 + d;

            start = best_index + 1;
            remaining -= 1;
        }

        total.push(best);
    }
    Some(total.iter().sum())
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
