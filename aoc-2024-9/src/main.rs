const DEBUG: bool = false;

#[derive(Debug)]
pub enum Input {
    Simplest,
    Simple,
    Default,
}

const INPUT: Input = Input::Default;

pub struct Compactor {
    pub files_list: Vec<u8>,
    pub free_spaces_list: Vec<u8>,
    pub total_num_files: usize,
    pub total_num_free_spaces: usize,
}

impl Compactor {
    pub fn new(disk_map: &str) -> Self {
        let mut files_list = Vec::new();
        let mut free_spaces_list = Vec::new();
        let mut total_num_files = 0;
        let mut total_num_free_spaces = 0;
        for (idx, num) in disk_map.chars().enumerate() {
            let next_val = num.to_string().parse::<u8>().unwrap();
            if idx % 2 == 0 {
                files_list.push(next_val);
                total_num_files += next_val as usize;
            } else {
                free_spaces_list.push(next_val);
                total_num_free_spaces += next_val as usize;
            }
        }
        assert!(!files_list.is_empty());
        Self {
            files_list,
            free_spaces_list,
            total_num_files,
            total_num_free_spaces,
        }
    }

    pub fn create_uncompacted(&self) -> Vec<Option<usize>> {
        let mut uncompacted: Vec<Option<usize>> =
            Vec::with_capacity(self.total_num_free_spaces + self.total_num_files);
        let mut file_idx = 0;
        let mut free_spaces_idx = 0;
        loop {
            if DEBUG {
                println!("compacted len: {}", uncompacted.len());
            }
            if uncompacted.len() == uncompacted.capacity() {
                break;
            }
            if file_idx < self.files_list.len() {
                for _ in 0..self.files_list[file_idx] {
                    uncompacted.push(Some(file_idx));
                }
                file_idx += 1;
            }
            if free_spaces_idx < self.free_spaces_list.len() {
                for _ in 0..self.free_spaces_list[free_spaces_idx] {
                    uncompacted.push(None);
                }
                free_spaces_idx += 1;
            }
        }
        uncompacted
    }

    pub fn run_compacting_simple_p1(&self) -> (usize, Vec<usize>) {
        let mut compacted = self.create_uncompacted();
        if DEBUG {
            println!("uncompacted: {:?}", compacted);
        }
        // Now loop from right to left and left to right at the same time until the indexes are
        // equal and move numbers on the right to the free spaces.
        let mut left_idx = 0;
        let mut right_idx = compacted.len() - 1;
        loop {
            while compacted[left_idx].is_some() {
                left_idx += 1;
            }
            while compacted[right_idx].is_none() {
                right_idx -= 1;
            }
            if left_idx >= right_idx {
                break;
            }
            compacted.swap(left_idx, right_idx);
        }
        if DEBUG {
            println!("compacted: {:?}", compacted);
        }
        let mut checksum = 0;
        let mut compacted_ids = Vec::new();
        for (idx, val) in compacted.iter().enumerate() {
            if val.is_none() {
                break;
            }
            checksum += val.unwrap() * idx;
            compacted_ids.push(val.unwrap());
        }
        (checksum, compacted_ids)
    }

    pub fn run_compacting_p2(&self) -> (usize, Vec<Option<usize>>) {
        let mut compacted = self.create_uncompacted();
        let mut right_idx = compacted.len() - 1;
        let mut left_idx;
        let mut files_chunk_len;
        let mut free_spaces_len;
        let mut dest_start;
        while right_idx > 0 {
            if compacted[right_idx].is_none() {
                right_idx -= 1;
                continue;
            }
            let block_id = compacted[right_idx].unwrap();
            // Calculate the block size
            files_chunk_len = 1;
            while right_idx > 0
                && compacted[right_idx - 1].is_some()
                && compacted[right_idx - 1].unwrap() == block_id
            {
                files_chunk_len += 1;
                right_idx -= 1;
            }
            if right_idx == 0 {
                break;
            }
            if DEBUG {
                println!(
                    "Detected files block with ID {:?} at position {} with length {}",
                    compacted[right_idx], right_idx, files_chunk_len
                );
            }
            let mut moved_block = false;
            left_idx = 0;
            // Now we check all free spaces to see if one can accomodate the block and move it
            // where appropriate.
            while left_idx < right_idx {
                while compacted[left_idx].is_some() && left_idx < right_idx {
                    left_idx += 1;
                }
                dest_start = left_idx;
                free_spaces_len = 0;
                while compacted[left_idx].is_none() {
                    left_idx += 1;
                    free_spaces_len += 1;
                    if free_spaces_len == files_chunk_len {
                        // copy block to free spaces
                        compacted.copy_within(right_idx..right_idx + files_chunk_len, dest_start);
                        if DEBUG {
                            println!(
                                "Moving block {:?} to destination index {}",
                                &compacted[right_idx..right_idx + files_chunk_len],
                                dest_start
                            );
                        }
                        compacted[right_idx..right_idx + files_chunk_len].fill(None);
                        // Search for next slock to move.
                        moved_block = true;
                        break;
                    }
                }
                // Search for next block to move.
                if moved_block {
                    break;
                }
                // In any other case, the free spaces block was not large enough, so we search for
                // the next one by simply re-running the loop.
                left_idx += 1;
            }
            // prepare next loop iteration
            right_idx -= 1;
        }
        if DEBUG {
            println!("compacted part 2: {:?}", compacted);
        }
        let mut checksum = 0;
        for (idx, opt_val) in compacted.iter().enumerate() {
            if let Some(val) = opt_val {
                checksum += idx * val;
            }
        }
        (checksum, compacted)
    }

