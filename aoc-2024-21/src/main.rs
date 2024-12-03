use std::vec;

const DEBUG: bool = false;

#[derive(Debug)]
pub enum Input {
    Example,
    Default,
}

/// Numpad
///
/// 7 | 8 | 9
/// 4 | 5 | 6
/// 1 | 2 | 3
///   | 0 | A
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Numpad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

impl TryFrom<char> for Numpad {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let numpad = match value {
            '0' => Numpad::Zero,
            '1' => Numpad::One,
            '2' => Numpad::Two,
            '3' => Numpad::Three,
            '4' => Numpad::Four,
            '5' => Numpad::Five,
            '6' => Numpad::Six,
            '7' => Numpad::Seven,
            '8' => Numpad::Eight,
            '9' => Numpad::Nine,
            'A' => Numpad::Activate,
            _ => return Err(()),
        };
        Ok(numpad)
    }
}

impl Numpad {
    pub fn get_directions(&self, target: Numpad) -> &[&[Button]] {
        get_directions_numpad(*self, target)
    }
}

/// Directional buttons
///
///   | ^ | A
/// < | v | >
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

pub type Btn = Button;

pub mod comb {

    use super::*;
    pub const U: &[&[Btn]] = &[&[Btn::Up]];
    pub const UU: &[&[Btn]] = &[&[Btn::Up, Btn::Up]];
    pub const UUU: &[&[Btn]] = &[&[Btn::Up, Btn::Up, Btn::Up]];
    pub const D: &[&[Btn]] = &[&[Btn::Down]];
    pub const DD: &[&[Btn]] = &[&[Btn::Down, Btn::Down]];
    pub const DDD: &[&[Btn]] = &[&[Btn::Down, Btn::Down, Btn::Down]];
    pub const L: &[&[Btn]] = &[&[Btn::Left]];
    pub const LL: &[&[Btn]] = &[&[Btn::Left, Btn::Left]];
    pub const R: &[&[Btn]] = &[&[Btn::Right]];
    pub const RR: &[&[Btn]] = &[&[Btn::Right, Btn::Right]];
    pub const RU: &[&[Btn]] = &[&[Btn::Right, Btn::Up], &[Btn::Up, Btn::Right]];
    pub const RUU: &[&[Btn]] = &[
        &[Btn::Right, Btn::Up, Btn::Up],
        &[Btn::Up, Btn::Right, Btn::Up],
        &[Btn::Up, Btn::Up, Btn::Right],
    ];
    pub const RUUU: &[&[Btn]] = &[
        &[Btn::Right, Btn::Up, Btn::Up, Btn::Up],
        &[Btn::Up, Btn::Right, Btn::Right, Btn::Up],
        &[Btn::Up, Btn::Up, Btn::Right, Btn::Up],
        &[Btn::Up, Btn::Up, Btn::Up, Btn::Right],
    ];
    pub const RRUU: &[&[Btn]] = &[
        &[Btn::Right, Btn::Right, Btn::Up, Btn::Up],
        &[Btn::Right, Btn::Up, Btn::Up, Btn::Right],
        &[Btn::Up, Btn::Up, Btn::Right, Btn::Right],
        &[Btn::Right, Btn::Up, Btn::Right, Btn::Up],
    ];
    pub const RRDD: &[&[Btn]] = &[
        &[Btn::Right, Btn::Right, Btn::Down, Btn::Down],
        &[Btn::Right, Btn::Down, Btn::Down, Btn::Right],
        &[Btn::Down, Btn::Down, Btn::Right, Btn::Right],
        &[Btn::Right, Btn::Down, Btn::Right, Btn::Down],
    ];
    pub const RRDDD: &[&[Btn]] = &[
        &[Btn::Right, Btn::Right, Btn::Down, Btn::Down, Btn::Down],
        &[Btn::Right, Btn::Down, Btn::Right, Btn::Down, Btn::Down],
        &[Btn::Right, Btn::Down, Btn::Down, Btn::Right, Btn::Down],
        &[Btn::Right, Btn::Down, Btn::Down, Btn::Down, Btn::Right],
        &[Btn::Down, Btn::Right, Btn::Right, Btn::Down, Btn::Down],
        &[Btn::Down, Btn::Right, Btn::Down, Btn::Right, Btn::Down],
        &[Btn::Down, Btn::Right, Btn::Down, Btn::Down, Btn::Right],
        &[Btn::Down, Btn::Down, Btn::Right, Btn::Right, Btn::Down],
        &[Btn::Down, Btn::Down, Btn::Right, Btn::Down, Btn::Right],
        &[Btn::Down, Btn::Down, Btn::Down, Btn::Right, Btn::Right],
    ];
    pub const LLUUU: &[&[Btn]] = &[
        &[Btn::Left, Btn::Left, Btn::Up, Btn::Up, Btn::Up],
        &[Btn::Left, Btn::Up, Btn::Left, Btn::Up, Btn::Up],
        &[Btn::Left, Btn::Up, Btn::Up, Btn::Left, Btn::Up],
        &[Btn::Left, Btn::Up, Btn::Up, Btn::Up, Btn::Left],
        &[Btn::Up, Btn::Left, Btn::Left, Btn::Up, Btn::Up],
        &[Btn::Up, Btn::Left, Btn::Up, Btn::Left, Btn::Up],
        &[Btn::Up, Btn::Left, Btn::Up, Btn::Up, Btn::Left],
        &[Btn::Up, Btn::Up, Btn::Left, Btn::Left, Btn::Up],
        &[Btn::Up, Btn::Up, Btn::Left, Btn::Up, Btn::Left],
        &[Btn::Up, Btn::Up, Btn::Up, Btn::Left, Btn::Left],
    ];
    pub const RRU: &[&[Btn]] = &[
        &[Btn::Right, Btn::Right, Btn::Up],
        &[Btn::Right, Btn::Up, Btn::Right],
        &[Btn::Up, Btn::Right, Btn::Right],
    ];
    pub const RRD: &[&[Btn]] = &[
        &[Btn::Right, Btn::Right, Btn::Down],
        &[Btn::Right, Btn::Down, Btn::Right],
        &[Btn::Down, Btn::Right, Btn::Right],
    ];
    pub const RD: &[&[Btn]] = &[&[Btn::Down, Btn::Right], &[Btn::Right, Btn::Down]];
    pub const RDD: &[&[Btn]] = &[
        &[Btn::Right, Btn::Down, Btn::Down],
        &[Btn::Down, Btn::Right, Btn::Down],
        &[Btn::Down, Btn::Down, Btn::Right],
    ];
    pub const RDDD: &[&[Btn]] = &[
        &[Btn::Right, Btn::Down, Btn::Down, Btn::Down],
        &[Btn::Down, Btn::Right, Btn::Down, Btn::Down],
        &[Btn::Down, Btn::Down, Btn::Right, Btn::Down],
        &[Btn::Down, Btn::Down, Btn::Down, Btn::Right],
    ];
    pub const LU: &[&[Btn]] = &[&[Btn::Left, Btn::Up], &[Btn::Up, Btn::Left]];
    pub const LUU: &[&[Btn]] = &[
        &[Btn::Left, Btn::Up, Btn::Up],
        &[Btn::Up, Btn::Left, Btn::Up],
        &[Btn::Up, Btn::Up, Btn::Left],
    ];

