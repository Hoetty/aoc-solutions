use std::fs;

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

const MASK: usize = (1 << 20) - 1;

fn solve_second(input: Vec<u32>) -> u16 {
    let mut scores: Vec<u16> = vec![0; 1 << 20];
    let mut seen: Vec<u16> = vec![0; 1 << 20];

    for (monkey, secret) in input.iter().enumerate() {
        let mut secret = *secret;
        let mut sequence = 0;
    
        for i in 0..2000 {
            let next = step(secret);
    
            let bananas = (next as i32 % 10) as i8;
            let bananas_last = (secret as i32 % 10) as i8;
    
            sequence = (sequence << 5) | (10 + bananas - bananas_last) as usize;
    
            secret = next;
    
            if i > 3 && seen[sequence & MASK] < monkey as u16 + 1 {
                scores[sequence & MASK] += bananas as u16;
                seen[sequence & MASK] = monkey as u16 + 1;
            }
        }
    }

    *scores.iter().max().unwrap()
}
