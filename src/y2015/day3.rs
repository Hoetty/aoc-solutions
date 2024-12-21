use std::{collections::HashSet, fs};

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2015/day3.txt");

    Solution::evaluated(
        "Day 3".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

pub fn get_input(file: &'static str) -> String {
    fs::read_to_string(file).unwrap()
}

pub fn solve_first(input: String) -> usize {
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

pub fn solve_second(input: String) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (usize, usize) {
        let file = fs::read_to_string("test-inputs/2015/day3-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2015/day3.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2015/day3.txt"));
        assert_eq!(result, expected().1);
    }
}