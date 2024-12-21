use std::fs;

use regex::Regex;

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day3.txt");

    Solution::evaluated(
        "Day 3".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

fn get_input(file: &'static str) -> String {
    fs::read_to_string(file).expect("No file there")
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