    pub const LUUU: &[&[Btn]] = &[
        &[Btn::Left, Btn::Left, Btn::Up, Btn::Up],
        &[Btn::Up, Btn::Left, Btn::Up, Btn::Up],
        &[Btn::Up, Btn::Up, Btn::Left, Btn::Up],
        &[Btn::Up, Btn::Up, Btn::Up, Btn::Left],
    ];
    pub const LLU: &[&[Btn]] = &[
        &[Btn::Left, Btn::Left, Btn::Up],
        &[Btn::Left, Btn::Up, Btn::Left],
        &[Btn::Up, Btn::Left, Btn::Left],
    ];
    pub const LLUU: &[&[Btn]] = &[
        &[Btn::Left, Btn::Left, Btn::Up, Btn::Up],
        &[Btn::Left, Btn::Up, Btn::Up, Btn::Left],
        &[Btn::Up, Btn::Up, Btn::Left, Btn::Left],
        &[Btn::Left, Btn::Up, Btn::Left, Btn::Up],
    ];
    pub const LD: &[&[Btn]] = &[&[Btn::Left, Btn::Down], &[Btn::Down, Btn::Left]];
    pub const LLD: &[&[Btn]] = &[
        &[Btn::Left, Btn::Left, Btn::Down],
        &[Btn::Down, Btn::Left, Btn::Left],
        &[Btn::Left, Btn::Down, Btn::Left],
    ];
    pub const LLDD: &[&[Btn]] = &[
        &[Btn::Left, Btn::Left, Btn::Down, Btn::Down],
        &[Btn::Left, Btn::Down, Btn::Left, Btn::Down],
        &[Btn::Left, Btn::Down, Btn::Down, Btn::Left],
        &[Btn::Down, Btn::Left, Btn::Left, Btn::Down],
        &[Btn::Down, Btn::Left, Btn::Down, Btn::Left],
        &[Btn::Down, Btn::Down, Btn::Left, Btn::Left],
    ];
    pub const LDD: &[&[Btn]] = &[
        &[Btn::Left, Btn::Down, Btn::Down],
        &[Btn::Down, Btn::Left, Btn::Down],
        &[Btn::Down, Btn::Down, Btn::Left],
    ];
    pub const LDDD: &[&[Btn]] = &[
        &[Btn::Left, Btn::Down, Btn::Down, Btn::Down],
        &[Btn::Down, Btn::Left, Btn::Down, Btn::Down],
        &[Btn::Down, Btn::Down, Btn::Left, Btn::Down],
        &[Btn::Down, Btn::Down, Btn::Down, Btn::Left],
    ];
}

