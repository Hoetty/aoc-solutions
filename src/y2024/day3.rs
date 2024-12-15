use std::fs;

use regex::Regex;

pub fn solutions() {
    let input = get_input("inputs/2024/day3.txt");
    println!("2024 Day 3 #1: {}", solve_first(input.clone()));
    println!("2024 Day 3 #2: {}", solve_second(input));
}

fn get_input(file: &'static str) -> String {
    return fs::read_to_string(file).expect("No file there");
}

fn solve_first(input: String) -> i32 {
    let regex = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();

    let mut sum: i32 = 0;

    for (_, [first, second]) in regex.captures_iter(&input).map(|c| c.extract()) {
        sum += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }

    sum
}

fn solve_second(input: String) -> i32 {
    let regex = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();

    let mut sum: i32 = 0;

    for (m, (_, [first, second])) in regex.captures_iter(&input).map(|c| (c.get(0).unwrap(), c.extract())) {

        let substr = &input[0..m.start()];

        let do_i = substr.rfind("do()");
        let dont_i = substr.rfind("don't()");

        if dont_i.is_none() || (do_i.is_some_and(|do_index| do_index > dont_i.unwrap())) {
            sum += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (i32, i32) {
        let file = fs::read_to_string("test-inputs/2024/day3-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day3.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day3.txt"));
        assert_eq!(result, expected().1);
    }
}