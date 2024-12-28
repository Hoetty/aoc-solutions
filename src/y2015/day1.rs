use std::fs;

use crate::formatting::Solution;
use crate::solutions;

solutions!{2015, 1}

pub fn get_input(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

pub fn solve_first(input: &str) -> i32 {
    let down = input.chars().filter(|c| *c == ')').count();
    let up = input.chars().count() - down;

    (up - down) as i32
}

pub fn solve_second(input: &str) -> i32 {
    let mut current = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => current += 1,
            ')' => current -= 1,
            _ => panic!()
        }

        if current == -1 {
            return i as i32 + 1;
        }
    }

    panic!();
}
