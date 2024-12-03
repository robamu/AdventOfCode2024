use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
pub enum Input {
    Example,
    Default,
}

pub type ComputerMap = HashMap<String, HashSet<String>>;
fn read_computer_map(data: &[u8]) -> ComputerMap {
    let mut computer_map = HashMap::new();
    for line in data.lines() {
        let line = line.unwrap();
        let computer_pair: Vec<&str> = line.split('-').collect();
        computer_map
            .entry(computer_pair[0].to_string())
            .or_insert_with(|| {
                let mut hashset = HashSet::new();
                hashset.insert(computer_pair[1].to_string());
                hashset
            })
            .insert(computer_pair[1].to_string());
        computer_map
            .entry(computer_pair[1].to_string())
            .or_insert_with(|| {
                let mut hashset = HashSet::new();
                hashset.insert(computer_pair[0].to_string());
                hashset
            })
            .insert(computer_pair[0].to_string());
    }
    computer_map
}

fn find_triple_sets(computer_map: &ComputerMap) -> HashSet<Vec<&String>> {
    let mut triple_sets = HashSet::new();
    for (pc, connected_list) in computer_map {
        for (i, connected_pc) in connected_list.iter().enumerate() {
            for other_connected_pc in connected_list.iter().skip(i + 1) {
                if computer_map[connected_pc].contains(other_connected_pc)
                    && computer_map[connected_pc].contains(pc)
                {
                    let mut triple_set = vec![pc, connected_pc, other_connected_pc];
                    triple_set.sort();
                    // This is the condition for a full set
                    triple_sets.insert(triple_set);
                }
            }
        }
    }
    triple_sets
}
fn main() {
    let start = std::time::Instant::now();
    let input_file = std::fs::read("input.txt").unwrap();
    let computer_map = read_computer_map(&input_file);
    let triple_sets = find_triple_sets(&computer_map);
    let relevant_sets = triple_sets
        .iter()
        .filter(|set| set.iter().any(|s| s.starts_with("t")))
        .collect::<Vec<_>>();
    println!("elapsed: {}ms", start.elapsed().as_millis());
    println!("solution p1: ");
    println!("{}", relevant_sets.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input_file = std::fs::read("example.txt").unwrap();
        let computer_map = read_computer_map(&input_file);
        let triple_sets = find_triple_sets(&computer_map);
        assert_eq!(triple_sets.len(), 12);
        let relevant_sets = triple_sets
            .iter()
            .filter(|set| set.iter().any(|s| s.starts_with("t")))
            .collect::<Vec<_>>();
        assert_eq!(relevant_sets.len(), 7);
    }
}
