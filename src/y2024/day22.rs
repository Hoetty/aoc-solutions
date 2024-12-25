use std::{fs::{self}, hash::{BuildHasherDefault, Hash}};

use fxhash::{FxHashMap, FxHashSet};

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 22}

fn get_input(file: &str) -> Vec<u32> {
    fs::read_to_string(file).expect("No file there").lines().map(|line| line.parse().unwrap()).collect()
}

#[inline(always)]
fn mix(lhs: u32, rhs: u32) -> u32 {
    lhs ^ rhs
}

#[inline(always)]
fn prune(lhs: u32) -> u32 {
    lhs & 0xFFFFFF
}

#[inline]
fn step_unpruned(secret: u32) -> u32 {
    let step1 = prune(mix(secret << 6, secret));
    let step2 = mix(step1 >> 5, step1);
    mix(step2 << 11, step2)
}

#[inline]
fn step(secret: u32) -> u32 {
    prune(step_unpruned(secret))
}


fn step_n_times(secret: u32, n: usize) -> u32 {
    let mut secret = secret;

    for _ in 0..n {
        secret = step_unpruned(secret);
    }

    prune(secret)
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

fn gains(secret: u32, gains: &mut FxHashSet<Gain>) {
    let mut secret = secret;
    let mut sequence = (0, 0, 0, 0);

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
}

fn solve_second(input: Vec<u32>) -> u32 {
    let mut scores: FxHashMap<(i8, i8, i8, i8), u32> = FxHashMap::with_capacity_and_hasher(2048, BuildHasherDefault::default());

    let mut set: FxHashSet<Gain> = FxHashSet::with_capacity_and_hasher(2048, BuildHasherDefault::default());

    for num in input {
        gains(num, &mut set);
        for gain in set.drain() {
            scores.entry(gain.0)
                .and_modify(|v| *v += gain.1 as u32)
                .or_insert(gain.1 as u32);
        }
    }

    *scores.values().max().unwrap()
}
