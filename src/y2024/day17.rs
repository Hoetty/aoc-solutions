use std::{collections::VecDeque, fs::{self}};

use crate::solutions;

solutions!{2024, 17}

const REGISTER_SKIP_LEN: usize = "Register _: ".len();
const PROGRAM_SKIP_LEN: usize = "Program: ".len();

fn get_input(file: &str) -> ((u64, u64, u64), Vec<u8>) {
    let file = fs::read_to_string(file).expect("No file there");
    let lines: Vec<&str> = file.lines().collect();

    let a = lines[0][REGISTER_SKIP_LEN..].parse().unwrap();
    let b = lines[1][REGISTER_SKIP_LEN..].parse().unwrap();
    let c = lines[2][REGISTER_SKIP_LEN..].parse().unwrap();

    let instructions = lines[4][PROGRAM_SKIP_LEN..]
        .chars()
        .step_by(2)
        .map(|c| c as u8 - b'0')
        .collect();

    ((a, b, c), instructions)
}

/// Computes the combo value for the given value
/// 0..=3 evaluate to the value itself
/// Then 4, 5 and 6 correspond to the values in register a, b and c
#[inline(always)]
fn combo(value: u8, a: u64, b: u64, c: u64) -> u64 {
    match value {
        0..=3 => value as u64,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid Combo opcode {value}")
    }
}

fn solve_first(input: &((u64, u64, u64), Vec<u8>)) -> String {
    let ((mut a, mut b, mut c), instructions) = input;

    let mut ip = 0;
    let mut output = String::new();

    while ip < instructions.len() {
        let instruction = instructions[ip];
        let opcode = instructions[ip + 1];

        ip += 2;

        match instruction {
            0 => a = a >> combo(opcode, a, b, c),
            1 => b ^= opcode as u64,
            2 => b = combo(opcode, a, b, c) & 0b111,
            3 => if a != 0 {
                ip = opcode as usize
            },
            4 => b ^= c,
            5 => {
                let number = (combo(opcode, a, b, c) & 0b111) as u8;
                output.push((number + b'0') as char);
                output.push(',');
            },
            6 => b = a >> combo(opcode, a, b, c),
            7 => c = a >> combo(opcode, a, b, c),
            _ => panic!("Unknown instruction {instruction}")
        }
    }

    output.pop().unwrap();
    output
}

fn bytes_num(bytes: &[u8]) -> u64 {
    let mut output: u64 = 0;
    for (i, byte) in bytes.iter().enumerate() {
        output += (*byte as u64) << (i * 3);
    }
    output
}

fn solve_second(input: &((u64, u64, u64), Vec<u8>)) -> u64 {
    let ((_, b_initial, c_initial), instructions) = input;
    let target = bytes_num(instructions);

    let mut queue: VecDeque<(u64, u8)> = VecDeque::new();

    let mut found = u64::MAX;

    queue.push_back((0, instructions.len() as u8 - 1));

    while let Some((start_value, position)) = queue.pop_front() {
        if start_value > found {
            continue;
        }

        for i in 0..8 {
            let try_value = start_value | (i << (3 * position));

            let mut a = try_value;
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
                    1 => b ^= opcode as u64,
                    2 => b = combo(opcode, a, b, c) & 0b111,
                    3 => if a != 0 {
                        ip = opcode as usize
                    },
                    4 => b ^= c,
                    5 => {
                        output |= ((combo(opcode, a, b, c) & 0b111) as u64) << (output_len * 3);
                        output_len += 1;
                    },
                    6 => b = a >> combo(opcode, a, b, c),
                    7 => c = a >> combo(opcode, a, b, c),
                    _ => panic!("Unknown instruction {instruction}")
                }
            }

            if output >> (position * 3) == target >> (position * 3) {
                if position == 0 {
                    if try_value < found {
                        found = try_value;
                    }
                } else {
                    queue.push_back((try_value, position - 1));
                }
            }
        }
    }

    found
}
