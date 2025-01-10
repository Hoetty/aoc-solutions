use std::fs;

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::solutions;

solutions!{2024, 21}

const X: u8 = 3 << 2;
const Y: u8 = 3;

const BUTTON: u8 = X | Y;

const X_0: u8 = 1 << 2;
const X_1: u8 = 2 << 2;
const X_2: u8 = 3 << 2;
const X_UNSET: u8 = 0;

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

const CONTROL_A: u8 = X_0 | Y_1;
const UP_BUTTON: u8 = X_1 | Y_1;
const DOWN_BUTTON: u8 = X_1 | Y_0;
const RIGHT_BUTTON: u8 = X_0 | Y_0;
const LEFT_BUTTON: u8 = X_2 | Y_0;

fn get_input(file: &str) -> Vec<(usize, u32)> {
    fs::read_to_string(file).expect("No file there").lines().map(|line| (line[..line.len() - 1].parse().unwrap(), {
        let mut sequence: u32 = 0;
        for button in line.chars() {
            sequence = (sequence << 4) | match button {
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
                _ => panic!("Unknown code char {button}")
            } as u32;
        }

        sequence
    })).collect()
}

/// Checks how many buttons have to be pressed on the next keypad 
/// to perform a sequence of button presses on the current keypad
/// For each button to be pressed, this calculates the difference in position and creates a new sequence to move there
/// the number of presses needed for that sequence are than added to the total of the current sequence
/// Once the depth reaches 1 - a human - all moves in a sequence only need one press
fn number_of_presses(sequence: u32, start: u8, depth: u8, cache: &mut FxHashMap<(u32, u8, u8), u64>) -> u64 {
    let key = (sequence, start, depth);

    if let Some(presses) = cache.get(&key) {
        return *presses;
    }

    let mut position = start;
    let mut presses = 0;

    for item in (0..8).rev() {

        let target = ((sequence >> (item * 4)) & BUTTON as u32) as u8;

        // If x is unset, the sequence is shorter than the current value
        if target & X == X_UNSET {
            continue;
        }

        // A human only needs one press to perform this press
        if depth == 1 {
            presses += 1;
            continue;
        }

        let position_x = position & X;
        let position_rx = position_x >> 2;
        let target_x = target & X;
        let target_rx = target >> 2;

        let position_y = position & Y;
        let target_y = target & Y;

        let mut next_sequence: u32 = 0;

        // Left is always valid, unless the target is on x2 and the current y is on the same height as the A button
        let can_move_left = target_x < X_2 || position_y != start & Y;
        // Down is always valid, unless we are on x_2 on the keypad and want to move to the height of the A button
        let can_move_down = target_y != start & Y || position_x != X_2;
        // Up is always valid, unless we are on the control panel on x2,y0
        let can_move_up = start != CONTROL_A || !(position_y == Y_0 && position_x == X_2);
        // Right is always valid

        // We first check the optimal move order of left, down, up, right
        if can_move_left && target_x > position_x {
            for _ in 0..(target_rx - position_rx) {
                next_sequence = (next_sequence << 4) | LEFT_BUTTON as u32;
            }
        }

        if can_move_down && target_y < position_y {
            for _ in 0..(position_y - target_y) {
                next_sequence = (next_sequence << 4) | DOWN_BUTTON as u32;
            }
        }

        if can_move_up && target_y > position_y {
            for _ in 0..(target_y - position_y) {
                next_sequence = (next_sequence << 4) | UP_BUTTON as u32;
            }
        }

        if target_x < position_x {
            for _ in 0..(position_rx - target_rx) {
                next_sequence = (next_sequence << 4) | RIGHT_BUTTON as u32;
            }
        }

        // If the optimal move order can't be done we instead to the safe move order right, up, down, left
        if !can_move_up && target_y > position_y {
            for _ in 0..(target_y - position_y) {
                next_sequence = (next_sequence << 4) | UP_BUTTON as u32;
            }
        }
        
        if !can_move_down && target_y < position_y {
            for _ in 0..(position_y - target_y) {
                next_sequence = (next_sequence << 4) | DOWN_BUTTON as u32;
            }
        }

        if !can_move_left && target_x > position_x {
            for _ in 0..(target_rx - position_rx) {
                next_sequence = (next_sequence << 4) | LEFT_BUTTON as u32;
            }
        }

        next_sequence = (next_sequence << 4) | CONTROL_A as u32;

        presses += number_of_presses(next_sequence, CONTROL_A, depth - 1, cache);
        position = target;
    }

    cache.insert(key, presses);

    presses
}

/// ### Door with 2 Robots
/// 
/// Counts the number of presses for a sequence a human needs to enter into a control panel 
/// to control a robot to input a sequence into a control panel
/// to control a robot to input a sequence into a control panel
/// to control a robot to input a sequence into a key pad
/// to unlock a door
fn solve_first(input: &[(usize, u32)]) -> u64 {
    input.iter().map(|(num, sequence)| (num, {
        number_of_presses(*sequence, KEY_A, 4, &mut FxHashMap::with_capacity_and_hasher(0x80, FxBuildHasher))
    })).map(|(num, count)| *num as u64 * count).sum()
}

/// ### Door with 25 Robots
/// 
/// Counts the number of presses for a sequence a human needs to enter into a control panel 
/// (to control a robot to input a sequence into a control panel) * 25
/// to control a robot to input a sequence into a key pad
/// to unlock a door
fn solve_second(input: &[(usize, u32)]) -> u64 {
    input.iter().map(|(num, sequence)| (num, {
        number_of_presses(*sequence, KEY_A, 27, &mut FxHashMap::with_capacity_and_hasher(0x0800, FxBuildHasher))
    })).map(|(num, count)| *num as u64 * count).sum()
}
