use std::fs;

use regex::Regex;

pub fn solutions() {
    let input = get_input();
    println!("Day 3, #1: {}", solve_first(input.clone()));
    println!("Day 3, #2: {}", solve_second(input.clone()));
}

pub fn get_input() -> String {
    return fs::read_to_string("inputs/day3.txt").expect("No file there");
}

pub fn solve_first(input: String) -> i32 {
    let regex = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();

    let mut sum: i32 = 0;

    for (_, [first, second]) in regex.captures_iter(&input).map(|c| c.extract()) {
        sum += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }

    sum
}

pub fn solve_second(input: String) -> i32 {
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