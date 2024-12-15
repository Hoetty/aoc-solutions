use std::{fs, hash::BuildHasherDefault};

use cached::proc_macro::cached;
use fxhash::FxHashMap;

pub fn solutions() {
    let input = get_input("inputs/2024/day11.txt");
    println!("2024 Day 11 #1: {}", solve_first(input.clone()));
    println!("2024 Day 11 #2: {}", solve_second(input));
}

fn get_input(file: &'static str) -> Vec<u64> {
    fs::read_to_string(file).expect("No file there").split_whitespace().map(|s| s.parse().unwrap()).collect()
}

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

fn solve_first(input: Vec<u64>) -> u64 {
    input.iter()
        .map(|num| mutations(*num, 25))
        .sum()
}

fn solve_second(input: Vec<u64>) -> u64 {
    input.iter()
        .map(|num| mutations(*num, 75))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (u64, u64) {
        let file = fs::read_to_string("test-inputs/2024/day11-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day11.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day11.txt"));
        assert_eq!(result, expected().1);
    }
}