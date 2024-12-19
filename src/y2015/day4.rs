use std::fs;

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2015/day4.txt");

    Solution::evaluated(
        "Day 4".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

pub fn get_input(file: &'static str) -> String {
    fs::read_to_string(file).unwrap()
}

pub fn solve_first(input: String) -> usize {
    let mut i = 0;

    loop {
        let test = format!("{}{}", input, i);
        let hash = md5::compute(test);

        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0b11110000) == 0 {
            return i;  
        }

        i += 1;
    }
}

pub fn solve_second(input: String) -> usize {
    let mut i = 0;

    loop {
        let test = format!("{}{}", input, i);
        let hash = md5::compute(test);

        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i;  
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (usize, usize) {
        let file = fs::read_to_string("test-inputs/2015/day4-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2015/day4.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2015/day4.txt"));
        assert_eq!(result, expected().1);
    }
}