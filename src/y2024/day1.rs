use std::fs;

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 1}

type List = (Vec<u32>, Vec<u32>);

/// Parses lines containing two numbers seperated by whitespace,
///    into two lists: One left list and one right list
fn get_input(file: &str) -> List {
    let content = fs::read_to_string(file).expect("The input file is missing");

    let (left, right): List = content
        .lines()
        .map(|line| line.split_once("   ").expect("The line is not splittable by whitespace"))
        .map(|(left_number, right_number)| (
            left_number.parse::<u32>().expect("Must be number"), 
            right_number.parse::<u32>().expect("Must be a number")
        )).unzip();

    (left, right)
}

/// ### Total List Distance
/// 
/// Provided two lists of numbers,
///   sort both lists and then calculate the pairwise absolute difference
/// The sum of all differences is the total distance of the two lists
/// 
fn solve_first(input: List) -> u32 {
    let (mut left, mut right) = input;

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(first, second)| first.abs_diff(*second))
        .sum()
}

/// ### List Similarity Score
/// 
/// Calculate how often each number from the left list appears in the right list (its frequency)
/// Then sum all numbers from the left list multiplied by their respective frequency
/// 
fn solve_second(input: List) -> u32 {
    let (left, right) = input;
    
    let mut frequencies: FxHashMap<u32, u8> = FxHashMap::with_capacity_and_hasher(1000, FxBuildHasher);

    // First calculate the frequencies of all numbers in the right list 
    for num in right {
        match frequencies.get_mut(&num) {
            Some(count) => { *count += 1; },
            None => { frequencies.insert(num, 1); }
        }
    }

    // Then look up the frequencies of the right list for the numbers in the left list
    left.iter()
        .map(|num| num * *frequencies.get(num).unwrap_or(&0) as u32)
        .sum()
}
