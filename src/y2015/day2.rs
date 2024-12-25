use std::{cmp::min, fs};

use crate::formatting::Solution;
use crate::solutions;

solutions!{2015, 2}

pub fn get_input(file: &str) -> Vec<(i32, i32, i32)> {
    fs::read_to_string(file).unwrap().lines().map(|l| l.split_terminator('x').map(|i| i.parse().unwrap()).collect::<Vec<i32>>()).map(|v| (v[0], v[1], v[2])).collect()
}

pub fn solve_first(input: Vec<(i32, i32, i32)>) -> i32 {
    let mut sum = 0;

    for present in input {
        let first = present.0 * present.1;
        let second = present.0 * present.2;
        let third = present.1 * present.2;

        sum += 2 * first + 2 * second + 2 * third + min(min(first, second), third);
    }

    sum
}

pub fn solve_second(input: Vec<(i32, i32, i32)>) -> i32 {
    let mut sum = 0;

    for present in input {
        let mut ordered = [present.0, present.1, present.2];
        ordered.sort();

        sum += present.0 * present.1 * present.2 + 2 * ordered[0] + 2 * ordered[1];
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (i32, i32) {
        let file = fs::read_to_string("test-inputs/2015/day2-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2015/day2.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2015/day2.txt"));
        assert_eq!(result, expected().1);
    }
}