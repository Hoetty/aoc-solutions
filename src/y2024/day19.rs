use std::fs;

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::solutions;

solutions!{2024, 19}

const WHITE: u8 = 0;
const BLUE: u8 = 1;
const BLACK: u8 = 2;
const RED: u8 = 3;
const GREEN: u8 = 4;

type Towels = [Vec<Vec<u8>>; 5];

fn string_to_num(pattern: &str) -> Vec<u8> {
    pattern.chars().map(|c| match c {
        'w' => WHITE,
        'u' => BLUE,
        'b' => BLACK,
        'r' => RED,
        'g' => GREEN,
        _ => panic!("Unkown char {c}")
    }).collect()
}

fn get_input(file: &str) -> (Towels, Vec<Vec<u8>>) {
    let file = fs::read_to_string(file).expect("No file there");
    let (first, second) = file.split_once("\n\n").unwrap();

    let mut towels: Towels = Default::default();

    for pattern in first.split(", ").map(&string_to_num) {
        towels[pattern[0] as usize].push(pattern);
    }

    let possible_patterns = second.lines().map(&string_to_num).filter(|target| is_possible(target, &towels)).collect();

    (
        towels,
        possible_patterns
    )
}

fn is_possible<'a>(target: &'a [u8], towels: &Towels) -> bool {
    for towel in &towels[target[0] as usize] {
        if towel.len() > target.len() || !target.starts_with(towel) {
            continue;
        }

        if towel.len() == target.len() || is_possible(&target[towel.len()..], towels) {
            return true;
        }
    }

    false
}

fn solve_first(input: &(Towels, Vec<Vec<u8>>)) -> usize {
    input.1.len()
}

fn possibilities<'a>(target: &'a [u8], towels: &Towels, cache: &mut FxHashMap<&'a[u8], u64>) -> u64 {
    if let Some(v) = cache.get(&target) {
        return *v;
    }

    let mut found = 0;

    for towel in &towels[target[0] as usize] {
        if towel.len() > target.len() || !target.starts_with(towel) {
            continue;
        }

        found += if towel.len() == target.len() {
            1
        } else {
            possibilities(&target[towel.len()..], towels, cache)
        };
    }

    cache.insert(target, found);

    found
}

fn solve_second(input: &(Towels, Vec<Vec<u8>>)) -> u64 {
    input.1.iter().map(|target| possibilities(target, &input.0, &mut FxHashMap::with_capacity_and_hasher(192, FxBuildHasher))).sum()
}
