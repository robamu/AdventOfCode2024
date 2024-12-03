use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
pub enum Input {
    Example0,
    Default,
}

const INPUT: Input = Input::Example0;
const DEBUG: bool = false;

#[derive(Debug, Clone, Copy, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Computer {
    a: u64,
    b: u64,
    c: u64,

    /// Instruction pointer
    ip: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum ExecResult {
    Ok,
    Halted,
    IterativeStepP2,
    MissmatchP2,
}

impl Computer {
    pub fn new_from_data(data: &[u8]) -> (Self, Vec<u8>) {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut memory = Vec::new();
        for line in data.lines() {
            let line = line.unwrap();
            if line.starts_with("Register A:") {
                a = line.split(':').nth(1).unwrap().trim().parse().unwrap();
            } else if line.starts_with("Register B:") {
                b = line.split(':').nth(1).unwrap().trim().parse().unwrap();
            } else if line.starts_with("Register C:") {
                c = line.split(':').nth(1).unwrap().trim().parse().unwrap();
            } else if line.starts_with("Program:") {
                memory = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split(',')
                    .map(|x| u8::from_str(x.trim()).unwrap())
                    .collect();
            }
        }
        (Self::new(a, b, c), memory)
    }

    pub fn new_at_state(a: u64, b: u64, c: u64, ip: usize) -> Computer {
        Computer { a, b, c, ip }
    }

    pub fn new(a: u64, b: u64, c: u64) -> Computer {
        Computer::new_at_state(a, b, c, 0)
    }

    pub fn execute(&mut self, memory: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        while let ExecResult::Ok = self.execute_next_instruction(memory, &mut output) {}
        output
    }

    pub fn execute_p2_brute_force(&mut self, a: u64, memory: &[u8]) -> bool {
        self.reset();
        self.a = a;
        let mut output = Vec::new();
        let mut state_chain = Vec::new();
        while let ExecResult::Ok =
            self.execute_next_instruction_p2(a, memory, &mut state_chain, &mut output)
        {}
        output == memory
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.ip = 0;
    }

    pub fn execute_next_instruction_p2(
        &mut self,
        _init_a: u64,
        memory: &[u8],
        state_chain: &mut Vec<Computer>,
        out: &mut Vec<u8>,
    ) -> ExecResult {
        if self.ip >= memory.len() {
            return ExecResult::Halted;
        }
        state_chain.push(*self);
        let next_instruction = Instruction::try_from(memory[self.ip]).unwrap();
        let next_op = memory[self.ip + 1];
        match next_instruction {
            Instruction::Adv => self.adv(next_op),
            Instruction::Bxl => self.bxl(next_op),
            Instruction::Bst => self.bst(next_op),
            Instruction::Jnz => {
                if self.jnz(next_op) {
                    return ExecResult::Ok;
                }
            }
            Instruction::Bxc => self.bxc(),
            Instruction::Out => {
                out.push(self.out(next_op));
                if *out.last().unwrap() != memory[out.len() - 1] {
                    return ExecResult::MissmatchP2;
                }
            }
            Instruction::Bdv => self.bdv(next_op),
            Instruction::Cdv => self.cdv(next_op),
        }
        self.ip += 2;
        ExecResult::Ok
    }

    pub fn execute_next_instruction(&mut self, memory: &[u8], out: &mut Vec<u8>) -> ExecResult {
        if self.ip >= memory.len() {
            return ExecResult::Halted;
        }
        let next_instruction = Instruction::try_from(memory[self.ip]).unwrap();
        let next_op = memory[self.ip + 1];
        match next_instruction {
            Instruction::Adv => self.adv(next_op),
            Instruction::Bxl => self.bxl(next_op),
            Instruction::Bst => self.bst(next_op),
            Instruction::Jnz => {
                if self.jnz(next_op) {
                    return ExecResult::Ok;
                }
            }
            Instruction::Bxc => self.bxc(),
            Instruction::Out => out.push(self.out(next_op)),
            Instruction::Bdv => self.bdv(next_op),
            Instruction::Cdv => self.cdv(next_op),
        }
        self.ip += 2;
        ExecResult::Ok
    }

    pub fn combo_op(&self, op: u8) -> u64 {
        match op {
            0..=3 => op as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid op"),
        }
    }

    pub fn div_op(&self, op: u8) -> u64 {
        self.a >> self.combo_op(op)
    }

    pub fn adv(&mut self, op: u8) {
        self.a = self.div_op(op)
    }

    pub fn bxl(&mut self, op: u8) {
        self.b ^= op as u64
    }

    pub fn bst(&mut self, op: u8) {
        self.b = self.combo_op(op) % 8
    }

    pub fn jnz(&mut self, op: u8) -> bool {
        if self.a == 0 || self.ip == op as usize {
            return false;
        }
        self.ip = op as usize;
        true
    }

    pub fn bxc(&mut self) {
        self.b ^= self.c
    }

    pub fn out(&mut self, op: u8) -> u8 {
        (self.combo_op(op) % 8).try_into().unwrap()
    }

    pub fn bdv(&mut self, op: u8) {
        self.b = self.div_op(op)
    }

    pub fn cdv(&mut self, op: u8) {
        self.c = self.div_op(op)
    }
}

