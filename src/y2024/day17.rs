use std::{collections::VecDeque, fs::{self}, u64};


pub fn solutions() {
    let input = get_input("inputs/2024/day17.txt",);
    println!("2024 Day 17 #1: {}", solve_first(input.clone()));
    println!("2024 Day 17 #2: {}", solve_second(input));
}


fn get_input(file: &'static str) -> ((u64, u64, u64), Vec<u8>) {
    let file = fs::read_to_string(file).expect("No file there");
    let lines: Vec<&str> = file.lines().collect();

    let a = lines[0]["Register _: ".len()..].parse().unwrap();
    let b = lines[1]["Register _: ".len()..].parse().unwrap();
    let c = lines[2]["Register _: ".len()..].parse().unwrap();

    let instructions = lines[4]["Program: ".len()..].replace(",", "").chars().map(|c| c as u8 - 48).collect();

    ((a, b, c), instructions)
}

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

fn solve_first(input: ((u64, u64, u64), Vec<u8>)) -> String {
    let ((mut a, mut b, mut c), instructions) = input;

    let mut output: Vec<u8> = Vec::new();

    let mut ip = 0;
    loop {
        if ip >= instructions.len() {
            break;
        }

        let current_instruction = instructions[ip];
        let opcode = instructions[ip + 1];

        ip += 2;

        match current_instruction {
            0 => a = a >> combo(opcode, a, b, c),
            1 => b = b ^ opcode as u64,
            2 => b = combo(opcode, a, b, c) & 0b111,
            3 => if a != 0 {
                ip = opcode as usize
            },
            4 => b = b ^ c,
            5 => output.push((combo(opcode, a, b, c) & 0b111) as u8),
            6 => b = a >> combo(opcode, a, b, c),
            7 => c = a >> combo(opcode, a, b, c),
            _ => panic!("Unknown instruction {current_instruction}")
        }
    }

    let mut result = String::new();

    result.push((output[0] + 48) as char);

    for num in &output[1..] {
        result.push(',');
        result.push((*num + 48) as char);
    }

    result
}

fn bytes_num(nums: &Vec<u8>) -> u64 {
    let mut carry: u64 = 0;

    for (i, num) in nums.iter().enumerate() {
        carry += (*num as u64) << (i * 3);
    }

    carry
}

fn solve_second(input: ((u64, u64, u64), Vec<u8>)) -> u64 {
    let ((_ao, bo, co), instructions) = input;

    let target = bytes_num(&instructions);

    let mut queue: VecDeque<(u64, u8)> = VecDeque::new();

    let mut found = u64::MAX;

    queue.push_back((0, instructions.len() as u8 - 1));

    while !queue.is_empty() {
        let (start_value, position) = queue.pop_front().unwrap();

        if start_value > found {
            continue;
        }

        for i in 0..8 {

            let try_value = start_value | (i << (3 * position));

            let mut a = try_value;
            let mut b = bo;
            let mut c = co;
    
            let mut output: Vec<u8> = Vec::new();
        
            let mut ip = 0;
            loop {
                if ip >= instructions.len() {
                    break;
                }
        
                let current_instruction = instructions[ip];
                let opcode = instructions[ip + 1];
        
                ip += 2;
        
                match current_instruction {
                    0 => a = a >> combo(opcode, a, b, c),
                    1 => b = b ^ opcode as u64,
                    2 => b = combo(opcode, a, b, c) & 0b111,
                    3 => if a != 0 {
                        ip = opcode as usize
                    },
                    4 => b = b ^ c,
                    5 => output.push((combo(opcode, a, b, c) & 0b111) as u8),
                    6 => b = a >> combo(opcode, a, b, c),
                    7 => c = a >> combo(opcode, a, b, c),
                    _ => panic!("Unknown instruction {current_instruction}")
                }
            }
    
            let wrong = bytes_num(&output) ^ target;
    
            if wrong & (7 << (position * 3)) == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (String, u64) {
        let file = fs::read_to_string("test-inputs/2024/day17-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day17.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day17.txt"));
        assert_eq!(result, expected().1);
    }
}