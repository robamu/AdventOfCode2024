use core::panic;
use regex::Regex;
use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Copy, Clone)]
enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
pub struct Monitoring {
    wires: HashMap<String, Option<u8>>,
    operations: HashMap<String, HashMap<String, Vec<(Operator, String)>>>,
    z_vars: HashMap<String, usize>,
}

fn extract_number(input: &str) -> Option<u32> {
    let digits: String = input.chars().filter(|c| c.is_numeric()).collect();
    digits.parse::<u32>().ok()
}

impl Monitoring {
    pub fn new(data: &[u8]) -> Self {
        let mut wires = HashMap::new();
        let mut operations = HashMap::new();
        let mut z_vars = HashMap::new();
        let mut resolved_vars = Vec::new();
        let re = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();
        for line in data.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                continue;
            }
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim().to_string();
                // Safely parse the value
                let value = val.trim().parse().unwrap();
                wires.insert(key, Some(value));
            }
            if let Some(captures) = re.captures(&line) {
                let wire0 = captures.get(1).unwrap().as_str();
                let operator_str = captures.get(2).unwrap().as_str().to_lowercase();
                let op = match operator_str.as_str() {
                    "and" => Operator::And,
                    "or" => Operator::Or,
                    "xor" => Operator::Xor,
                    _ => panic!("Unknown operator"),
                };
                let wire1 = captures.get(3).unwrap().as_str();
                let dest_wire = captures.get(4).unwrap().as_str();
                let mut z_check = |wire: &str| {
                    if wire.contains("z") {
                        let index = extract_number(wire).unwrap() as usize;
                        z_vars.insert(wire.to_string(), index);
                    }
                };
                z_check(wire0);
                z_check(wire1);
                z_check(dest_wire);
                if let (Some(&Some(wire0_val)), Some(&Some(wire1_val))) =
                    (wires.get(wire0), wires.get(wire1))
                {
                    let value = Self::resolve_wire(wire0_val, wire1_val, op);
                    wires.insert(dest_wire.to_string(), Some(value));
                    resolved_vars.push((
                        wire0.to_string(),
                        wire1.to_string(),
                        dest_wire.to_string(),
                        value,
                    ));
                } else {
                    let mut handle_wire = |wire0: &str, wire1: &str| {
                        if !wires.contains_key(wire0) {
                            wires.insert(wire0.to_string(), None);
                        }
                        let entry = operations
                            .entry(wire0.to_string())
                            .or_insert_with(HashMap::new);

                        let ops_list = entry.entry(wire1.to_string()).or_insert_with(Vec::new);
                        ops_list.push((op, dest_wire.to_string()));
                    };
                    handle_wire(wire0, wire1);
                    handle_wire(wire1, wire0);
                    if let Some(Some(_)) = wires.get(dest_wire) {
                        panic!("dest wire with known value already inside wire map");
                    }
                    if !wires.contains_key(dest_wire) {
                        wires.insert(dest_wire.to_string(), None);
                    }
                }
            }
        }
        let mut monitoring = Self {
            wires,
            operations,
            z_vars,
        };
        for (wire_left, wire_right, dest, value) in &resolved_vars {
            monitoring.handle_resolved_vars(wire_left, wire_right, dest, *value);
        }
        monitoring
    }

    fn resolve_wire(wire_val_left: u8, wire_val_right: u8, operator: Operator) -> u8 {
        match operator {
            Operator::And => wire_val_left & wire_val_right,
            Operator::Or => wire_val_left | wire_val_right,
            Operator::Xor => wire_val_left ^ wire_val_right,
        }
    }

    pub fn handle_resolved_result_wire(&mut self, resolved_var: &str, result: u8) {
        let mut resolved_vars = Vec::new();
        if let Some(operations_for_wire) = self.operations.get(resolved_var) {
            for (other_wire, ops) in operations_for_wire.iter() {
                for (op, dest) in ops {
                    if let Some(Some(other_wire_val)) = self.wires.get(other_wire) {
                        let value = Self::resolve_wire(result, *other_wire_val, *op);
                        self.wires.insert(dest.to_string(), Some(value));
                        resolved_vars.push((
                            resolved_var.to_string(),
                            other_wire.clone(),
                            dest.to_string(),
                            value,
                        ));
                    }
                }
            }
        }
        for (wire_left, wire_right, dest, value) in &resolved_vars {
            self.handle_resolved_vars(wire_left, wire_right, dest, *value);
        }
    }

    pub fn handle_resolved_vars(
        &mut self,
        wire_left: &str,
        wire_right: &str,
        dest_wire: &str,
        value: u8,
    ) {
        let mut handle_wires = |wire: &str, other_wire: &str| {
            if let Some(op_map) = self.operations.get_mut(wire) {
                op_map.remove(other_wire);
                if op_map.is_empty() {
                    self.operations.remove(wire);
                }
            }
        };
        handle_wires(wire_left, wire_right);
        handle_wires(wire_right, wire_left);
        self.handle_resolved_result_wire(dest_wire, value);
    }

    pub fn check_all_operations(&mut self) {
        let mut resolved_vars = Vec::new();
        for (wire, op_map) in &mut self.operations {
            let wire_val = self.wires.get(wire).unwrap();
            if wire_val.is_none() {
                continue;
            }
            let wire_val = wire_val.unwrap();
            for (other_wire, ops) in op_map {
                for (op, dest) in ops {
                    let other_wire_val = self.wires.get(other_wire).unwrap();
                    if other_wire_val.is_none() {
                        continue;
                    }
                    let other_wire_val = other_wire_val.unwrap();
                    let value = Self::resolve_wire(wire_val, other_wire_val, *op);
                    self.wires.insert(dest.to_string(), Some(value));
                    resolved_vars.push((
                        wire.to_string(),
                        other_wire.to_string(),
                        dest.clone(),
                        value,
                    ));
                }
            }
        }
        for (wire, other, dest, value) in &resolved_vars {
            self.handle_resolved_vars(wire, other, dest, *value);
        }
    }

    pub fn result(&self) -> Option<u64> {
        if !self.operations.is_empty() {
            return None;
        }

        let mut number = 0;
        for (z_var, idx) in &self.z_vars {
            number |=
                (self.wires.get(z_var).unwrap().unwrap_or_else(|| {
                    panic!("{}", format!("no value at index {}", idx).to_string())
                }) as u64)
                    << idx;
        }
        Some(number)
    }
}

fn main() {
    let start = std::time::Instant::now();
    let input_file = std::fs::read("input.txt").unwrap();
    let mut monotoring = Monitoring::new(&input_file);
    while !monotoring.operations.is_empty() {
        monotoring.check_all_operations();
    }
    // No operations left.
    if monotoring.operations.is_empty() {
        let result = monotoring.result();
        println!("elapsed {}ms", start.elapsed().as_millis());
        println!("Result: {:?}", result);
        assert_eq!(result.unwrap(), 65635066541798);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example0() {
        let input_file = std::fs::read("example0.txt").unwrap();
        let mut monotoring = Monitoring::new(&input_file);
        while !monotoring.operations.is_empty() {
            monotoring.check_all_operations();
        }
        assert_eq!(monotoring.wires.len(), 9);
        assert_eq!(monotoring.result().unwrap(), 4);
    }

    #[test]
    fn test_example1() {
        let input_file = std::fs::read("example1.txt").unwrap();
        let mut monotoring = Monitoring::new(&input_file);
        while !monotoring.operations.is_empty() {
            monotoring.check_all_operations();
        }
        assert_eq!(monotoring.result().unwrap(), 2024);
    }
}