fn test_sample_inputs() {
    let mut computer = Computer::new(0, 0, 9);
    computer.execute(&[2, 6]);
    assert_eq!(computer.b, 1);
    let mut computer = Computer::new(10, 0, 0);
    let mut out = computer.execute(&[5, 0, 5, 1, 5, 4]);
    assert_eq!(out, &[0, 1, 2]);
    computer = Computer::new(2024, 0, 0);
    out = computer.execute(&[0, 1, 5, 4, 3, 0]);
    assert_eq!(out, &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    assert_eq!(computer.a, 0);
    computer = Computer::new(0, 29, 0);
    computer.execute(&[1, 7]);
    assert_eq!(computer.b, 26);
    computer = Computer::new(0, 2024, 43690);
    computer.execute(&[4, 0]);
    assert_eq!(computer.b, 44354);
}

fn test_p2() {
    let mut a = 0;
    let mut computer = Computer::new(a, 0, 0);
    let memory = &[0, 3, 5, 4, 3, 0];
    loop {
        if computer.execute_p2_brute_force(a, memory) {
            assert_eq!(a, 117440);
            break;
        }
        a += 1;
    }
}

fn decompiled_program(mut a: u64) -> (u64, u8) {
    // Program: 2,4,1,5,7,5,1,6,4,1,5,5,0,3,3,0
    //  1. (2, 4): bst(4): b = a & 7
    //  2. (1, 5): bxl(5): b = b ^ 5
    //  3. (7, 5): cdv(5): c = a / (2 pow B) = a >> b
    //  4. (1, 6): bxl(6): b = b ^ 6
    //  5. (4, 1): bxc(1): b = b ^ c
    //  6. (5, 5): out(5): out (b % 8) = out (b & 7)
    //  7. (0, 3): adv(3): a = a / (2 pow 3) = a >> 3
    //  8. (3, 0): jnz(0): if a == 0 { nothing } else { ip = 0 }

    // Insights:
    //  1. The program loops until a is 0. a is always shifted 3 to the right and will eventually
    //     become 0
    //  2. The output of the program is the next a value and the output value. b and c and derived.
    //  3. Derived insight: The last value of the output is generated with a being between 0 and 7.
    //
    let mut b = a & 7;
    b ^= 5;
    let c = a >> b;
    b ^= 6;
    b ^= c;
    let out = (b & 7) as u8;
    a >>= 3;
    (a, out)
}

fn reverse_engineered_solution(program: &[u8]) {
    // Viable inputs (a) to generate the next program part.
    let mut viable_inputs = vec![0];
    // Preallocate space for the next viable inputs to avoid frequent allocations
    let mut next_viable_inputs = Vec::with_capacity(8 * viable_inputs.len());
    let len_program = program.len();
    for i in (0..len_program).rev() {
        for &a_candidate in &viable_inputs {
            // a candidates need to be shifted because this is the last step of the decompiled
            // program which we can reverse.
            let shifted_a = a_candidate << 3;
            // We try all variations of the last three bits.
            for last_three_bits in 0..8 {
                let candidate = shifted_a | last_three_bits as u64;
                // We try out all candidates for the next expected output at the index.
                let (_next_a, out) = decompiled_program(candidate);
                if out == program[i] {
                    next_viable_inputs.push(candidate);
                }
            }
        }
        // Swap vectors instead of cloning and clearing
        std::mem::swap(&mut viable_inputs, &mut next_viable_inputs);
        next_viable_inputs.clear();
    }
    if DEBUG {
        println!("Viable inputs to generate the program: {:?}", viable_inputs);
    }
    // We check whether this really works.
    for a in &viable_inputs {
        let mut next_a = *a;
        let mut out_list = Vec::new();
        while next_a > 0 {
            let (next_a_new, out) = decompiled_program(next_a);
            next_a = next_a_new;
            out_list.push(out);
        }
        assert_eq!(out_list, program);
    }
    println!(
        "solution for part 2: {:?}",
        viable_inputs.iter().min().unwrap()
    );
}

fn main() {
    test_sample_inputs();

    let filename = match INPUT {
        Input::Example0 => "example.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let (mut computer, memory) = Computer::new_from_data(&input_file);
    println!("computer: {:?}", computer);
    let out = computer.execute(&memory);
    let solution = out
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("solution: {:?}", solution);
    match INPUT {
        Input::Example0 => assert_eq!(solution, "4,6,3,5,6,3,5,2,1,0"),
        Input::Default => assert_eq!(solution, "3,5,0,1,5,1,5,1,0"),
    }
    test_p2();
    reverse_engineered_solution(&memory);
}

pub fn solution_copied_from_python(program: &[u8]) {
    let mut computer = Computer::new(0, 0, 0);
    let mut a = 0;
    // Solution was copied, could not figure it out. Apparently, the solution can be built
    // from backwards because the last part does not change if A is incremented in octals.
    for i in (0..program.len()).rev() {
        if DEBUG {
            println!(
                "computing partial solution which should result in {:?}",
                &program[i..]
            );
        }
        a <<= 3;
        computer.reset();
        computer.a = a;
        while computer.execute(program) != program[i..] {
            computer.reset();
            a += 1;
            computer.a = a;
        }
    }
    println!("solution for part 2: {:?}", a);
    computer.reset();
    computer.a = a;
    assert_eq!(&computer.execute(program), &program);
}
