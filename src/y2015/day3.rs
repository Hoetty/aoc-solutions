use std::{collections::HashSet, fs};

use crate::solutions;

solutions!{2015, 3}

pub fn get_input(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

pub fn solve_first(input: &str) -> usize {
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let mut current = (0, 0);
    for char in input.chars() {
        match char {
            '^' => current = (current.0, current.1 + 1),
            '<' => current = (current.0 - 1, current.1),
            'v' => current = (current.0, current.1 - 1),
            '>' => current = (current.0 + 1, current.1),
            c => panic!("{c} is not defined")
        }

        visited.insert(current);
    }

    visited.len()
}

pub fn solve_second(input: &str) -> usize {
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let mut current = (0, 0);
    let mut current2 = (0, 0);
    let mut iter = input.chars();
    while let Some(char) = iter.next() {

        match char {
            '^' => current = (current.0, current.1 + 1),
            '<' => current = (current.0 - 1, current.1),
            'v' => current = (current.0, current.1 - 1),
            '>' => current = (current.0 + 1, current.1),
            c => panic!("{c} is not defined")
        }

        visited.insert(current);

        let char = match iter.next() {
            Some(c) => c,
            None => break
        };

        match char {
            '^' => current2 = (current2.0, current2.1 + 1),
            '<' => current2 = (current2.0 - 1, current2.1),
            'v' => current2 = (current2.0, current2.1 - 1),
            '>' => current2 = (current2.0 + 1, current2.1),
            c => panic!("{c} is not defined")
        }

        visited.insert(current2);
    }

    visited.len()
}