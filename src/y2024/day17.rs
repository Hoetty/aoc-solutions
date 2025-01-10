//! Design Choice: The instructions are saved in a u64 even if they can only be between 0 and 8
//! However, using u64 over u8 results in 32% better performace
use std::{collections::VecDeque, fs::{self}};
use crate::solutions;

solutions!{2024, 17}

const REGISTER_SKIP_LEN: usize = "Register _: ".len();
const PROGRAM_SKIP_LEN: usize = "Program: ".len();

fn get_input(file: &str) -> ((u64, u64, u64), Vec<u64>) {
    let file = fs::read_to_string(file).expect("No file there");
    let lines: Vec<&str> = file.lines().collect();

    let a = lines[0][REGISTER_SKIP_LEN..].parse().unwrap();
    let b = lines[1][REGISTER_SKIP_LEN..].parse().unwrap();
    let c = lines[2][REGISTER_SKIP_LEN..].parse().unwrap();

    let instructions = lines[4][PROGRAM_SKIP_LEN..]
        .chars()
        .step_by(2)
        .map(|c| c as u64 - b'0' as u64)
        .collect();

    ((a, b, c), instructions)
}

/// Computes the combo value for the given value
/// 0..=3 evaluate to the value itself
/// Then 4, 5 and 6 correspond to the values in register a, b and c
#[inline(always)]
fn combo(value: u64, a: u64, b: u64, c: u64) -> u64 {
    match value {
        0..=3 => value,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid Combo opcode {value}")
    }
}

/// ### Program Evaluation
/// 
/// Runs through the entire program and returns the output
/// The specs of the "virtual machine" can be found [here](https://adventofcode.com/2024/day/17)
fn solve_first(input: &((u64, u64, u64), Vec<u64>)) -> String {
    let ((mut a, mut b, mut c), instructions) = input;

    // Ip is the Instruction Pointer and points to the next instruction to be executed
    let mut ip = 0;
    let mut output = String::new();

    while ip < instructions.len() {
        let instruction = instructions[ip];
        let opcode = instructions[ip + 1];

        ip += 2;

        // Execute the instruction according to the specs
        match instruction {
            0 => a = a >> combo(opcode, a, b, c),
            1 => b ^= opcode,
            2 => b = combo(opcode, a, b, c) & 0b111,
            3 => if a != 0 {
                ip = opcode as usize
            },
            4 => b ^= c,
            5 => {
                let number = combo(opcode, a, b, c) & 0b111;
                output.push((number as u8 + b'0') as char);
                output.push(',');
            },
            6 => b = a >> combo(opcode, a, b, c),
            7 => c = a >> combo(opcode, a, b, c),
            _ => panic!("Unknown instruction {instruction}")
        }
    }

    // Remove the last comma from the output
    output.pop().unwrap();
    output
}

/// Converts a set of instructions to a number
fn bytes_num(bytes: &[u64]) -> u64 {
    let mut output: u64 = 0;
    for (i, byte) in bytes.iter().enumerate() {
        output += *byte << (i * 3);
    }
    output
}

/// ### Replicating Program
/// 
/// Searches for the input value for register a, such that the program outputs itself
/// A 3 bit set in the input only influences the same offset and lower of the output
/// So we reconstruct the input 3 bits at a time, from highest to lowest
fn solve_second(input: &((u64, u64, u64), Vec<u64>)) -> u64 {
    let ((_, b_initial, c_initial), instructions) = input;
    let target = bytes_num(instructions);

    let mut queue: VecDeque<(u64, u64)> = VecDeque::new();

    let mut register_a_seed = u64::MAX;

    queue.push_back((0, instructions.len() as u64 - 1));

    while !queue.is_empty() {
        let (start_seed, position) = queue.pop_front().unwrap();

        if start_seed > register_a_seed {
            continue;
        }

        // We check all 3 bit combinations
        for i in 0..8 {
            let test_seed = start_seed | (i << (3 * position));

            let mut a = test_seed;
            let mut b = *b_initial;
            let mut c = *c_initial;

            let mut output = 0;
            let mut output_len = 0;

            let mut ip = 0;

            while ip < instructions.len() {
                let instruction = instructions[ip];
                let opcode = instructions[ip + 1];

                ip += 2;

                match instruction {
                    0 => a = a >> combo(opcode, a, b, c),
                    1 => b ^= opcode,
                    2 => b = combo(opcode, a, b, c) & 0b111,
                    3 => if a != 0 {
                        ip = opcode as usize
                    },
                    4 => b ^= c,
                    5 => {
                        // We save the output in a number for efficient comparision
                        output |= (combo(opcode, a, b, c) & 0b111) << (output_len * 3);
                        output_len += 1;
                    },
                    6 => b = a >> combo(opcode, a, b, c),
                    7 => c = a >> combo(opcode, a, b, c),
                    _ => panic!("Unknown instruction {instruction}")
                }
            }

            // The higher bits are already equal, so we can just compare the rest of the number without masking
            if output >> (position * 3) == target >> (position * 3) {
                if position == 0 {
                    if test_seed < register_a_seed {
                        register_a_seed = test_seed;
                    }
                } else {
                    queue.push_back((test_seed, position - 1));
                }
            }
        }
    }

    register_a_seed
}
