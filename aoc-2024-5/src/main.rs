use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

pub type NumbersThatMustComeAfter = Vec<u32>;
pub type RuleMap = HashMap<u32, NumbersThatMustComeAfter>;

const DEBUG_P1: bool = false;
const DEBUG_P2: bool = false;

fn main() {
    let input_file = std::fs::read("input.txt").unwrap();
    let mut rule_map = RuleMap::default();
    let mut page_sets_to_produce = Vec::new();
    for line in input_file.lines() {
        let next_line = line.unwrap();
        if next_line.contains("|") {
            let next_number_pair: Vec<u32> = next_line
                .split('|')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            assert_eq!(next_number_pair.len(), 2);
            rule_map
                .entry(next_number_pair[0])
                .or_insert_with(Vec::new)
                .push(next_number_pair[1]);
        }
        if next_line.contains(",") {
            let next_page: Vec<u32> = next_line
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            assert!(!next_page.is_empty());
            page_sets_to_produce.push(next_page);
        }
    }
    //println!("Rule map: {:?}", rule_map);
    //println!("Pages to produce: {:?}", pages_to_produce);

    let mut mid_page_nums_added = 0;
    let mut invalid_page_sets = Vec::new();
    for page_set in page_sets_to_produce {
        let mut correctly_ordered = true;
        let mut finished_pages = HashSet::new();
        assert!(page_set.len() > 1);
        finished_pages.insert(page_set[0]);
        for page in page_set.iter().skip(1) {
            if let Some(numbers_must_come_after) = rule_map.get(page) {
                for number in numbers_must_come_after {
                    if finished_pages.contains(number) {
                        if DEBUG_P1 {
                            println!("Page set {:?} is not ordered correctly", page_set);
                            println!(
                            "Number {} which should come after number {} is in finished set {:?}",
                            number,
                            page,
                            finished_pages
                        );
                        }
                        correctly_ordered = false;
                        break;
                    }
                }
                if !correctly_ordered {
                    invalid_page_sets.push(page_set.clone());
                    break;
                }
            }

            finished_pages.insert(*page);
        }
        assert_ne!(page_set.len() % 2, 0);
        if correctly_ordered {
            if DEBUG_P1 {
                println!("Page set {:?} is ordered correctly", page_set);
                println!("Added number {}", page_set[page_set.len() / 2]);
            }
            mid_page_nums_added += page_set[page_set.len() / 2];
        }
    }
    println!("Result part 1: {}", mid_page_nums_added);

    // Part 2
    if DEBUG_P2 {
        println!("Invalid page sets:");
    }
    mid_page_nums_added = 0;
    for page_set in invalid_page_sets.iter_mut() {
        if DEBUG_P2 {
            println!("Invalid page set: {:?}", page_set);
        }
        loop {
            let mut finished_pages = Vec::new();
            let mut values_to_push_back = Vec::new();
            for page in page_set.iter() {
                if let Some(numbers_must_come_after) = rule_map.get(page) {
                    for number in numbers_must_come_after {
                        for finished_page in finished_pages.iter() {
                            if finished_page == number {
                                values_to_push_back.push(*finished_page);
                            }
                        }
                    }
                    if !values_to_push_back.is_empty() {
                        break;
                    }
                }
                finished_pages.push(*page);
            }
            // We are done, nothing to re-order
            if values_to_push_back.is_empty() {
                if DEBUG_P2 {
                    println!("Ordered: {:?}", page_set);
                }
                mid_page_nums_added += page_set[page_set.len() / 2];
                break;
            }
            // We push back the values that should come after a certain number and then re-run the
            // algorithm from the start.
            page_set.retain(|val| !values_to_push_back.contains(val));
            for value in values_to_push_back {
                page_set.push(value);
            }
        }
    }
    println!("Result part 2: {}", mid_page_nums_added);
}
