use rayon::prelude::*;
use std::collections::HashMap;
use std::default::Default;
pub mod list_based;
use std::sync::{
    atomic::{self, AtomicUsize},
    Arc, Mutex,
};

pub use aoc_2024_11::*;
pub use list_based::*;

const DEBUG: bool = true;

#[derive(Debug)]
pub enum Input {
    Simple,
    Default,
}

const INPUT: Input = Input::Default;

const CALC_POINT_P1: usize = 25;
const CALC_POINT_P2: usize = 75;

#[derive(Debug)]
pub struct StoneRecursor {
    init_stones: Vec<u64>,
    pub total_num_of_stones: Arc<AtomicUsize>,
    cache: Arc<Mutex<HashMap<(u64, usize), usize>>>,
}

impl StoneRecursor {
    pub fn new(data: &[u8]) -> Self {
        let init_stones = get_initial_stones(data);
        let total_num_of_stones = init_stones.len();

        Self {
            init_stones,
            total_num_of_stones: Arc::new(AtomicUsize::new(total_num_of_stones)),
            cache: Default::default(),
        }
    }
    pub fn blink_n_times(&self, times: usize) -> usize {
        for val in self.init_stones.clone() {
            println!("blinking initial stone {} {} times", val, times);
            let mut num_of_stones = 0;
            self.blink_recursion(val, 0, times, &mut num_of_stones);
            self.total_num_of_stones
                .fetch_add(num_of_stones, atomic::Ordering::Relaxed);
            println!(
                "additional stones after processing {}: {}",
                val, num_of_stones
            );
        }
        let stones = self.total_num_of_stones.load(atomic::Ordering::Relaxed);
        self.reset();
        stones
    }

    pub fn blink_n_times_parallelized(&self, times: usize) -> usize {
        // Process each initial stone in parallel
        self.init_stones.par_iter().for_each(|&val| {
            println!("blinking initial stone {} {} times", val, times);
            let mut num_of_stones = 0;
            self.blink_recursion(val, 0, times, &mut num_of_stones);
            // Aggregate the total number of stones
            self.total_num_of_stones
                .fetch_add(num_of_stones, atomic::Ordering::Relaxed);
            println!(
                "additional stones after processing {}: {}",
                val, num_of_stones
            );
        });

        let stones = self.total_num_of_stones.load(atomic::Ordering::Relaxed);
        self.reset();
        stones
    }

    pub fn reset(&self) {
        self.total_num_of_stones
            .store(self.init_stones.len(), atomic::Ordering::Relaxed);
        self.cache.lock().unwrap().clear();
    }

    pub fn blink_recursion(
        &self,
        val: u64,
        blink_depth: usize,
        times: usize,
        num_of_stones: &mut usize,
    ) {
        // Check if result is already cached
        {
            let cache = self.cache.lock().unwrap();
            if let Some(&cached_result) = cache.get(&(val, blink_depth)) {
                *num_of_stones += cached_result;
                return;
            }
        }

        // Perform computation if not cached
        let mut stones_for_this_call = 0;

        if blink_depth < times {
            match apply_blink_algo(val) {
                BlinkResult::Replaced(new_val) => {
                    self.blink_recursion(
                        new_val,
                        blink_depth + 1,
                        times,
                        &mut stones_for_this_call,
                    );
                }
                BlinkResult::Split(first, second) => {
                    self.blink_recursion(first, blink_depth + 1, times, &mut stones_for_this_call);
                    self.blink_recursion(second, blink_depth + 1, times, &mut stones_for_this_call);
                    stones_for_this_call += 1;
                }
            }
        }

        // Update the total stones count
        *num_of_stones += stones_for_this_call;

        // Cache the result
        let mut cache = self.cache.lock().unwrap();
        cache.insert((val, blink_depth), stones_for_this_call);
    }

    pub fn blink_with_stack(val: u64, times: usize) -> usize {
        let mut stack = vec![(val, 0)];
        let mut num_of_stones = 0;

        while let Some((current_val, blink_depth)) = stack.pop() {
            if blink_depth == times {
                continue;
            }
            match apply_blink_algo(current_val) {
                BlinkResult::Replaced(new_val) => {
                    stack.push((new_val, blink_depth + 1));
                }
                BlinkResult::Split(first, second) => {
                    stack.push((first, blink_depth + 1));
                    stack.push((second, blink_depth + 1));
                    num_of_stones += 1;
                }
            }
        }
        num_of_stones
    }
}

fn main() {
    let start = std::time::Instant::now();
    let filename = match INPUT {
        Input::Simple => "example.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let mut stones_recursor = StoneRecursor::new(&input_file);
    let mut num_of_stones = stones_recursor.blink_n_times_parallelized(CALC_POINT_P1);
    match INPUT {
        Input::Simple => assert_eq!(num_of_stones, 55312),
        Input::Default => assert_eq!(num_of_stones, 194557),
    }
    println!("total num of stones p1 {}", num_of_stones);
    stones_recursor = StoneRecursor::new(&input_file);
    num_of_stones = stones_recursor.blink_n_times_parallelized(CALC_POINT_P2);
    println!("total num of stones p2 {}", num_of_stones);
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}

// Works, list sizes are sufficiently small.
pub fn part1_list_based(data: &[u8]) {
    let mut stones = StonesInRamListBased::new(data);
    for _ in 0..25 {
        stones.blink();
        if DEBUG {
            println!("Stones: {:?}", stones);
        }
    }
    println!(
        "Number of stones after 25 blinks: {}",
        stones.num_of_stones()
    );
    match INPUT {
        Input::Simple => assert_eq!(stones.num_of_stones(), 55312),
        Input::Default => assert_eq!(stones.num_of_stones(), 194557),
    }
}

// Does not work, RAM or disk usage is too high.
pub fn part2_too_long(stones: StonesInRamListBased) {
    let mut stones = StonesWrapper::StonesInRam(stones);
    for current_idx in 25..75 {
        println!("Blink iteration: {}", current_idx);
        stones.blink();
        match stones {
            StonesWrapper::StonesInRam(ref stones_in_ram) => {
                if stones_in_ram.memory_usage() >= 1_000_000 {
                    println!("Switching to filesystem usage");
                    // Switch to file system storage.
                    stones = stones.convert_to_fs();
                }
            }
            StonesWrapper::StonesInFs(_) => (),
        }
    }
    println!("Number of stones: {}", stones.num_of_stones());
}