impl Button {
    pub const fn negate(&self) -> Self {
        match self {
            Button::Up => Button::Down,
            Button::Down => Button::Up,
            Button::Left => Button::Right,
            Button::Right => Button::Left,
            Button::Activate => Button::Activate,
        }
    }
}

pub fn get_directions_dirpad(start: Button, target: Button) -> &'static [&'static [Button]] {
    if start == target {
        panic!("start and target are the same");
    }
    match start {
        Button::Up => match target {
            Button::Up => unreachable!(),
            Button::Down => comb::D,
            Button::Left => &comb::LD[1..],
            Button::Right => comb::RD,
            Button::Activate => comb::R,
        },
        Button::Down => match target {
            Button::Up => comb::U,
            Button::Down => unreachable!(),
            Button::Left => comb::L,
            Button::Right => comb::R,
            Button::Activate => comb::RU,
        },
        Button::Left => match target {
            Button::Up => &comb::RU[..1],
            Button::Down => comb::R,
            Button::Left => unreachable!(),
            Button::Right => comb::RR,
            Button::Activate => &comb::RRU[..2],
        },
        Button::Right => match target {
            Button::Up => comb::LU,
            Button::Down => comb::L,
            Button::Left => comb::LL,
            Button::Right => unreachable!(),
            Button::Activate => comb::U,
        },
        Button::Activate => match target {
            Button::Up => comb::L,
            Button::Down => comb::LD,
            Button::Left => &comb::LLD[1..],
            Button::Right => comb::D,
            Button::Activate => unreachable!(),
        },
    }
}

