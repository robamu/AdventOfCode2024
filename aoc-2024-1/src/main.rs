use std::{collections::HashMap, io::BufRead};

fn main() {
    let input_file = std::fs::read("input.txt").unwrap();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input_file.lines() {
        let next_line = line.unwrap();

        for (idx, num) in next_line.split_whitespace().enumerate() {
            let number = num.parse::<i32>().unwrap();
            match idx {
                0 => left_list.push(number),
                1 => right_list.push(number),
                _ => println!("Row has more than two numbers"),
            }
        }
    }
    println!("{}", part1(left_list.clone(), right_list.clone()));
    println!("{}", part2_dumb(left_list.clone(), right_list.clone()));
    println!("{}", part2_smart(left_list, right_list));
}

fn part1(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> u32 {
    left_list.sort();
    right_list.sort();
    let mut sum = 0;
    for (left, right) in left_list.iter().zip(right_list.iter()) {
        sum += left.abs_diff(*right);
    }
    sum
}

fn part2_dumb(left_list: Vec<i32>, right_list: Vec<i32>) -> u32 {
    let mut similarity = 0;
    for left in &left_list {
        let mut multiplicator = 0;
        for right in &right_list {
            if left == right {
                multiplicator += 1;
            }
        }
        similarity += left * multiplicator;
    }
    similarity as u32
}

fn part2_smart(left_list: Vec<i32>, right_list: Vec<i32>) -> u32 {
    // Count occurrences of each number in right_list
    let mut right_counts = HashMap::new();
    for &num in &right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate similarity
    let similarity: i32 = left_list
        .iter()
        .map(|&left| left * right_counts.get(&left).unwrap_or(&0))
        .sum();

    similarity as u32
}
