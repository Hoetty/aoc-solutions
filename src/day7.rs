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

#[inline]
pub fn is_concattable(target: i64, operand: i64) -> (bool, i64) {
    if target <= operand {
        return (false, target);
    }

    let mut t = 10;

    while t <= operand {
        t *= 10;
    }

    let minus = target - operand;
    is_multipliable(minus, t)
}

#[inline]
pub fn is_addable(target: i64, operand: i64) -> (bool, i64) {
    let minus = target - operand;
    (minus >= 0, minus)
}

#[inline]
pub fn is_multipliable(target: i64, operand: i64) -> (bool, i64) {
    (target >= operand && target % operand == 0, target / operand)
}

pub fn is_possible(target: i64, rest: &[i64]) -> bool {
    match rest {
        [] => target == 0,
        [x, xs @ ..] => {
            let multipliable = is_multipliable(target, *x);

            if multipliable.0 && is_possible(multipliable.1, xs) {
                return true;
            }

            let addable = is_addable(target, *x);
            return addable.0 && is_possible(addable.1, xs);
        }
    }
}

pub fn solve_first(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter()
        .map(|(target, operands)| (target, operands.iter().rev().map(|v| *v).collect::<Vec<i64>>()))
        .filter(|(target, operands)| is_possible(**target, &operands[0..operands.len()]))
        .map(|(target, _)| target)
        .sum()
}

pub fn is_possible_with_concat(target: i64, rest: &[i64]) -> bool {
    match rest {
        [] => target == 0,
        [x, xs @ ..] => {
            let concattable = is_concattable(target, *x);

            if concattable.0 && is_possible_with_concat(concattable.1, xs) {
                return true;
            }

            let multipliable = is_multipliable(target, *x);

            if multipliable.0 && is_possible_with_concat(multipliable.1, xs) {
                return true;
            }

            let addable = is_addable(target, *x);
            return addable.0 && is_possible_with_concat(addable.1, xs);
        }
    }
}

pub fn solve_second(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter()
        .map(|(target, operands)| (target, operands.iter().rev().map(|v| *v).collect::<Vec<i64>>()))
        .filter(|(target, operands)| is_possible_with_concat(**target, &operands[0..operands.len()]))
        .map(|(target, _)| target)
        .sum()
}