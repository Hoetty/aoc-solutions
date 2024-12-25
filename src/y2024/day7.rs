use std::fs;
use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 7}

fn get_input(file: &str) -> Vec<(i64, Vec<i64>)> {
    fs::read_to_string(file)
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
fn is_concattable(target: i64, operand: i64) -> (bool, i64) {
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
fn is_addable(target: i64, operand: i64) -> (bool, i64) {
    let minus = target - operand;
    (minus >= 0, minus)
}

#[inline]
fn is_multipliable(target: i64, operand: i64) -> (bool, i64) {
    (target >= operand && target % operand == 0, target / operand)
}

fn is_possible(target: i64, rest: &[i64]) -> bool {
    match rest {
        [] => target == 0,
        [x, xs @ ..] => {
            let multipliable = is_multipliable(target, *x);

            if multipliable.0 && is_possible(multipliable.1, xs) {
                return true;
            }

            let addable = is_addable(target, *x);
            addable.0 && is_possible(addable.1, xs)
        }
    }
}

fn solve_first(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter()
        .map(|(target, operands)| (target, operands.iter().rev().copied().collect::<Vec<i64>>()))
        .filter(|(target, operands)| is_possible(**target, &operands[0..operands.len()]))
        .map(|(target, _)| target)
        .sum()
}

fn is_possible_with_concat(target: i64, rest: &[i64]) -> bool {
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
            addable.0 && is_possible_with_concat(addable.1, xs)
        }
    }
}

fn solve_second(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter()
        .map(|(target, operands)| (target, operands.iter().rev().copied().collect::<Vec<i64>>()))
        .filter(|(target, operands)| is_possible_with_concat(**target, &operands[0..operands.len()]))
        .map(|(target, _)| target)
        .sum()
}
