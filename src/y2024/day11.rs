use std::fs;

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::solutions;

solutions!{2024, 11}

fn get_input(file: &str) -> FxHashMap<u64, usize> {
    let input: Vec<u64> = fs::read_to_string(file).expect("No file there").split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Precalculate 25 Mutations to use later
    let stones = stones_to_map(&input);
    mutate_n_times::<25>(&stones)
}

/// Tests if a number has an even amount of digits, and splits it into two
/// 123 -> None
/// 1234 -> Some((12, 34))
fn is_splittable(num: u64) -> Option<(u64, u64)> {
    let mut is_odd = 0;
    let mut ten = 1;
    let mut ten_half = 1;

    while ten <= num {
        is_odd += 1;
        ten *= 10;
        if is_odd & 1 == 0 {
            ten_half *= 10;
        }
    }

    if is_odd & 1 == 1 {
        return None;
    }

    let right = num % ten_half;
    let left = (num - right) / ten_half;
    Some((left, right))
}

/// Stores the result of what a stone becomes after blinking
enum Mutation {
    One(u64),
    Two(u64, u64)
}

/// Mutate a stone once
/// If its 0 then it becomes 1
/// Else if its splittable, left and right are returned
/// Else the number is multiplied by 2024
fn mutate(stone: u64) -> Mutation {
    if stone == 0 {
        Mutation::One(1)
    } else if let Some((left, right)) = is_splittable(stone) {
        Mutation::Two(left, right)
    } else {
        Mutation::One(stone * 2024)
    }
}

/// Mutates all stones in the HashMap. And returns a new HashMap with the mutated stone counts
fn mutate_all(stones: &FxHashMap<u64, usize>) -> FxHashMap<u64, usize> {
    let mut next_stones = FxHashMap::with_capacity_and_hasher(stones.len() + stones.len() / 2, FxBuildHasher);

    for (stone, count) in stones {
        match mutate(*stone) {
            Mutation::One(new_stone) => *next_stones.entry(new_stone).or_default() += count,
            Mutation::Two(first_stone, second_stone) => {
                *next_stones.entry(first_stone).or_default() += count;
                *next_stones.entry(second_stone).or_default() += count;
            },
        }
    }

    next_stones
}

/// Converts a list of stones into a count map, 
///    where the key is the stones number and the value how often it appeared in the list
fn stones_to_map(stones: &[u64]) -> FxHashMap<u64, usize> {
    let mut stone_counts: FxHashMap<u64, usize> = FxHashMap::default();

    for stone in stones {
        *stone_counts.entry(*stone).or_default() += 1;
    }

    stone_counts
}

/// Mutates the stones n times and returns the new HashMap
fn mutate_n_times<const N: usize>(stones: &FxHashMap<u64, usize>) -> FxHashMap<u64, usize> {
    let mut new_stones = mutate_all(stones);
    for _ in 0..N - 1 {
        new_stones = mutate_all(&new_stones);
    }

    new_stones
}

/// ### 25 Blinks
/// 
/// Count the number of stones after 25 blinks
fn solve_first(input: &FxHashMap<u64, usize>) -> usize {
    input.values().sum()
}

/// ### 75 Blinks
/// 
/// Count the number of stones after 75 blinks
fn solve_second(input: &FxHashMap<u64, usize>) -> usize {
    let stones = mutate_n_times::<50>(input);
    stones.values().sum()
}