use std::fs;

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::solutions;

solutions!{2024, 11}

fn get_input(file: &str) -> Vec<u64> {
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

const CACHE_TRESHOLD: u8 = 4;

fn mutations(num: u64, depth: u8, cache: &mut FxHashMap<(u64, u8), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }

    if num == 0 {
        return mutations(1, depth - 1, cache);
    }

    let (is, left, right) = is_splittable(num);

    if is {
        if depth > CACHE_TRESHOLD {
            if let Some(value) = cache.get(&(num, depth)) {
                return *value;
            }
        }

        let value = mutations(left, depth - 1, cache) + mutations(right, depth - 1, cache);
        if depth > CACHE_TRESHOLD {
            cache.insert((num, depth), value);
        }
        return value;
    }

    mutations(num * 2024, depth - 1, cache)
}

fn solve_first(input: &[u64]) -> u64 {
    let mut cache = FxHashMap::with_capacity_and_hasher(4096, FxBuildHasher);
    input.iter()
        .map(|num| mutations(*num, 25, &mut cache))
        .sum()
}

fn solve_second(input: &[u64]) -> u64 {
    let mut cache = FxHashMap::with_capacity_and_hasher(4096, FxBuildHasher);
    input.iter()
        .map(|num| mutations(*num, 75, &mut cache))
        .sum()
}