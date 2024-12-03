use std::io::BufRead;

#[derive(Debug)]
pub enum Input {
    Simple,
    Default,
}

const INPUT: Input = Input::Default;
const DEBUG: bool = false;

fn main() {
    let filename = match INPUT {
        Input::Simple => "example.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let mut calib_result_p1 = 0;
    let mut calib_result_p2 = 0;
    for line in input_file.lines() {
        let next_line = line.unwrap();
        let mut split = next_line.split(':');
        let result = split.next().unwrap().parse::<u64>().unwrap();
        let numbers: Vec<u64> = split
            .next()
            .unwrap()
            .split(" ")
            .skip(1)
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        // Part 1
        // Start with the first number as the initial result
        let mut prev_results = vec![numbers[0]];
        for number in &numbers[1..] {
            // Pre-allocate space
            let mut next_results = Vec::with_capacity(prev_results.len() * 2);
            for &prev_result in &prev_results {
                next_results.push(prev_result + number);
                next_results.push(prev_result * number);
            }
            // Update previous results
            prev_results = next_results;
        }
        prev_results.sort_unstable();
        if prev_results.binary_search(&result).is_ok() {
            if DEBUG {
                println!(
                    "Result {} is in possible results {:?}",
                    result, prev_results
                );
            }
            calib_result_p1 += result;
        }

        // Part 2
        // Start with the first number as the initial result
        let mut prev_results = vec![numbers[0]];
        for number in &numbers[1..] {
            // Pre-allocate space
            let mut next_results = Vec::with_capacity(prev_results.len() * 3);
            for &prev_result in &prev_results {
                next_results.push(prev_result + number);
                next_results.push(prev_result * number);
                next_results.push(format!("{}{}", prev_result, number).parse().unwrap());
            }
            // Update previous results
            prev_results = next_results;
        }
        prev_results.sort_unstable();
        if prev_results.binary_search(&result).is_ok() {
            calib_result_p2 += result;
        }
    }
    match INPUT {
        Input::Simple => assert_eq!(calib_result_p1, 3749),
        Input::Default => assert_eq!(calib_result_p1, 28730327770375),
    }
    println!("Total calibration result p1: {}", calib_result_p1);
    println!("Total calibration result p2: {}", calib_result_p2);
}
