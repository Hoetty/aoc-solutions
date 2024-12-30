use std::{collections::VecDeque, fs};

use rustc_hash::FxHashSet;

use crate::solutions;

solutions!{2024, 10}

fn get_input(file: &str) -> Vec<(usize, usize)> {
    let mut trail_heads = Vec::with_capacity(64);
    let mut map = Vec::with_capacity(1024);

    let mut width = 0;
    let mut height = 0;

    for (y, line) in fs::read_to_string(file).expect("No file there").lines().enumerate() {
        width = line.len();
        height += 1;
        for (x, level) in line.chars().map(|character| character.to_digit(10).unwrap() as u8).enumerate() {
            map.push(level);

            if level == 0 {
                trail_heads.push(y * width + x);
            }
        }
    }

    get_trails(&map, &trail_heads, width, height)
}

/// Find all trails, where a trail is a path from 0 to 9 with one step increments
/// trail_heads contains all 0 starting positions
fn get_trails(map: &[u8], trail_heads: &Vec<usize>, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut trails = Vec::with_capacity(1024);
    // A queue to hold all positions we need to evaluate
    let mut queue = VecDeque::new();

    for tail_head in trail_heads {
        queue.push_back(*tail_head);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let value = map[current];
            let next_value = value + 1;

            // If the current value is a 9, we hit an end, add it to the list and continue
            if value == 9 {
                trails.push((*tail_head, current));
                continue;
            }

            if current % width < width - 1 && map[current + 1] == next_value {
                queue.push_back(current + 1);
            }

            if current % width > 0 && map[current - 1] == next_value {
                queue.push_back(current - 1);
            }

            if current / width < height - 1 && map[current + width] == next_value {
                queue.push_back(current + width);
            }

            if current / width > 0 && map[current - width] == next_value {
                queue.push_back(current - width);
            }
        }
    }

    trails
}

/// ### Unique Scoring
/// 
/// Calculate the sum of all trail heads' scores - the number of unique trails 
fn solve_first(input: &[(usize, usize)]) -> usize {
    FxHashSet::from_iter(input.iter()).len()
}

/// ### Duplicate Scoring
/// 
/// Calculate the sum of all trail heads' scores - the number of trails
fn solve_second(input: &[(usize, usize)]) -> usize {
    input.len()
}