pub fn get_directions_numpad(start: Numpad, target: Numpad) -> &'static [&'static [Button]] {
    if start == target {
        panic!("start and target are the same");
    }
    match start {
        Numpad::Zero => match target {
            Numpad::Zero => unreachable!(),
            Numpad::One => &comb::LU[1..],
            Numpad::Two => comb::U,
            Numpad::Three => comb::RU,
            Numpad::Four => comb::LUU,
            Numpad::Five => comb::UU,
            Numpad::Six => comb::LUU,
            Numpad::Seven => &comb::LUUU[1..],
            Numpad::Eight => comb::UUU,
            Numpad::Nine => comb::RUUU,
            Numpad::Activate => comb::R,
        },
        Numpad::One => match target {
            Numpad::Zero => &comb::RD[1..],
            Numpad::One => unreachable!(),
            Numpad::Two => comb::R,
            Numpad::Three => comb::RR,
            Numpad::Four => comb::U,
            Numpad::Five => comb::RU,
            Numpad::Six => comb::RRU,
            Numpad::Seven => comb::UU,
            Numpad::Eight => comb::RUU,
            Numpad::Nine => comb::RRUU,
            Numpad::Activate => comb::RRD,
        },
        Numpad::Two => match target {
            Numpad::Zero => comb::D,
            Numpad::One => comb::L,
            Numpad::Two => unreachable!(),
            Numpad::Three => comb::R,
            Numpad::Four => comb::LU,
            Numpad::Five => comb::U,
            Numpad::Six => comb::RU,
            Numpad::Seven => comb::LUU,
            Numpad::Eight => comb::UU,
            Numpad::Nine => comb::RUU,
            Numpad::Activate => comb::RD,
        },
        Numpad::Three => match target {
            Numpad::Zero => comb::LD,
            Numpad::One => comb::LL,
            Numpad::Two => comb::L,
            Numpad::Three => unreachable!(),
            Numpad::Four => comb::LLU,
            Numpad::Five => comb::LU,
            Numpad::Six => comb::U,
            Numpad::Seven => comb::LLUU,
            Numpad::Eight => comb::LLU,
            Numpad::Nine => comb::UU,
            Numpad::Activate => comb::D,
        },
        Numpad::Four => match target {
            Numpad::Zero => comb::RDD,
            Numpad::One => comb::D,
            Numpad::Two => comb::RD,
            Numpad::Three => comb::RRD,
            Numpad::Four => unreachable!(),
            Numpad::Five => comb::R,
            Numpad::Six => comb::RR,
            Numpad::Seven => comb::U,
            Numpad::Eight => comb::RU,
            Numpad::Nine => comb::RRU,
            Numpad::Activate => comb::RRDD,
        },
        Numpad::Five => match target {
            Numpad::Zero => comb::DD,
            Numpad::One => comb::LD,
            Numpad::Two => comb::D,
            Numpad::Three => comb::RD,
            Numpad::Four => comb::L,
            Numpad::Five => unreachable!(),
            Numpad::Six => comb::R,
            Numpad::Seven => comb::LU,
            Numpad::Eight => comb::U,
            Numpad::Nine => comb::RU,
            Numpad::Activate => comb::RDD,
        },
        Numpad::Six => match target {
            Numpad::Zero => comb::LDD,
            Numpad::One => comb::LLD,
            Numpad::Two => comb::LD,
            Numpad::Three => comb::D,
            Numpad::Four => comb::LL,
            Numpad::Five => comb::L,
            Numpad::Six => unreachable!(),
            Numpad::Seven => comb::LLU,
            Numpad::Eight => comb::LU,
            Numpad::Nine => comb::U,
            Numpad::Activate => comb::DD,
        },
        Numpad::Seven => match target {
            Numpad::Zero => &comb::RDDD[..3],
            Numpad::One => comb::DD,
            Numpad::Two => comb::RDD,
            Numpad::Three => comb::RRDD,
            Numpad::Four => comb::D,
            Numpad::Five => comb::RD,
            Numpad::Six => comb::RRD,
            Numpad::Seven => unreachable!(),
            Numpad::Eight => comb::R,
            Numpad::Nine => comb::RR,
            Numpad::Activate => comb::RRDDD,
        },
        Numpad::Eight => match target {
            Numpad::Zero => comb::DDD,
            Numpad::One => comb::LDD,
            Numpad::Two => comb::DD,
            Numpad::Three => comb::RDD,
            Numpad::Four => comb::LD,
            Numpad::Five => comb::D,
            Numpad::Six => comb::RD,
            Numpad::Seven => comb::L,
            Numpad::Eight => unreachable!(),
            Numpad::Nine => comb::R,
            Numpad::Activate => comb::RDDD,
        },
        Numpad::Nine => match target {
            Numpad::Zero => comb::LDDD,
            Numpad::One => comb::LLDD,
            Numpad::Two => comb::LDD,
            Numpad::Three => comb::DD,
            Numpad::Four => comb::LLD,
            Numpad::Five => comb::LD,
            Numpad::Six => comb::D,
            Numpad::Seven => comb::LL,
            Numpad::Eight => comb::L,
            Numpad::Nine => unreachable!(),
            Numpad::Activate => comb::DDD,
        },
        Numpad::Activate => match target {
            Numpad::Zero => comb::L,
            Numpad::One => &comb::LLU[1..],
            Numpad::Two => comb::LU,
            Numpad::Three => comb::U,
            Numpad::Four => &comb::LLUU[1..],
            Numpad::Five => comb::LUU,
            Numpad::Six => comb::UU,
            Numpad::Seven => &comb::LLUUU[1..],
            Numpad::Eight => comb::LUUU,
            Numpad::Nine => comb::UUU,
            Numpad::Activate => unreachable!(),
        },
    }
}

pub const DOOR_CODE_EXAMPLE: [&str; 5] = ["029A", "980A", "179A", "456A", "379A"];
pub const DOOR_CODE_INPUT: [&str; 5] = ["129A", "176A", "985A", "170A", "528A"];