    pub fn run_compacting_fancy(&self) -> (usize, Vec<usize>) {
        let mut compacted = Vec::with_capacity(self.total_num_files);
        let mut ids = Vec::new();
        let mut file_idx = 0;
        let mut spaces_idx = 0;
        let mut last_file_idx = self.files_list.len() - 1;
        let mut num_to_push = Self::next_num_to_push(&self.files_list, &mut last_file_idx);
        let mut free_spaces = Self::next_free_spaces(&self.free_spaces_list, &mut spaces_idx);
        let mut num_to_push_idx = 0;
        let mut free_spaces_idx = 0;
        let mut loop_done = false;

        if DEBUG {
            println!("file list: {:?}", self.files_list);
            println!("free spaces: {:?}", self.free_spaces_list);
        }
        // how to handle the case where the last file index becomes smaller than the
        // regular file index? It basically means we can't move anymore, because there is nothing
        // left to move.
        while !loop_done {
            // Handle regular files first. Those are simply pushed onto the compacted
            // list.
            for _ in 0..self.files_list[file_idx] {
                if DEBUG {
                    println!(
                        "pushing regular compacted {} with ID {}",
                        self.files_list[file_idx], file_idx
                    );
                }
                compacted.push(self.files_list[file_idx]);
                ids.push(file_idx);
                if compacted.len() == self.total_num_files {
                    loop_done = true;
                    break;
                }
            }
            if loop_done {
                break;
            }
            file_idx += 1;
            if DEBUG {
                println!("next available free spaces: {}", free_spaces);
            }
            loop {
                compacted.push(num_to_push);
                ids.push(last_file_idx);
                if DEBUG {
                    println!(
                        "moved number to compacted: {} with ID {}",
                        num_to_push, last_file_idx
                    );
                }
                if compacted.len() == self.total_num_files {
                    println!("done moving");
                    loop_done = true;
                    break;
                }
                num_to_push_idx += 1;
                free_spaces_idx += 1;
                let all_files_pushed = num_to_push_idx == num_to_push;
                let all_free_spaces_expended = free_spaces_idx == free_spaces;
                // We have pushed all available files at the end to compact and jump to the next file,
                // which is the file before the current one.
                if all_files_pushed {
                    if DEBUG {
                        println!("finished pushing num with ID {}", last_file_idx);
                    }
                    last_file_idx -= 1;
                    num_to_push = Self::next_num_to_push(&self.files_list, &mut last_file_idx);
                    num_to_push_idx = 0;
                    if DEBUG {
                        println!(
                            "pushing num with ID {} {} times",
                            last_file_idx, num_to_push
                        );
                    }
                }
                // We have expended all of the next free spaces for compacting file blocks
                // and go back to pushing files.
                if all_free_spaces_expended {
                    if DEBUG {
                        println!("free spaces expended");
                    }
                    spaces_idx += 1;
                    free_spaces = Self::next_free_spaces(&self.free_spaces_list, &mut spaces_idx);
                    free_spaces_idx = 0;
                    break;
                }
            }
        }
        assert_eq!(compacted.len(), ids.len());
        let mut checksum = 0;
        for (idx, val) in ids.iter().enumerate() {
            checksum += idx * val;
        }
        if let Input::Simple = INPUT {
            assert_eq!(checksum, 1928)
        }
        (checksum, ids)
    }
    fn next_num_to_push(files: &[u8], last_file_idx: &mut usize) -> u8 {
        let mut num_to_push = files[*last_file_idx];
        while num_to_push == 0 {
            *last_file_idx -= 1;
            num_to_push = files[*last_file_idx];
        }
        num_to_push
    }

    fn next_free_spaces(free_spaces_list: &[u8], idx: &mut usize) -> u8 {
        if *idx >= free_spaces_list.len() - 1 {
            return 0;
        }
        let mut free_spaces = free_spaces_list[*idx];
        while free_spaces == 0 {
            *idx += 1;
            if *idx >= free_spaces_list.len() - 1 {
                return 0;
            }
            free_spaces = free_spaces_list[*idx];
        }
        free_spaces
    }
}

fn main() {
    let filename = match INPUT {
        Input::Simple => "example.txt",
        Input::Default => "input.txt",
        Input::Simplest => "simplest.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    // Handle the case where no suffix is found
    let binding = String::from_utf8(input_file).unwrap();
    let disk_map = binding.trim_end();
    let compactor = Compactor::new(disk_map);
    let (checksum, _ids_simple) = compactor.run_compacting_simple_p1();

    println!("Checksum part 1 (simple way): {}", checksum);
    verify_checksum(checksum);

    let (checksum, _ids_simple) = compactor.run_compacting_p2();
    println!("Checksum part 2 (simple way): {}", checksum);
    match INPUT {
        Input::Simplest => (),
        Input::Simple => {
            assert_eq!(checksum, 2858);
        }
        Input::Default => (),
    }

    // The fancy way is not working yet and debugging is really annoying because it only fails
    // with the large input..
    /*
    let (checksum, ids_fancy) = compactor.run_compacting_fancy();
    println!("Checksum (fancy way): {}", checksum);
    assert_eq!(ids_simple.len(), ids_fancy.len());
    for (idx, (val_left, val_right)) in ids_simple.iter().zip(ids_fancy.iter()).enumerate() {
        if val_left != val_right {
            println!(
                "missmatch at idx {}, left {} right {}",
                idx, val_left, val_right
            );
            break;
        }
    }

    verify_checksum(checksum);
    */
}

fn verify_checksum(checksum: usize) {
    match INPUT {
        Input::Simple => assert_eq!(checksum, 1928),
        Input::Default => assert_eq!(checksum, 6288707484810),
        Input::Simplest => assert_eq!(checksum, 0),
    }
}
