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

    let possible_patterns = second.lines()
        .map(&string_to_num)
        .filter(|target| is_possible(target, &towels))
        .collect();

    (
        towels,
        possible_patterns
    )
}

/// Checks if a pattern is possible
/// Only Towels that have the first stripe correct are checked if they match
/// For each towel, check if the towel isnt longer than the target pattern
/// and if the target starts with the towel. If it does, chop it of and check the remaining pattern
fn is_possible(target: &[u8], towels: &Towels) -> bool {
    for towel in &towels[target[0] as usize] {
        if !target.starts_with(towel) {
            continue;
        }

        // At this point we know that the target starts with the towel,
        // if they are equally long we know the pattern is possible, if not we check the remaining pattern
        if towel.len() == target.len() || is_possible(&target[towel.len()..], towels) {
            return true;
        }
    }

    false
}

/// ### Possible Patterns
/// Counts how many patterns are possible to produce from the towels
/// The solution is precomputed by the input and shared among both solutions 
fn solve_first(input: &(Towels, Vec<Vec<u8>>)) -> usize {
    input.1.len()
}

/// Calculates how many possibilities there are to produce the target pattern from the given towels
/// The number of possibilities is how many paths lead to a the target and towel being equal
fn possibilities<'a>(target: &'a [u8], towels: &Towels, cache: &mut FxHashMap<&'a[u8], u64>) -> u64 {
    if let Some(cached_possibilities) = cache.get(&target) {
        return *cached_possibilities;
    }

    let sum_of_possibilities = towels[target[0] as usize].iter().filter_map(|towel| {
        if !target.starts_with(towel) {
            None
        } else if towel.len() == target.len() {
            Some(1)
        } else {
            Some(possibilities(&target[towel.len()..], towels, cache))
        }
    }).sum();

    cache.insert(target, sum_of_possibilities);

    sum_of_possibilities
}

/// ### Possibilities for Patterns
/// Calculates how many possibilities there are to build all patterns
/// Only known to be possible patterns are tested
fn solve_second(input: &(Towels, Vec<Vec<u8>>)) -> u64 {
    input.1.iter()
        .map(|target| possibilities(target, &input.0, &mut FxHashMap::with_capacity_and_hasher(192, FxBuildHasher)))
        .sum()
}
