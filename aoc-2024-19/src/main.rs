use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
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
    let possible_p1 = towel_matching_p1(largest_pattern, &patterns, &towels);
    let total_matches_p2 = towel_matching_p2(largest_pattern, &patterns, &towels);
    match INPUT {
        Input::Example => {
            assert_eq!(possible_p1, 6);
            assert_eq!(total_matches_p2, 16);
        }
        Input::Default => {
            assert_eq!(possible_p1, 287);
            assert_eq!(total_matches_p2, 571894474468161);
        }
    }
    if DEBUG {
        println!("Patterns: {:?}", patterns);
        println!("Towels ({}): {:?}", towels.len(), towels);
    }
    println!("elapsed: {}ms", start.elapsed().as_millis());
    println!("solution p1: {}", possible_p1);
    println!("solution p2: {}", total_matches_p2);
}

pub fn towel_matcher_with_stack(
    largest_pattern: usize,
    patterns: &HashSet<String>,
    towel: &str,
) -> bool {
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

pub fn towel_matching_using_set_p2(
    largest_pattern: usize,
    patterns: &HashSet<String>,
    towel: &str,
) -> usize {
    if towel.is_empty() || patterns.is_empty() {
        // No matches possible
        return 0;
    }

    let mut active_patterns: Vec<(usize, Vec<&str>)> = Vec::new();
    let mut visited_states: HashSet<(usize, Vec<&str>)> = HashSet::new();
    let mut matches = 0;

    // Step 1: Start by matching all patterns from the beginning of the towel
    for pattern_len in (1..=largest_pattern.min(towel.len())).rev() {
        let pattern_to_match = &towel[..pattern_len];
        if patterns.contains(pattern_to_match) {
            if pattern_len == towel.len() {
                matches += 1;
            } else {
                let state = (pattern_len, vec![pattern_to_match]);
                active_patterns.push(state.clone());
                visited_states.insert(state); // Memoize initial state
            }
        }
    }

    // Step 2: Expand active patterns iteratively
    while let Some((current_idx, current_pattern)) = active_patterns.pop() {
        // Try to match patterns starting from the current index
        for pattern_len in (1..=largest_pattern.min(towel.len() - current_idx)).rev() {
            let pattern_to_match = &towel[current_idx..current_idx + pattern_len];
            if patterns.contains(pattern_to_match) {
                let next_idx = current_idx + pattern_len;

                if next_idx == towel.len() {
                    matches += 1; // Full match
                } else {
                    let mut next_pattern = current_pattern.clone();
                    next_pattern.push(pattern_to_match);
                    let state = (next_idx, next_pattern);

                    // Check if this state has been visited
                    if !visited_states.contains(&state) {
                        active_patterns.push(state.clone());
                        visited_states.insert(state); // Memoize state
                    }
                }
            }
        }
    }

    matches
}

pub fn towel_matcher_counting(
    largest_pattern: usize,
    patterns: &HashSet<String>,
    towel: &str,
) -> usize {
    if towel.is_empty() {
        panic!("empty towel");
    }
    if DEBUG {
        println!("handling towel: {}", towel);
    }
    let mut idx_with_pattern = VecDeque::new();
    // Memoization map.
    // takes too much memory for part2!
    let mut memo: HashSet<(usize, Vec<&str>)> = HashSet::new();
    let mut matches = 0;

    idx_with_pattern.push_back((0, Vec::new()));
    while let Some((idx, pattern_so_far)) = idx_with_pattern.pop_front() {
        println!("idx: {}, pattern_so_far: {:?}", idx, pattern_so_far);
        // Memoize state: skip if already visited.
        if !memo.insert((idx, pattern_so_far.clone())) {
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
                let mut next_pattern_so_far = pattern_so_far.clone();
                next_pattern_so_far.push(patterns.get(pattern_to_match).unwrap());
                if idx + pattern_len >= towel.len() {
                    matches += 1;
                } else {
                    idx_with_pattern.push_back((idx + pattern_len, next_pattern_so_far));
                }
            }
        }
    }
    matches
}

/// Use dynamic programming: Use sub-solutions to iteratively calculate the full solution.
pub fn towel_matcher_exhaustive_dynamic_programming(
    largest_pattern: usize,
    patterns: &HashSet<String>,
    towel: &str,
) -> usize {
    let mut partial_solutions = vec![0; towel.len() + 1];
    // There is one way to match an empty towel: Do nothing.
    partial_solutions[0] = 1;
    for i in 1..towel.len() + 1 {
        // We now match the patterns against the towel string left of the current index.
        // If we find a fitting match that completes until the index, we can increment the solution
        // based on the sub-solution at the start index of the pattern.
        for pattern_len in (1..=largest_pattern.min(i)).rev() {
            let pattern_to_match = &towel[i - pattern_len..i];
            if patterns.contains(pattern_to_match) {
                partial_solutions[i] += partial_solutions[i - pattern_len];
            }
        }
    }
    partial_solutions[towel.len()]
}

