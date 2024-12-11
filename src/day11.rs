use std::{fs, hash::BuildHasherDefault};

use cached::proc_macro::cached;
use fxhash::FxHashMap;

pub fn solutions() {
    let input = get_input();
    println!("Day 11, #1: {}", solve_first(input.clone()));
    println!("Day 11, #2: {}", solve_second(input.clone()));
}

pub fn get_input() -> Vec<u64> {
    fs::read_to_string("inputs/day11.txt").expect("No file there").split_whitespace().map(|s| s.parse().unwrap()).collect()
}

#[cached(
    ty = "FxHashMap<u64, (bool, u64, u64)>",
    create = "{ FxHashMap::with_capacity_and_hasher(1000, BuildHasherDefault::default()) }",
)]
fn is_splittable(num: u64) -> (bool, u64, u64) {
    let mut i = 0;
    let mut t = 1;
    let mut t_half = 1;

    while t <= num {
        i += 1;
        t *= 10;
        if i & 1 == 0 {
            t_half *= 10;
        }
    }

    if i & 1 == 1 {
        return (false, num, 0);
    }

    let right = num % t_half;
    let left = (num - right) / t_half;
    (true, left, right)
}

#[cached(
    ty = "FxHashMap<(u64, u8), u64>",
    create = "{ FxHashMap::with_capacity_and_hasher(100000, BuildHasherDefault::default()) }",
)]
fn mutations(num: u64, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    if num == 0 {
        return mutations(1, depth - 1);
    }

    let (is, left, right) = is_splittable(num);

    if is {
        return mutations(left, depth - 1) + mutations(right, depth - 1);
    }

    mutations(num * 2024, depth - 1)
}

pub fn solve_first(input: Vec<u64>) -> u64 {
    input.iter()
        .map(|num| mutations(*num, 25))
        .sum()
}

pub fn solve_second(input: Vec<u64>) -> u64 {
    input.iter()
        .map(|num| mutations(*num, 75))
        .sum()
}