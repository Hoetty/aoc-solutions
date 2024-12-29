use std::fs;

use rustc_hash::FxHashMap;

use crate::solutions;

solutions!{2024, 11}

fn get_input(file: &str) -> FxHashMap<u64, usize> {
    let input: Vec<u64> = fs::read_to_string(file).expect("No file there").split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut stones = stones_to_map(&input);
    mutate_n_times(&mut stones, 25);

    stones
}

fn is_splittable(num: u64) -> Option<(u64, u64)> {
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
        return None;
    }

    let right = num % t_half;
    let left = (num - right) / t_half;
    Some((left, right))
}

enum Mutation {
    One(u64),
    Two(u64, u64)
}

fn mutate(num: u64) -> Mutation {
    if num == 0 {
        Mutation::One(1)
    } else if let Some((left, right)) = is_splittable(num) {
        Mutation::Two(left, right)
    } else {
        Mutation::One(num * 2024)
    }
}

fn mutate_all(stones: &mut FxHashMap<u64, usize>) {
    let last_stones: Vec<_> = stones.drain().collect();

    for (stone, count) in last_stones {
        match mutate(stone) {
            Mutation::One(new_stone) => *stones.entry(new_stone).or_default() += count,
            Mutation::Two(first_stone, second_stone) => {
                *stones.entry(first_stone).or_default() += count;
                *stones.entry(second_stone).or_default() += count;
            },
        }
    }
}

fn stones_to_map(stones: &[u64]) -> FxHashMap<u64, usize> {
    let mut blink_stones: FxHashMap<u64, usize> = FxHashMap::default();

    for stone in stones {
        *blink_stones.entry(*stone).or_default() += 1;
    }

    blink_stones
}

fn mutate_n_times(stones: &mut FxHashMap<u64, usize>, n: usize) {
    for _ in 0..n {
        mutate_all(stones);
    }
}

fn solve_first(input: &FxHashMap<u64, usize>) -> usize {
    input.values().sum()
}

fn solve_second(input: &FxHashMap<u64, usize>) -> usize {
    let mut stones = input.clone();
    mutate_n_times(&mut stones, 50);
    stones.values().sum()
}