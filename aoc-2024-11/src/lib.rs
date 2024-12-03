use std::io::BufRead;

#[derive(Debug)]
pub enum BlinkResult {
    Replaced(u64),
    Split(u64, u64),
}

pub fn apply_blink_algo(val: u64) -> BlinkResult {
    match val {
        0 => BlinkResult::Replaced(1),
        _ => {
            let digits = (val as f64).log10().floor() as u32 + 1; // Number of digits
            if digits % 2 == 0 {
                let divisor = 10u64.pow(digits / 2); // Power of 10 to split the number
                BlinkResult::Split(val / divisor, val % divisor)
            } else {
                BlinkResult::Replaced(val * 2024)
            }
        }
    }
}

pub fn get_initial_stones(data: &[u8]) -> Vec<u64> {
    let mut stones = Vec::new();
    for line in data.lines() {
        let line = line.unwrap();
        for num in line.split_whitespace() {
            stones.push(num.parse().unwrap());
        }
    }
    stones
}
