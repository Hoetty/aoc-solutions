use std::fs;

use crate::solutions;

solutions!{2024, 22}

/// Mixing calculates the xor of both values
#[inline(always)]
fn mix(lhs: u32, rhs: u32) -> u32 {
    lhs ^ rhs
}

/// Pruning only keeps the last bits
#[inline(always)]
fn prune(lhs: u32) -> u32 {
    lhs & 0xFFFFFF
}

/// Calculates the next pseudorandom number from the previous
#[inline]
fn step(secret: u32) -> u32 {
    let step1 = prune(mix(secret << 6, secret));
    let step2 = mix(step1 >> 5, step1);
    prune(mix(step2 << 11, step2))
}

const MASK: usize = (1 << 20) - 1;

/// Calculates each monkeys 2000th number and keeps track of the current price sequence 
/// and the greatest possible amount of bananas in one go
fn get_input(file: &str) -> (u64, u16) {
    let input: Vec<_> = fs::read_to_string(file).expect("No file there").lines().map(|line| line.parse().unwrap()).collect();

    let mut two_thousands = 0;

    // Kepp track of the scores of each sequence. Sequences are 5 bits * 4 = 20 bits long, 
    // so we use them as array indices
    let mut scores: Vec<u16> = vec![0; 1 << 20];
    let mut seen: Vec<u16> = vec![0; 1 << 20];

    for (monkey, secret) in input.iter().enumerate() {
        let mut secret = *secret;
        let mut sequence = 0;
    
        let mut last_bananas = secret as usize % 10;

        for i in 0..2000 {
            let next = step(secret);
            let bananas = next as usize % 10;
    
            // Update the sequence
            sequence = ((sequence << 5) | (10 + bananas - last_bananas)) & MASK;
            last_bananas = bananas;
    
            secret = next;
    
            // Check if the sequence has already been seen by this monkey
            if i > 3 && seen[sequence] < monkey as u16 + 1 {
                // If not, increment the according score by the bananas
                scores[sequence] += bananas as u16;
                // And set the sequence to seen by this monkey
                seen[sequence] = monkey as u16 + 1;
            }
        }

        // Add the 2000th number of this monkey to the sum
        two_thousands += secret as u64;
    }

    (two_thousands, *scores.iter().max().unwrap())
}

/// ### 2000th Number Sum
/// 
/// Gets the precalculated 2000th number sum from input
fn solve_first(input: &(u64, u16)) -> u64 {
    input.0
}

/// ### Most Bananas
/// 
/// Gets the precalculated banana amount from the input
fn solve_second(input: &(u64, u16)) -> u16 {
    input.1
}
