use std::fs;

pub fn solutions() {
    let input = get_input();
    println!("Day 7, #1: {}", solve_first(input.clone()));
    println!("Day 7, #2: {}", solve_second(input.clone()));
}

pub fn get_input() -> Vec<(i64, Vec<i64>)> {
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

pub fn concat(lhs: i64, rhs: i64) -> i64 {
    let r = (rhs as f64).log10().floor() as u32;
    lhs * 10i64.pow(r + 1) + rhs
}

pub fn is_possible(target: i64, carry: i64, rest: &[i64], allow_concat: bool) -> bool {
    match rest {
        _ if carry > target => false,
        [] => target == carry,
        [x, xs @ ..] => is_possible(target, carry + x, xs, allow_concat) || is_possible(target, carry * x, xs, allow_concat) || (allow_concat && is_possible(target, concat(carry, *x), xs, allow_concat))
    }
}

pub fn solve_first(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter()
        .filter(|(target, operands)| is_possible(*target, *operands.first().unwrap(), &operands[1..operands.len()], false))
        .map(|(target, _)| target)
        .sum()
}

pub fn solve_second(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter()
        .filter(|(target, operands)| is_possible(*target, *operands.first().unwrap(), &operands[1..operands.len()], true))
        .map(|(target, _)| target)
        .sum()
}