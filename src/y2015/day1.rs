use std::fs;

use crate::formatting::Solution;
use crate::solutions;

solutions!{2015, 1}

pub fn get_input(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

pub fn solve_first(input: String) -> i32 {
    let down = input.chars().filter(|c| *c == ')').count();
    let up = input.chars().count() - down;

    (up - down) as i32
}

pub fn solve_second(input: String) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (i32, i32) {
        let file = fs::read_to_string("test-inputs/2015/day1-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2015/day1.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2015/day1.txt"));
        assert_eq!(result, expected().1);
    }
}