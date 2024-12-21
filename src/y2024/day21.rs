use std::{fs, hash::BuildHasherDefault, u32};

use fxhash::FxHashMap;

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day21.txt");

    Solution::evaluated(
        "Day 21".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

const X: u8 = 3 << 2;
const Y: u8 = 3;

const BUTTON: u8 = X | Y;

const X_0: u8 = 1 << 2;
const X_1: u8 = 2 << 2;
const X_2: u8 = 3 << 2;
const X_POISON: u8 = 0;

const Y_0: u8 = 0;
const Y_1: u8 = 1;
const Y_2: u8 = 2;
const Y_3: u8 = 3;

const KEY_A: u8 = X_0 | Y_0;
const KEY_0: u8 = X_1 | Y_0;
const KEY_1: u8 = X_2 | Y_1;
const KEY_2: u8 = X_1 | Y_1;
const KEY_3: u8 = X_0 | Y_1;
const KEY_4: u8 = X_2 | Y_2;
const KEY_5: u8 = X_1 | Y_2;
const KEY_6: u8 = X_0 | Y_2;
const KEY_7: u8 = X_2 | Y_3;
const KEY_8: u8 = X_1 | Y_3;
const KEY_9: u8 = X_0 | Y_3;


fn get_input(file: &'static str) -> Vec<(usize, u32)> {
    fs::read_to_string(file).expect("No file there").lines().map(|line| (line[..line.len() - 1].parse().unwrap(), {
        let mut sequence: u32 = 0;
        for c in line.chars() {
            sequence = (sequence << 4) | match c {
                'A' => KEY_A,
                '0' => KEY_0,
                '1' => KEY_1,
                '2' => KEY_2,
                '3' => KEY_3,
                '4' => KEY_4,
                '5' => KEY_5,
                '6' => KEY_6,
                '7' => KEY_7,
                '8' => KEY_8,
                '9' => KEY_9,
                _ => panic!("Unknown code char {c}")
            } as u32;
        }

        sequence
    })).collect()
}

const CONTROL_A: u8 = X_0 | Y_1;
const UP_BUTTON: u8 = X_1 | Y_1;
const DOWN_BUTTON: u8 = X_1 | Y_0;
const RIGHT_BUTTON: u8 = X_0 | Y_0;
const LEFT_BUTTON: u8 = X_2 | Y_0;

fn number_of_presses(sequence: u32, start: u8, depth: u8, cache: &mut FxHashMap<(u32, u8, u8), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }

    let key = (sequence, start, depth);

    if let Some(v) = cache.get(&key) {
        return *v;
    }

    let mut position = start;
    let mut count = 0;

    for item in (0..8).rev() {

        let target = ((sequence >> item * 4) & BUTTON as u32) as u8;

        if target & X == X_POISON {
            continue;
        }

        let position_x = position & X;
        let position_rx = position_x >> 2;
        let target_x = target & X;
        let target_rx = target >> 2;

        let position_y = position & Y;
        let target_y = target & Y;

        let mut new_sequence: u32 = 0;

        let can_move_left = target_x < X_2 || position_y != start & Y;
        let can_move_down = target_y != start & Y || position_x != X_2;
        let can_move_up = start != CONTROL_A || !(position_y == Y_0 && position_x == X_2);

        if can_move_left && target_x > position_x {
            for _ in 0..(target_rx - position_rx) {
                new_sequence = (new_sequence << 4) | LEFT_BUTTON as u32;
            }
        }

        if can_move_down && target_y < position_y {
            for _ in 0..(position_y - target_y) {
                new_sequence = (new_sequence << 4) | DOWN_BUTTON as u32;
            }
        }

        if can_move_up && target_y > position_y {
            for _ in 0..(target_y - position_y) {
                new_sequence = (new_sequence << 4) | UP_BUTTON as u32;
            }
        }

        if target_x < position_x {
            for _ in 0..(position_rx - target_rx) {
                new_sequence = (new_sequence << 4) | RIGHT_BUTTON as u32;
            }
        }

        if !can_move_up && target_y > position_y {
            for _ in 0..(target_y - position_y) {
                new_sequence = (new_sequence << 4) | UP_BUTTON as u32;
            }
        }
        
        if !can_move_down && target_y < position_y {
            for _ in 0..(position_y - target_y) {
                new_sequence = (new_sequence << 4) | DOWN_BUTTON as u32;
            }
        }

        if !can_move_left && target_x > position_x {
            for _ in 0..(target_rx - position_rx) {
                new_sequence = (new_sequence << 4) | LEFT_BUTTON as u32;
            }
        }

        new_sequence = (new_sequence << 4) | CONTROL_A as u32;

        count += number_of_presses(new_sequence, CONTROL_A, depth - 1, cache);
        position = target;
    }

    cache.insert(key, count);

    count
}

fn solve_first(input: Vec<(usize, u32)>) -> u64 {
    input.iter().map(|(num, sequence)| (num, {
        number_of_presses(*sequence, KEY_A, 4, &mut FxHashMap::with_capacity_and_hasher(0x80, BuildHasherDefault::default()))
    })).map(|(num, count)| *num as u64 * count).sum()
}

fn solve_second(input: Vec<(usize, u32)>) -> u64 {
    input.iter().map(|(num, sequence)| (num, {
        number_of_presses(*sequence, KEY_A, 27, &mut FxHashMap::with_capacity_and_hasher(0x0800, BuildHasherDefault::default()))
    })).map(|(num, count)| *num as u64 * count).sum()
}
