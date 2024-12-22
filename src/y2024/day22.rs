use std::{fs::{self}, hash::{BuildHasherDefault, Hash}};

use fxhash::{FxHashMap, FxHashSet};

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day22.txt");

    Solution::evaluated(
        "Day 22".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

fn get_input(file: &'static str) -> Vec<u32> {
    fs::read_to_string(file).expect("No file there").lines().map(|line| line.parse().unwrap()).collect()
}

#[inline(always)]
fn mix(lhs: u32, rhs: u32) -> u32 {
    lhs ^ rhs
}

#[inline(always)]
fn prune(lhs: u32) -> u32 {
    lhs & 16777215
}

#[inline]
fn step(secret: u32) -> u32 {
    let step1 = prune(mix(secret << 6, secret));
    let step2 = mix(step1 >> 5, step1);
    prune(mix(step2 << 11, step2))
}

fn step_n_times(secret: u32, n: usize) -> u32 {
    let mut secret = secret;

    for _ in 0..n {
        secret = step(secret);
    }

    secret
}

fn solve_first(input: Vec<u32>) -> u64 {
    input.iter().map(|num| step_n_times(*num, 2000) as u64).sum()
}

#[derive(Clone, Copy)]
struct Gain((i8, i8, i8, i8), u8);

impl Hash for Gain {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for Gain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Gain {

}

fn gains(secret: u32) -> FxHashSet<Gain> {
    let mut secret = secret;
    let mut sequence = (0, 0, 0, 0);

    let mut gains = FxHashSet::with_capacity_and_hasher(1024, BuildHasherDefault::default());

    for i in 0..2000 {
        let next = step(secret);

        let bananas = (next as i32 % 10) as i8;
        let bananas_last = (secret as i32 % 10) as i8;

        sequence = (sequence.1, sequence.2, sequence.3, bananas - bananas_last);

        secret = next;

        if i > 3 {
            gains.insert(Gain(sequence, bananas as u8));
        }
    }

    gains
}

fn solve_second(input: Vec<u32>) -> u32 {
    let mut scores: FxHashMap<(i8, i8, i8, i8), u32> = FxHashMap::with_capacity_and_hasher(4096, BuildHasherDefault::default());

    for num in input {
        for gain in gains(num) {
            match scores.get_mut(&gain.0) {
                Some(v) => *v += gain.1 as u32,
                None => { scores.insert(gain.0, gain.1 as u32); },
            }
        }
    }

    let mut best = ((0, 0, 0, 0), 0);

    for score in scores {
        if score.1 > best.1 {
            best = score;
        }
    }

    best.1
}
