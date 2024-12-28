use std::{cmp::min, fs};

use crate::solutions;

solutions!{2015, 2}

pub fn get_input(file: &str) -> Vec<(i32, i32, i32)> {
    fs::read_to_string(file).unwrap().lines().map(|l| l.split_terminator('x').map(|i| i.parse().unwrap()).collect::<Vec<i32>>()).map(|v| (v[0], v[1], v[2])).collect()
}

pub fn solve_first(input: &Vec<(i32, i32, i32)>) -> i32 {
    let mut sum = 0;

    for present in input {
        let first = present.0 * present.1;
        let second = present.0 * present.2;
        let third = present.1 * present.2;

        sum += 2 * first + 2 * second + 2 * third + min(min(first, second), third);
    }

    sum
}

pub fn solve_second(input: &Vec<(i32, i32, i32)>) -> i32 {
    let mut sum = 0;

    for present in input {
        let mut ordered = [present.0, present.1, present.2];
        ordered.sort();

        sum += present.0 * present.1 * present.2 + 2 * ordered[0] + 2 * ordered[1];
    }

    sum
}
