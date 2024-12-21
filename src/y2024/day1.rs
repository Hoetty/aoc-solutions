use std::{collections::HashMap, fs};

use crate::Solution;

type List = (Vec<i32>, Vec<i32>);

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day1.txt");

    Solution::evaluated(
        "Day 1".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

fn get_input(file: &'static str) -> List {
    let content = fs::read_to_string(file).expect("No file there");

    let (first, second): List = content
        .lines()
        .map(|line| line.split_once("   ").expect("Must be splittable"))
        .map(|(s1, s2)| (s1.parse::<i32>().expect("Must be number"), s2.parse::<i32>().expect("Must be a number")))
        .unzip();

    (first, second)
}

fn solve_first(input: List) -> i32 {

    let (mut first, mut second) = input;

    first.sort();
    second.sort();

    first.iter()
        .zip(second.iter())
        .map(|(first, second)| (first - second).abs())
        .sum()
}

fn solve_second(input: List) -> i32 {
    let (first, second) = input;

    let mut frequency: HashMap<i32, i32> = HashMap::new();

    for num in second {
        match frequency.get_mut(&num) {
            Some(count) => { *count += 1; },
            None => { frequency.insert(num, 1); }
        }
    }

    first.iter()
        .map(|i| i * frequency.get(i).unwrap_or(&0))
        .sum()
}
