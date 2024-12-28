use std::fs;
use crate::solutions;

solutions!{2024, 7}

fn get_input(file: &str) -> Vec<(i64, Vec<i64>)> {
    fs::read_to_string(file)
        .expect("No file there")
        .lines()
        .map(|line| line.split_once(": ").expect("No divider"))
        .map(|(target, operands)| (
            target.parse()
                .expect("Target is not a number"), 
            operands.split_whitespace()
                .map(|operand| operand.parse().expect("Operand ist not a number")).rev().collect()
        )).collect()
}

/// Checks if targets ends in operand, like how 123 ends in 23
#[inline]
fn is_concattable(target: i64, end: i64) -> (bool, i64) {
    // When the target is smaller than the end number, the target cannot possibly end in it
    if target <= end {
        return (false, target);
    }

    let mut digits = 10;

    while digits <= end {
        digits *= 10;
    }

    // Remove the end from the target. If the target ends in the number, it should now end in all zeros, 
    //    meaning it is divisble by digits
    let minus = target - end;
    is_multipliable(minus, digits)
}

/// Checks if the target number, could be made by adding the operand to some number and also returns that number
#[inline]
fn is_addable(target: i64, operand: i64) -> (bool, i64) {
    let minus = target - operand;
    (minus >= 0, minus)
}

/// Checks if the target number could be the product of the operand and another number and also returns that number
#[inline]
fn is_multipliable(target: i64, operand: i64) -> (bool, i64) {
    (target >= operand && target % operand == 0, target / operand)
}

/// Recursively tests if the target can be calculated from the operands. 
/// The test is performed back to front
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

/// ### Calculation Calibration
/// 
/// For each target number, test if it could be the result of a calculation 
///     involving all operands and the operators + and *
fn solve_first(input: &[(i64, Vec<i64>)]) -> i64 {
    input.iter()
        .map(|(target, operands)| (target, operands))
        .filter(|(target, operands)| is_possible(**target, &operands[0..operands.len()]))
        .map(|(target, _)| target)
        .sum()
}

/// Like is_possible but has a third operator: concat
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

/// ### Calculation Calibration with Concat
/// 
/// For each target number, test if it could be the result of a calculation 
///     involving all operands and the operators +, * and concat
fn solve_second(input: &[(i64, Vec<i64>)]) -> i64 {
    input.iter()
        .map(|(target, operands)| (target, operands))
        .filter(|(target, operands)| is_possible_with_concat(**target, &operands[0..operands.len()]))
        .map(|(target, _)| target)
        .sum()
}
