use std::fs;

use crate::solutions;

solutions!{2024, 25}

const WIDTH: usize = 5;
const OFFSET: usize = 6;
const HEIGHT: u8 = 7;

const NUMBER_OF_SCHEMATICS: usize = 250;

type Heights = [u8; WIDTH];

fn get_input(file: &str) -> (Vec<Heights>, Vec<Heights>) {
    let file = fs::read_to_string(file).unwrap();
    let schematics = file.split("\n\n")
        .map(|schematic| schematic.chars().collect::<Vec<_>>())
        .map(|schematic| if schematic[0] == '#' { 
            // If we see this character for the first time, we know the height
            (true, schematic, '.') 
        } else { 
            (false, schematic, '#') 
        });

    let mut locks = Vec::with_capacity(NUMBER_OF_SCHEMATICS);
    let mut keys = Vec::with_capacity(NUMBER_OF_SCHEMATICS);

    for (is_lock, schematic, character) in schematics {
        let mut heights = [0; WIDTH];
        for x in 0..WIDTH {
            for y in 0..(HEIGHT as usize) {
                // Due to the newlines not being stripped, the offset is 6
                if schematic[x + y * OFFSET] == character {
                    heights[x] = if is_lock { y as u8 } else { HEIGHT - y as u8 };
                    break;
                }
            }
        }

        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    (locks, keys)
}

/// ### Fitting Key
/// 
/// Tests all keys for each lock to see if the keys heights fit into the locks height,
/// shown by their combined heights being at most 7
fn solve_first(input: &(Vec<Heights>, Vec<Heights>)) -> u64 {
    let mut fitting = 0;

    for lock in &input.0 {
        for key in &input.1 {
            if 
                lock[0] + key[0] <= HEIGHT && 
                lock[1] + key[1] <= HEIGHT && 
                lock[2] + key[2] <= HEIGHT && 
                lock[3] + key[3] <= HEIGHT && 
                lock[4] + key[4] <= HEIGHT 
            {
                fitting += 1;
            }
        }
    }

    fitting
}

/// ### Chronicle
/// 
/// Does nothing as Part 1 was the last puzzle
fn solve_second(_input: &(Vec<Heights>, Vec<Heights>)) -> &'static str {
    ""
}
