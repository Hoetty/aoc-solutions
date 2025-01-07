use std::fs;

use crate::solutions;

solutions!{2024, 22}

#[inline(always)]
fn mix(lhs: u32, rhs: u32) -> u32 {
    lhs ^ rhs
}

#[inline(always)]
fn prune(lhs: u32) -> u32 {
    lhs & 0xFFFFFF
}

#[inline]
fn step(secret: u32) -> u32 {
    let step1 = prune(mix(secret << 6, secret));
    let step2 = mix(step1 >> 5, step1);
    prune(mix(step2 << 11, step2))
}

fn get_input(file: &str) -> (u64, u16) {
    let input: Vec<_> = fs::read_to_string(file).expect("No file there").lines().map(|line| line.parse().unwrap()).collect();

    let mut two_thousands = 0;

    let mut scores: Vec<u16> = vec![0; 1 << 20];
    let mut seen: Vec<u16> = vec![0; 1 << 20];

    for (monkey, secret) in input.iter().enumerate() {
        let mut secret = *secret;
        let mut sequence = 0;
    
        for i in 0..2000 {
            let next = step(secret);
    
            let bananas = (next % 10) as i8;
            let bananas_last = (secret % 10) as i8;
    
            sequence = (sequence << 5) | (10 + bananas - bananas_last) as usize;
    
            secret = next;
    
            if i > 3 && seen[sequence & MASK] < monkey as u16 + 1 {
                scores[sequence & MASK] += bananas as u16;
                seen[sequence & MASK] = monkey as u16 + 1;
            }
        }

        two_thousands += secret as u64;
    }

    (two_thousands, *scores.iter().max().unwrap())
}

fn solve_first(input: &(u64, u16)) -> u64 {
    input.0
}

const MASK: usize = (1 << 20) - 1;

fn solve_second(input: &(u64, u16)) -> u16 {
    input.1
}
