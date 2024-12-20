use std::{fs::{self}, ops::{BitAnd, BitOr, BitXor, Shl, Shr}, u128};

use fxhash::FxHashMap;

use crate::Solution;

const WHITE: U256 = U256([0, 1]);
const BLUE: U256 = U256([0, 2]);
const BLACK: U256 = U256([0, 3]);
const RED: U256 = U256([0, 4]);
const GREEN: U256 = U256([0, 5]);

const SHIFT: usize = 3;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct U256([u128; 2]);

impl U256 {
    const ONE: U256 = U256([0, 1]);
    const ZERO: U256 = U256([0, 0]);

    fn decremented(&self) -> U256 {

        if self.0[0] == 0 {
            if self.0[1] == 0 {
                U256::ZERO
            } else {
                U256([0, self.0[1] - 1])
            }
        } else {
            if self.0[1] == 0 {
                U256([self.0[0] - 1, u128::MAX])
            } else {
                U256([0, self.0[1] - 1])
            }
        }
    }
}

impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U256 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0[1].cmp(&other.0[1]) {
            std::cmp::Ordering::Equal => self.0[0].cmp(&other.0[0]),
            other => other,
        }
    }
}

impl BitXor for U256 {
    type Output = U256;

    fn bitxor(self, rhs: Self) -> Self::Output {
        U256([self.0[0] ^ rhs.0[0], self.0[1] ^ rhs.0[1]])
    }
}

impl BitAnd for U256 {
    type Output = U256;

    fn bitand(self, rhs: Self) -> Self::Output {
        U256([self.0[0] & rhs.0[0], self.0[1] & rhs.0[1]])
    }
}

impl BitOr for U256 {
    type Output = U256;

    fn bitor(self, rhs: Self) -> Self::Output {
        U256([self.0[0] | rhs.0[0], self.0[1] | rhs.0[1]])
    }
}

impl Shr<usize> for U256 {
    type Output = U256;

    fn shr(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return self;
        }

        if rhs >= 256 {
            return U256::ZERO;
        }

        if rhs >= 128 {
            U256([0, self.0[0] >> (rhs - 128)])
        } else {
            let overflow = self.0[0] << (128 - rhs);
            U256([self.0[0] >> rhs, (self.0[1] >> rhs) | overflow])
        }
    }
}

impl Shl<usize> for U256 {
    type Output = U256;

    fn shl(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return self;
        }

        if rhs >= 256 {
            return U256::ZERO;
        }

        if rhs >= 128 {
            U256([self.0[1] << (rhs - 128), 0])
        } else {
            let overflow = self.0[1] >> (128 - rhs);
            U256([(self.0[0] << rhs) | overflow, self.0[1] << rhs])
        }
    }
}

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day19.txt");

    Solution::evaluated(
        "Day 19".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pattern(U256, usize);

fn string_to_num(pattern: &str) -> Pattern {
    let mut num = U256::ZERO;
    let mut i = 0;

    for c in pattern.chars().rev() {
        num = num | (match c {
            'w' => WHITE,
            'u' => BLUE,
            'b' => BLACK,
            'r' => RED,
            'g' => GREEN,
            _ => panic!("Unkown char {c}")
        } << SHIFT * i);
        i += 1;
    }

    Pattern(num, i)
}

fn get_input(file: &'static str) -> (Vec<Vec<Pattern>>, Vec<Pattern>) {
    let file = fs::read_to_string(file).expect("No file there");
    let (first, second) = file.split_once("\n\n").unwrap();

    let mut patterns: Vec<Vec<Pattern>> = Vec::new();
    patterns.resize(6, Vec::new());

    for pattern in first.split(", ").map(&string_to_num) {
        patterns[(pattern.0.0[1] & 7) as usize].push(pattern);
    }

    (
        patterns,
        second.lines().map(&string_to_num).collect()
    )
}

fn is_possible(target: Pattern, patterns: &Vec<Vec<Pattern>>, cache: &mut FxHashMap<Pattern, bool>) -> bool {
    if let Some(v) = cache.get(&target) {
        return *v;
    }

    for pattern in &patterns[(target.0.0[1] & 7) as usize] {
        if pattern.1 > target.1 {
            continue;
        }

        let wrong = target.0 ^ pattern.0;

        if wrong == U256::ZERO {
            cache.insert(target, true);
            return true;
        }

        let tested_bits = (U256::ONE << pattern.1 * SHIFT).decremented();

        if wrong & tested_bits == U256::ZERO {
            let next = Pattern(target.0 >> (pattern.1 * SHIFT), target.1 - pattern.1);

            if is_possible(next, patterns, cache) {
                return true;
            }
        }
    }

    cache.insert(target, false);

    false
}

fn solve_first(input: (Vec<Vec<Pattern>>, Vec<Pattern>)) -> usize {
    input.1.iter().filter(|target| is_possible(**target, &input.0, &mut FxHashMap::default())).count()
}

fn possibilities(target: Pattern, patterns: &Vec<Vec<Pattern>>, cache: &mut FxHashMap<Pattern, u64>) -> u64 {
    if let Some(v) = cache.get(&target) {
        return *v;
    }

    let mut found = 0;

    for pattern in &patterns[(target.0.0[1] & 7) as usize] {
        if pattern.1 > target.1 {
            continue;
        }

        let wrong = target.0 ^ pattern.0;

        if wrong == U256::ZERO {
            found += 1;
        }

        let tested_bits = (U256::ONE << pattern.1 * SHIFT).decremented();

        if wrong & tested_bits == U256::ZERO {
            let next = Pattern(target.0 >> (pattern.1 * SHIFT), target.1 - pattern.1);

            found += possibilities(next, patterns, cache);
        }
    }

    cache.insert(target, found);

    found
}

fn solve_second(input: (Vec<Vec<Pattern>>, Vec<Pattern>)) -> u64 {
    input.1.iter().map(|target| possibilities(*target, &input.0, &mut FxHashMap::default())).sum()
}
