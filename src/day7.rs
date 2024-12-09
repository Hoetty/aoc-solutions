use std::fs;

pub fn solutions() {
    let input = get_input();
    println!("Day 7, #1: {}", solve_first(input.clone()));
    println!("Day 7, #2: {}", solve_second(input.clone()));
}

pub fn get_input() -> Vec<(u64, Vec<u64>)> {
    fs::read_to_string("inputs/day7.txt")
        .expect("No file there")
        .lines()
        .map(|l| l.split_once(": ").expect("No divider"))
        .map(|(l, r)| (
            l.parse()
                .expect("Left is not a number"), 
            r.split_whitespace()
                .map(|i| i.parse().expect("Operand ist not a number")).collect()
        )).collect()
}

pub fn concat(lhs: u64, rhs: u64) -> u64 {
    let mut lhs = lhs * 10;
    let mut t = 10;

    while t <= rhs {
        t *= 10;
        lhs *= 10;
    }

    lhs + rhs
}

pub fn is_possible(target: u64, carry: u64, rest: &[u64]) -> bool {
    match rest {
        _ if carry > target => false,
        [] => target == carry,
        [x, xs @ ..] => is_possible(target, carry + x, xs) || is_possible(target, carry * x, xs)
    }
}

pub fn solve_first(input: Vec<(u64, Vec<u64>)>) -> u64 {
    input.iter()
        .filter(|(target, operands)| is_possible(*target, *operands.first().unwrap(), &operands[1..operands.len()]))
        .map(|(target, _)| target)
        .sum()
}

pub fn is_possible_with_concat(target: u64, carry: u64, rest: &[u64]) -> bool {
    match rest {
        _ if carry > target => false,
        [] => target == carry,
        [x, xs @ ..] => is_possible_with_concat(target, carry + x, xs) || is_possible_with_concat(target, carry * x, xs) || is_possible_with_concat(target, concat(carry, *x), xs)
    }
}

pub fn solve_second(input: Vec<(u64, Vec<u64>)>) -> u64 {
    input.iter()
        .filter(|(target, operands)| is_possible_with_concat(*target, *operands.first().unwrap(), &operands[1..operands.len()]))
        .map(|(target, _)| target)
        .sum()
}