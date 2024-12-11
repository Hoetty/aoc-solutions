use std::fs;

pub fn solutions() {
    let input = get_input("inputs/2015/day6.txt");
    println!("2015 Day 6 #1: {}", solve_first(input.clone()));
    println!("2015 Day 6 #2: {}", solve_second(input.clone()));
}

pub fn get_input(file: &'static str) -> Vec<String> {
    fs::read_to_string(file).unwrap().lines().map(|s| s.to_string()).collect()
}

pub fn solve_first(input: Vec<String>) -> usize {
    0
}


pub fn solve_second(input: Vec<String>) -> usize {
    0
}