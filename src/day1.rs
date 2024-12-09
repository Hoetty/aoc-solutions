use std::{collections::HashMap, fs};

type List = (Vec<i32>, Vec<i32>);

pub fn solutions() {
    let input = get_input();
    println!("Day 1, #1: {}", solve_first(input.clone()));
    println!("Day 1, #2: {}", solve_second(input.clone()));
}

pub fn get_input() -> List {

    let content = fs::read_to_string("inputs/day1.txt").expect("No file there");

    let (first, second): List = content
        .lines()
        .map(|line| line.split_once("   ").expect("Must be splittable"))
        .map(|(s1, s2)| (s1.parse::<i32>().expect("Must be number"), s2.parse::<i32>().expect("Must be a number")))
        .unzip();

    (first, second)
}

pub fn solve_first(input: List) -> i32 {

    let (mut first, mut second) = input;

    first.sort();
    second.sort();

    first.iter()
        .zip(second.iter())
        .map(|(first, second)| (first - second).abs())
        .sum()
}

pub fn solve_second(input: List) -> i32 {
    let (first, second) = input;

    let mut frequency: HashMap<i32, i32> = HashMap::new();

    for num in second {
        match frequency.get_mut(&num) {
            Some(count) => { *count = *count + 1; },
            None => { frequency.insert(num, 1); }
        }
    }

    first.iter()
        .map(|i| i * frequency.get(i).unwrap_or(&0))
        .sum()
}