#[derive(Debug)]
pub enum ButtonSequence {
    Single(Vec<Button>),
    Multi(Vec<Vec<Button>>),
}

impl ButtonSequence {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match self {
            ButtonSequence::Single(vec) => vec.len(),
            ButtonSequence::Multi(vec_list) => {
                vec_list
                    .iter()
                    .map(|sublist| sublist.len()) // Get the length of each sublist
                    .min()
                    .unwrap() // Find the smallest length
            }
        }
    }
}

#[derive(Debug)]
pub enum ButtonGroup {
    Single(Button),
    Multi(Vec<Button>),
}

fn main() {
    let mut sum = 0;
    for example_code in DOOR_CODE_EXAMPLE {
        println!("Handling example code: {}", example_code);
        let complexity = brute_force_keypad_calculation(example_code);
        match example_code {
            "029A" => assert_eq!(complexity, 68 * 29),
            "980A" => assert_eq!(complexity, 60 * 980),
            "179A" => assert_eq!(complexity, 68 * 179),
            "456A" => assert_eq!(complexity, 64 * 456),
            "379A" => assert_eq!(complexity, 64 * 379),
            _ => panic!("Unexpected example code"),
        }
        println!(
            "Complexity for combination {}: {}",
            example_code, complexity
        );
        sum += complexity;
    }
    println!("Sum of complexities: {}", sum);
    assert_eq!(sum, 126384);
    sum = 0;
    for input in DOOR_CODE_INPUT {
        let complexity = brute_force_keypad_calculation(input);
        sum += complexity;
    }
    println!("solution p1: {}", sum);
}

fn build_button_sequences_for_robot(
    target_button_presses: Vec<Vec<Button>>,
) -> Vec<Vec<ButtonSequence>> {
    let mut list_of_smallest_sequences = Vec::new();
    let mut min_len = usize::MAX;
    for sequence in target_button_presses {
        let mut next_sequence = vec![Button::Activate];
        next_sequence.extend(sequence.clone());
        let mut list_of_sequences = Vec::new();
        for (&first, &second) in next_sequence.iter().zip(next_sequence.iter().skip(1)) {
            if first == second {
                list_of_sequences.push(ButtonSequence::Single(vec![Button::Activate]));
                continue;
            }
            let directions = get_directions_dirpad(first, second);
            if directions.len() == 1 {
                let mut directions = directions[0].to_vec();
                directions.push(Button::Activate);
                list_of_sequences.push(ButtonSequence::Single(directions));
            } else {
                let mut mulitple_sequences = Vec::new();
                for direction in directions.iter() {
                    let mut next_direction = direction.to_vec();
                    next_direction.push(Button::Activate);
                    mulitple_sequences.push(next_direction);
                }
                list_of_sequences.push(ButtonSequence::Multi(mulitple_sequences));
            }
        }
        // Apparently it is not an issue to discard solutions with the same length..
        let len = list_of_sequences
            .iter()
            .map(|sequence| sequence.len())
            .sum::<usize>();
        #[allow(clippy::comparison_chain)]
        if len < min_len {
            min_len = len;
            list_of_smallest_sequences = vec![list_of_sequences];
        } else if len == min_len {
            list_of_smallest_sequences.push(list_of_sequences);
        }
    }
    list_of_smallest_sequences
}

/// Only keep the sequences with the smallest length.
pub fn reduce_button_sequence_list(sequence: &mut Vec<Vec<Button>>) {
    let min_len_r1 = sequence
        .iter()
        .map(|sequence| sequence.len())
        .min()
        .unwrap();
    sequence.retain(|v| v.len() == min_len_r1);
}

fn extract_number(input: &str) -> Option<u32> {
    let numeric_str: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    numeric_str.parse::<u32>().ok()
}

