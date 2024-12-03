use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use aoc_2024_11::{apply_blink_algo, get_initial_stones, BlinkResult};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSlice,
};

const PARALLEL_CHUNKS: usize = 1024;

pub trait Stones {
    fn blink(&mut self);
    fn num_of_stones(&self) -> usize;
}

#[derive(Debug)]
pub struct StonesInRamListBased(pub Vec<u64>);

impl StonesInRamListBased {
    pub fn new(data: &[u8]) -> Self {
        Self(get_initial_stones(data))
    }

    pub fn memory_usage(&self) -> usize {
        self.0.len() * std::mem::size_of::<u64>()
    }

    pub fn num_of_stones(&self) -> usize {
        self.0.len()
    }

    pub fn blink_algo(chunk: &[u64], new_list: &mut Vec<u64>) {
        for &val in chunk {
            match apply_blink_algo(val) {
                BlinkResult::Replaced(val) => new_list.push(val),
                BlinkResult::Split(first, second) => {
                    new_list.push(first);
                    new_list.push(second);
                }
            }
        }
    }

    pub fn blink(&mut self) {
        if self.0.len() < PARALLEL_CHUNKS {
            let mut new_list = Vec::with_capacity(self.0.len() * 2);
            Self::blink_algo(&self.0, &mut new_list);
            self.0 = new_list;
        } else {
            let num_chunks = self.0.len().div_ceil(PARALLEL_CHUNKS);
            let shared_list = Arc::new(Mutex::new(vec![Vec::<u64>::new(); num_chunks]));
            self.0
                .par_chunks(PARALLEL_CHUNKS)
                .enumerate()
                .for_each(|(idx, chunk)| {
                    let mut chunk_list = Vec::with_capacity(chunk.len() * 2);

                    Self::blink_algo(chunk, &mut chunk_list);
                    shared_list.lock().unwrap()[idx] = chunk_list;
                });
            let mut list = shared_list.lock().unwrap();
            let mut new_vec = Vec::with_capacity(self.0.len() * 2);
            for chunk in list.drain(..) {
                new_vec.extend(chunk);
            }
            self.0 = new_vec;
        }
    }
}

#[derive(Debug)]
pub struct StonesInFs {
    pub files: Vec<Option<PathBuf>>,
    pub num_of_stones: usize,
}

impl From<StonesInRamListBased> for StonesInFs {
    fn from(stones_in_ram: StonesInRamListBased) -> Self {
        if std::fs::exists("stones").unwrap() {
            std::fs::remove_dir_all("stones").unwrap();
        }
        std::fs::create_dir("stones").unwrap();
        let target_path = "stones/0.txt";
        let mut file = std::fs::File::create(target_path).unwrap();
        let num_of_stones = stones_in_ram.num_of_stones();
        for val in stones_in_ram.0 {
            writeln!(file, "{}", val).unwrap();
        }
        Self {
            files: vec![Some(PathBuf::from(target_path))],
            num_of_stones,
        }
    }
}

impl StonesInFs {
    pub fn blink(&mut self) {
        // Paths for temporary and target files
        let temp_path = "stones/0.txt.tmp";
        let target_path = "stones/0.txt";
        let mut new_file = std::fs::File::create(temp_path).unwrap();
        let old_file = std::fs::File::open(self.files[0].as_ref().unwrap()).unwrap();
        let buf_reader = BufReader::new(old_file);
        let mut buf_writer = BufWriter::new(&mut new_file);
        let mut string = String::new();
        let mut num_of_stones = 0;
        for line in buf_reader.lines() {
            let line = line.unwrap();
            let num: u64 = line.parse().unwrap();
            match apply_blink_algo(num) {
                BlinkResult::Replaced(val) => {
                    string.push_str(&format!("{}\n", val));
                    num_of_stones += 1;
                }
                BlinkResult::Split(first, second) => {
                    string.push_str(&format!("{}\n{}\n", first, second));
                    num_of_stones += 2;
                }
            }
            if string.len() > 1_000_000 {
                buf_writer.write_all(string.as_bytes()).unwrap();
                string.clear();
            }
        }
        buf_writer.write_all(string.as_bytes()).unwrap();
        // This should automatically close the old file handle.
        std::fs::rename(temp_path, target_path).unwrap();
        self.num_of_stones = num_of_stones;
    }

    pub fn num_of_stones(&self) -> usize {
        self.num_of_stones
    }
}

pub enum StonesWrapper {
    StonesInRam(StonesInRamListBased),
    StonesInFs(StonesInFs),
}

impl Stones for StonesWrapper {
    fn blink(&mut self) {
        match self {
            StonesWrapper::StonesInRam(stones_in_ram) => stones_in_ram.blink(),
            StonesWrapper::StonesInFs(stones_in_fs) => stones_in_fs.blink(),
        }
    }

    fn num_of_stones(&self) -> usize {
        match self {
            StonesWrapper::StonesInRam(stones_in_ram) => stones_in_ram.num_of_stones(),
            StonesWrapper::StonesInFs(stones_in_fs) => stones_in_fs.num_of_stones(),
        }
    }
}

impl StonesWrapper {
    pub fn convert_to_fs(mut self) -> Self {
        match self {
            StonesWrapper::StonesInRam(stones_in_ram) => {
                self = StonesWrapper::StonesInFs(stones_in_ram.into());
            }
            StonesWrapper::StonesInFs(_) => (),
        }
        self
    }
}
