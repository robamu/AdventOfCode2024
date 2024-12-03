use core::str;

use regex::Regex;

fn main() {
    let input_file = std::fs::read("input.txt").unwrap();
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut enabled = true;
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for captures in re.captures_iter(str::from_utf8(&input_file).unwrap()) {
        match captures.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            cmd if cmd.starts_with("mul") => {
                // Extract the numbers from the capture groups
                let a = captures
                    .get(1)
                    .and_then(|m| m.as_str().parse::<u32>().ok())
                    .expect("Failed to parse first operand");
                let b = captures
                    .get(2)
                    .and_then(|m| m.as_str().parse::<u32>().ok())
                    .expect("Failed to parse second operand");
                let increment = a * b;
                sum_part1 += increment;
                if enabled {
                    sum_part2 += increment;
                }
            }
            _ => continue,
        }
    }
    println!("{}", sum_part1);
    println!("{}", sum_part2);
}