/// The brute force solution repeatedly builds the full set of button presses which the next robot
/// or the human operator needs to press. It then flattens the set and only retains the sets with
/// the smallest amount of button presses so it can be used as an input for the next calculation.
fn brute_force_keypad_calculation(combination: &str) -> usize {
    let mut list_of_sequences = Vec::new();
    let mut combination_prepended = vec![Numpad::Activate];
    for c in combination.chars() {
        combination_prepended.push(Numpad::try_from(c).unwrap());
    }
    let number = extract_number(combination).unwrap();
    for (&prev, &next) in combination_prepended
        .iter()
        .zip(combination_prepended.iter().skip(1))
    {
        let directions = get_directions_numpad(prev, next);
        if directions.len() == 1 {
            let mut directions = directions[0].to_vec();
            directions.push(Button::Activate);
            list_of_sequences.push(ButtonSequence::Single(directions));
        } else {
            let mut mulitple_sequences = Vec::new();
            for direction in directions.iter() {
                let mut next_direction = direction.to_vec();
                next_direction.push(Button::Activate);
                mulitple_sequences.push(next_direction);
            }
            list_of_sequences.push(ButtonSequence::Multi(mulitple_sequences));
        }
    }
    if DEBUG {
        println!("List of sequences for R0: {:?}", list_of_sequences);
    }
    let mut sequences_r0 = flatten_button_sequence(&list_of_sequences);
    reduce_button_sequence_list(&mut sequences_r0);
    if DEBUG {
        println!("Flattended sequence R0: {:?}", sequences_r0);
    }

    let nested_list_of_sequences = build_button_sequences_for_robot(sequences_r0);
    let mut sequences_r1 = Vec::new();
    for list_of_sequences in nested_list_of_sequences.iter() {
        sequences_r1.extend(flatten_button_sequence(list_of_sequences));
    }
    reduce_button_sequence_list(&mut sequences_r1);
    if DEBUG {
        println!(
            "{} flattended sequences for R1 with min length {:?}",
            sequences_r1.len(),
            sequences_r1[0].len()
        );
    }

    let nested_list_of_sequences = build_button_sequences_for_robot(sequences_r1);
    let mut sequences_r2 = Vec::new();
    for list_of_sequences in nested_list_of_sequences.iter() {
        sequences_r2.extend(flatten_button_sequence(list_of_sequences));
    }
    reduce_button_sequence_list(&mut sequences_r2);
    if DEBUG {
        println!(
            "{} flattended sequences for R1 with min length {:?}",
            sequences_r2.len(),
            sequences_r2[0].len()
        );
    }
    sequences_r2[0].len() * number as usize
}

pub fn flatten_button_sequence(list_of_button_sequences: &[ButtonSequence]) -> Vec<Vec<Button>> {
    let mut flattened: Vec<Vec<Button>> = vec![Vec::new()];
    for button_sequence in list_of_button_sequences {
        match button_sequence {
            ButtonSequence::Single(vec) => {
                for sequence in flattened.iter_mut() {
                    sequence.extend(vec.clone());
                }
            }
            ButtonSequence::Multi(vec_list) => {
                let mut new_flattened: Vec<Vec<Button>> = Vec::new();
                for existing_vec in &flattened {
                    for vec in vec_list {
                        let mut new_vec = existing_vec.clone();
                        new_vec.extend(vec.clone());
                        new_flattened.push(new_vec);
                    }
                }
                flattened = new_flattened;
            }
        }
    }
    flattened
}

pub fn attempt_something_else() {
    let mut numpad_buttons = vec![Numpad::Activate];
    numpad_buttons.push(Numpad::Zero);
    numpad_buttons.push(Numpad::Two);
    numpad_buttons.push(Numpad::Nine);
    numpad_buttons.push(Numpad::Activate);
    let mut len_of_sequence_r0 = 0;
    let mut len_of_sequence_r1 = 0;
    for (&prev, &next) in numpad_buttons.iter().zip(numpad_buttons.iter().skip(1)) {
        println!("- Handling {:?} to {:?}", prev, next);
        let directions_list = get_directions_numpad(prev, next);
        println!(
            "- direction list from {:?} to {:?} for r0: {:?}",
            prev, next, directions_list
        );
        len_of_sequence_r0 += directions_list[0].len() + 1;
        let mut shortest_r1 = usize::MAX;
        for &directions in directions_list {
            println!("-- Handling direction {:?}", directions);
            let mut r0_sequence = vec![Button::Activate];
            r0_sequence.extend(directions.to_vec());
            r0_sequence.push(Button::Activate);
            let mut sequence_len_r1 = 0;
            for (&r0_first, &r0_second) in r0_sequence.iter().zip(r0_sequence.iter().skip(1)) {
                if r0_first == r0_second {
                    sequence_len_r1 += 1;
                    continue;
                }
                let directions_list_r1 = get_directions_dirpad(r0_first, r0_second);
                println!(
                    "-- directions from {:?} to {:?}: {:?} for number {:?} to {:?}",
                    r0_first, r0_second, directions_list_r1, prev, next
                );
                sequence_len_r1 += directions_list_r1[0].len() + 1;
                for &directions in directions_list_r1 {
                    let mut r0_sequence = vec![Button::Activate];
                    r0_sequence.extend(directions.to_vec());
                    r0_sequence.push(Button::Activate);
                    for (&first_r1, &second_r1) in
                        r0_sequence.iter().zip(r0_sequence.iter().skip(1))
                    {
                        if first_r1 == second_r1 {
                            continue;
                        }
                        //let directions_list_r2 = get_directions_dirpad(first_r1, second_r1);
                        //for &directions in directions_list_r2 {}
                    }
                }
            }
            println!(
                "-- sequence len r1 for sequence {:?}: {}",
                r0_sequence, sequence_len_r1
            );
            if sequence_len_r1 < shortest_r1 {
                shortest_r1 = sequence_len_r1;
            }
        }
        println!("- shortest sequence for r1: {}", shortest_r1);
        len_of_sequence_r1 += shortest_r1;
    }
    println!("length of sequence for robot 0: {}", len_of_sequence_r0);
    println!("length of sequence for robot 1: {}", len_of_sequence_r1);
}

