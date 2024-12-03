use std::io::BufRead;

fn main() {
    let input_file = std::fs::read("input.txt").unwrap();
    let mut safe_lines_part1 = 0;
    let mut safe_lines_part2 = 0;

    for line in input_file.lines() {
        let next_line = line.unwrap();
        let values_as_str = next_line.split_whitespace().collect::<Vec<&str>>();
        let mut values = Vec::new();
        for value in values_as_str.iter() {
            values.push(value.parse::<i32>().unwrap());
        }
        if values.len() <= 1 {
            panic!("invalid row");
        }
        if part1(&values) {
            safe_lines_part1 += 1;
            safe_lines_part2 += 1;
        } else if part2(&values) {
            safe_lines_part2 += 1;
        }
    }
    println!("{}", safe_lines_part1);
    println!("{}", safe_lines_part2);
}

fn part1(values: &[i32]) -> bool {
    let increasing = values[1] > values[0];
    !unsafe_line_dumb(values, increasing)
}

fn unsafe_line_dumb(values: &[i32], increasing: bool) -> bool {
    let mut unsafe_line = false;
    for (idx, val) in values.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        if increasing {
            if *val <= values[idx - 1] {
                unsafe_line = true;
                break;
            }
        } else if *val >= values[idx - 1] {
            unsafe_line = true;
            break;
        }
        if values[idx - 1].abs_diff(*val) > 3 {
            unsafe_line = true;
            break;
        }
    }
    unsafe_line
}

pub fn unsafe_line_smart(values: &[i32], increasing: bool) -> bool {
    values
        .iter()
        .zip(values.iter().skip(1))
        .any(|(&prev, &curr)| {
            (increasing && curr <= prev) || (!increasing && curr >= prev) || prev.abs_diff(curr) > 3
        })
}

fn part2(values: &[i32]) -> bool {
    println!("values not safe, part 2 check for {:?}", values);
    let num_values = values.len();
    for current_skip_idx in 0..num_values {
        let mut new_list = Vec::new();
        for (idx, val) in values.iter().enumerate() {
            if current_skip_idx == idx {
                continue;
            }
            new_list.push(*val);
        }
        if new_list.len() <= 1 {
            panic!("invalid row");
        }
        println!("checking new list {:?}", new_list);
        if !unsafe_line_smart(&new_list, new_list[1] > new_list[0]) {
            return true;
        }
    }
    false
}