pub fn towel_matcher_exhaustive_dfs_recursive(
    largest_pattern: usize,
    patterns: &HashSet<String>,
    towel: &str,
) -> usize {
    let mut current_path: Vec<&str> = Vec::new();
    let mut matches = 0;
    // Track the current path, and backtrack to cover all possible combinations.

    // Step 1: Start by matching all patterns from the beginning of the towel
    for pattern_len in (1..=largest_pattern.min(towel.len())).rev() {
        let pattern_to_match = &towel[..pattern_len];
        if patterns.contains(pattern_to_match) {
            if pattern_len == towel.len() {
                matches += 1;
            } else {
                current_path.push(pattern_to_match);
            }
        }
    }
    towel_matcher_exhaustive_dfs_recursion(
        largest_pattern,
        patterns,
        towel,
        current_path.clone(),
        &mut matches,
    );
    matches
}

pub fn towel_matcher_exhaustive_dfs_recursion(
    largest_pattern: usize,
    patterns: &HashSet<String>,
    towel: &str,
    path: Vec<&str>,
    matches: &mut usize,
) {
    let base_idx = path.iter().map(|v| v.len()).sum();
    for pattern_len in (1..=largest_pattern.min(towel.len() - base_idx)).rev() {
        let pattern_to_match = &towel[base_idx..base_idx + pattern_len];
        if patterns.contains(pattern_to_match) {
            if pattern_len == towel.len() {
                *matches += 1;
            } else {
                let mut next_path = path.clone();
                next_path.push(pattern_to_match);
                towel_matcher_exhaustive_dfs_recursion(
                    largest_pattern,
                    patterns,
                    towel,
                    next_path,
                    matches,
                );
            }
        }
    }
}

fn towel_matching_p1(
    largest_pattern: usize,
    patterns: &HashSet<String>,
    towels: &[String],
) -> usize {
    let mut possible_designs_p1 = 0;

    for towel in towels {
        if DEBUG {
            println!("handling towel {:?}", towel);
        }
        if towel_matcher_with_stack(largest_pattern, patterns, towel) {
            possible_designs_p1 += 1;
        }
    }

    possible_designs_p1
}

fn towel_matching_p2(
    largest_pattern: usize,
    patterns: &HashSet<String>,
    towels: &[String],
) -> usize {
    let mut total_num_designs = 0;

    for towel in towels {
        if DEBUG {
            println!("handling towel {:?}", towel);
        }
        total_num_designs +=
            towel_matcher_exhaustive_dynamic_programming(largest_pattern, patterns, towel);
    }

    total_num_designs
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_full_str() {
        let largest_pattern = 3;
        let mut patterns = HashSet::new();
        patterns.insert("abc".to_string());
        assert!(towel_matcher_with_stack(largest_pattern, &patterns, "abc"));
        assert_eq!(towel_matcher_counting(largest_pattern, &patterns, "abc"), 1);
    }

    #[test]
    fn test_with_dp() {
        let largest_pattern = 3;
        let mut patterns = HashSet::new();
        patterns.insert("abc".to_string());
        assert_eq!(
            towel_matcher_exhaustive_dynamic_programming(largest_pattern, &patterns, "abc"),
            1
        );
    }

    #[test]
    fn test_full_str_using_set_algorithm() {
        let largest_pattern = 3;
        let mut patterns = HashSet::new();
        patterns.insert("abc".to_string());
        assert_eq!(
            towel_matching_using_set_p2(largest_pattern, &patterns, "abc"),
            1
        );
    }

    #[test]
    fn test_single_chars() {
        let mut patterns: HashSet<String> = HashSet::new();
        patterns.insert("a".to_string());
        patterns.insert("b".to_string());
        patterns.insert("c".to_string());
        assert!(towel_matcher_with_stack(1, &patterns, "abc"));
        assert_eq!(towel_matcher_counting(1, &patterns, "abc"), 1);
    }

    #[test]
    fn test_pattern_too_many_chars() {
        let mut patterns: HashSet<String> = HashSet::new();
        patterns.insert("abcd".to_string());
        assert!(!towel_matcher_with_stack(4, &patterns, "abc"));
        assert_eq!(towel_matcher_counting(4, &patterns, "abc"), 0);
    }

    #[test]
    fn test_multi_comb() {
        let mut patterns: HashSet<String> = HashSet::new();
        patterns.insert("a".to_string());
        patterns.insert("b".to_string());
        patterns.insert("c".to_string());
        patterns.insert("ab".to_string());
        assert!(towel_matcher_with_stack(2, &patterns, "abc"));
        assert_eq!(towel_matcher_counting(2, &patterns, "abc"), 2);
    }

    #[test]
    fn test_multi_comb_with_set_algo() {
        let mut patterns: HashSet<String> = HashSet::new();
        patterns.insert("a".to_string());
        patterns.insert("b".to_string());
        patterns.insert("c".to_string());
        patterns.insert("ab".to_string());
        assert_eq!(towel_matching_using_set_p2(2, &patterns, "abc"), 2);
    }

    #[test]
    fn test_with_dp_multi_algo() {
        let mut patterns: HashSet<String> = HashSet::new();
        patterns.insert("a".to_string());
        patterns.insert("b".to_string());
        patterns.insert("c".to_string());
        patterns.insert("ab".to_string());
        assert_eq!(
            towel_matcher_exhaustive_dynamic_programming(2, &patterns, "abc"),
            2
        );
    }
}