pub fn attempt_building() {
    let numpad_buttons = "A029A";
    let mut list_of_sequences: Vec<ButtonSequence> = Vec::new();
    for (prev_char, next_char) in numpad_buttons.chars().zip(numpad_buttons.chars().skip(1)) {
        let prev = Numpad::try_from(prev_char).unwrap();
        let next = Numpad::try_from(next_char).unwrap();
        let directions = get_directions_numpad(prev, next);
        if directions.len() == 1 {
            let mut directions = directions[0].to_vec();
            directions.push(Button::Activate);
            list_of_sequences.push(ButtonSequence::Single(directions));
        } else {
            let mut mulitple_sequences = Vec::new();
            for direction in directions.iter() {
                let mut next_direction = direction.to_vec();
                next_direction.push(Button::Activate);
                mulitple_sequences.push(next_direction);
            }
            list_of_sequences.push(ButtonSequence::Multi(mulitple_sequences));
        }
    }
    println!("{:?}", list_of_sequences);

    let mut button_press_sequences = Vec::new();
    let check_next_step = |start: Button, target: Button| -> ButtonSequence {
        println!("creating sequence from {:?} to {:?}", start, target);
        let sequence = if start == target {
            // There is no shorter way.
            ButtonSequence::Single(vec![Button::Activate])
        } else {
            let directions = get_directions_dirpad(start, target);
            if directions.len() == 1 {
                let mut direction = directions[0].to_vec();
                direction.push(Button::Activate);
                ButtonSequence::Single(direction)
            } else {
                let mut list_of_directions = Vec::new();
                for direction in directions {
                    let mut direction_as_vec = direction.to_vec();
                    direction_as_vec.push(Button::Activate);
                    list_of_directions.push(direction_as_vec);
                }
                ButtonSequence::Multi(list_of_directions)
            }
        };
        println!("sequence: {:?}", sequence);
        sequence
    };
    for sequence in list_of_sequences.iter() {
        match sequence {
            ButtonSequence::Single(vec) => {
                let first = *vec.first().unwrap();
                // Consecutive A presses are never required.
                button_press_sequences.push(vec![check_next_step(Button::Activate, first)]);
                for (first, second) in vec.iter().zip(vec.iter().skip(1)) {
                    button_press_sequences.push(vec![check_next_step(*first, *second)]);
                }
            }
            ButtonSequence::Multi(vec_list) => {
                // Need to calculate the cumulative button presses by accumulating the length
                for vec in vec_list {
                    let mut button_presses = Vec::new();
                    let mut sum_len = 0;
                    button_presses.push(check_next_step(Button::Activate, *vec.first().unwrap()));
                    for (first, second) in vec.iter().zip(vec.iter().skip(1)) {
                        button_presses.push(check_next_step(*first, *second));
                    }
                    for button_press in button_presses.iter() {
                        sum_len += button_press.len();
                    }
                    println!(
                        "button sequence for {:?} in multi sequence {:?}: {:?} with cumulative cost {}",
                        vec, vec_list, button_presses, sum_len
                    );
                    // There can still be multiple button press sequences with the same cumulative
                    // length... so all of them need to be stored?
                    button_press_sequences.push(button_presses);
                }
            }
        }
    }
    println!("button press sequences: {:?}", button_press_sequences);
}
