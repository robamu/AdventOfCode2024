use std::io::BufRead;

fn main() {
    let start = std::time::Instant::now();
    let input_file = std::fs::read("input.txt").unwrap();
    let mut sum = 0;
    for line in input_file.lines() {
        let line = line.unwrap();
        let number = line.parse::<u64>().unwrap();
        let mut evolved = number;
        for _ in 0..2000 {
            evolved = evolve_number(evolved);
        }
        sum += evolved;
    }
    println!("elapsed: {:?}", start.elapsed());
    println!("solution p1: ");
    println!("{}", sum);
}

pub fn evolve_number(mut number: u64) -> u64 {
    let multiplied = number * 64;
    number ^= multiplied;
    number %= 16777216;
    let divided = number / 32;
    number ^= divided;
    number %= 16777216;
    let multiplied = number * 2048;
    number ^= multiplied;
    number % 16777216
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_evolve() {
        let number = 123;
        let mut evolved = evolve_number(number);
        assert_eq!(evolved, 15887950);
        evolved = evolve_number(evolved);
        assert_eq!(evolved, 16495136);
    }

    #[test]
    fn test_example() {
        let init_nums = [1, 10, 100, 2024];
        let mut evolved;
        for (idx, num) in init_nums.iter().enumerate() {
            evolved = *num;
            for _ in 0..2000 {
                evolved = evolve_number(evolved);
            }
            match idx {
                0 => assert_eq!(evolved, 8685429),
                1 => assert_eq!(evolved, 4700978),
                2 => assert_eq!(evolved, 15273692),
                3 => assert_eq!(evolved, 8667524),
                _ => unreachable!(),
            }
        }
    }
}
