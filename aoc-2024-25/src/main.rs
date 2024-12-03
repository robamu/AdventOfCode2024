use std::{collections::HashSet, io::BufRead};

pub type KeyLockType = [usize; 5];
pub type KeyLockList = Vec<KeyLockType>;

#[derive(Debug, Default, Eq, PartialEq, Hash)]
pub struct Key([usize; 5]);
#[derive(Debug, Default, Eq, PartialEq, Hash)]
pub struct Lock([usize; 5]);

#[derive(Debug, Default, Eq, PartialEq, Hash)]
pub struct Combination {
    key: Key,
    lock: Lock,
}

fn parse(input: &[u8]) -> (KeyLockList, KeyLockList) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut next_fill_level_from_top = [0_usize; 5];
    let mut next_is_key = true;
    let mut row_idx = 0;
    let num_of_lines = input.lines().count();
    for (idx, line) in input.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() || idx == num_of_lines - 1 {
            if !next_is_key {
                locks.push(next_fill_level_from_top);
            } else {
                next_fill_level_from_top = next_fill_level_from_top.map(|v| 5 - v);
                keys.push(next_fill_level_from_top);
            }
            row_idx = 0;
            next_fill_level_from_top = [0; 5];
            continue;
        }
        if row_idx == 0 {
            next_is_key = !line.contains("#");
            row_idx += 1;
            continue;
        }
        for (col_idx, char) in line.chars().enumerate() {
            if next_is_key && char == '.' {
                next_fill_level_from_top[col_idx] += 1;
            }
            if !next_is_key && char == '#' {
                next_fill_level_from_top[col_idx] += 1;
            }
        }
        row_idx += 1;
    }
    (keys, locks)
}

fn count_fitting_combinations(keys: KeyLockList, locks: KeyLockList) -> usize {
    let mut fitting_combination = 0;
    let mut combinations = HashSet::new();
    for key in &keys {
        for lock in &locks {
            combinations.insert((key, lock));
            let mut skip = false;
            for (&key_val, &lock_val) in key.iter().zip(lock.iter()) {
                if key_val + lock_val > 5 {
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            }
            fitting_combination += 1;
        }
    }
    fitting_combination
}

fn main() {
    let start = std::time::Instant::now();
    let input_file = std::fs::read("input.txt").unwrap();

    let (keys, locks) = parse(&input_file);
    let fitting_combinations = count_fitting_combinations(keys, locks);
    println!("Elapsed: {}ms", start.elapsed().as_millis());
    println!("Fitting combinations: {}", fitting_combinations);
    assert_eq!(fitting_combinations, 3397);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input_file = std::fs::read("example.txt").unwrap();
        let (keys, locks) = parse(&input_file);
        let fitting_combination = count_fitting_combinations(keys, locks);
        assert_eq!(fitting_combination, 3);
    }
}
