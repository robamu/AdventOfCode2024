use std::{
    collections::{BinaryHeap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
pub enum Input {
    Example,
    Default,
}

const INPUT: Input = Input::Default;
const DEBUG: bool = false;

fn main() {
    let start = std::time::Instant::now();
    let filename = match INPUT {
        Input::Example => "example.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let mut patterns: HashSet<String> = HashSet::new();
    let mut largest_pattern = 0;
    let mut towels: Vec<String> = Vec::new();
    for (idx, line) in input_file.lines().enumerate() {
        let line = line.unwrap();
        if idx == 0 {
            for pattern in line.split(',').map(|v| v.trim().to_string()) {
                patterns.insert(pattern.clone());
                if pattern.len() > largest_pattern {
                    largest_pattern = pattern.len();
                }
            }
            continue;
        }
        if line.is_empty() {
            continue;
        }
        towels.push(line);
    }
    let possible_towels = towel_matching(largest_pattern, &patterns, &towels);
    match INPUT {
        Input::Example => assert_eq!(possible_towels, 6),
        Input::Default => assert_eq!(possible_towels, 287),
    }
    if DEBUG {
        println!("Patterns: {:?}", patterns);
        println!("Towels ({}): {:?}", towels.len(), towels);
    }
    println!("elapsed: {}ms", start.elapsed().as_millis());
    println!("possible towels: {}", possible_towels);
}

fn towel_matcher(largest_pattern: usize, patterns: &HashSet<String>, towel: &str) -> bool {
    if towel.is_empty() {
        panic!("empty towel");
    }
    if DEBUG {
        println!("handling towel: {}", towel);
    }
    let mut idx_to_check = BinaryHeap::new();
    let mut visited = HashSet::new();

    idx_to_check.push(0);
    while let Some(idx) = idx_to_check.pop() {
        if visited.contains(&idx) {
            continue;
        }
        for pattern_len in (1..=std::cmp::min(largest_pattern, towel.len() - idx)).rev() {
            let pattern_to_match = &towel[idx..idx + pattern_len];
            if DEBUG {
                println!("Checking for pattern {} in set", pattern_to_match);
            }
            if patterns.contains(pattern_to_match) {
                if DEBUG {
                    println!("found pattern: {}", pattern_to_match);
                }
                if idx + pattern_to_match.len() >= towel.len() {
                    return true;
                }
                idx_to_check.push(idx + pattern_to_match.len());
            }
        }
        visited.insert(idx);
    }
    false
}

fn towel_matching(
    largest_pattern: usize,
    patterns: &HashSet<String>,
    towels: &[String],
) -> usize {
    let mut possible = 0;

    for towel in towels {
        if towel_matcher(largest_pattern, patterns, towel) {
            possible += 1;
        }
    }

    possible
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_full_str() {
        let largest_pattern = 3;
        let mut patterns = HashSet::new();
        patterns.insert("abc".to_string());
        assert!(towel_matcher(largest_pattern, &patterns, "abc"));
    }

    #[test]
    fn test_single_chars() {
        let mut patterns: HashSet<String> = HashSet::new();
        patterns.insert("a".to_string());
        patterns.insert("b".to_string());
        patterns.insert("c".to_string());
        assert!(towel_matcher(1, &patterns, "abc"));
    }

    #[test]
    fn test_pattern_too_many_chars() {
        let mut patterns: HashSet<String> = HashSet::new();
        patterns.insert("abcd".to_string());
        assert!(!towel_matcher(4, &patterns, "abc"));
    }
}
