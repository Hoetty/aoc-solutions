use std::fs;

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 4}

fn get_input(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file).expect("No file there").lines().map(|line| line.chars().collect()).collect()
}

/// ### XMAS Word Search
/// 
/// Find all occurences of the word "XMAS" in any direction on the grid
fn solve_first(input: Vec<Vec<char>>) -> i32 {
    let puzzle_height = input.len() as isize;
    let puzzle_width = input[0].len() as isize;
    let mut xmas_count = 0;

    // Foreach character on the grid
    for y in 0..puzzle_height {
        for x in 0..puzzle_width {
            // If that character is an 'X'
            if input[y as usize][x as usize] == 'X' {
                // Check in all directions
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if 
                            // Check if the 'S' fits inside the grid
                            x + 3 * dx >= 0 && x + 3 * dx < puzzle_width &&
                            y + 3 * dy >= 0 && y + 3 * dy < puzzle_height &&
                            // Check if the next characters are MAS
                            input[(y + dy) as usize][(x + dx) as usize] == 'M' &&
                            input[(y + 2 * dy) as usize][(x + 2 * dx) as usize] == 'A' &&
                            input[(y + 3 * dy) as usize][(x + 3 * dx) as usize] == 'S' 
                        {
                            xmas_count += 1;
                        }
                    }
                }
            }
        }
    }

    xmas_count
}

#[inline(always)]
fn is_s_or_m(c: char) -> bool {
    c == 'S' || c == 'M'
}

/// An X-MAS is found, when the center is an 'A', which is already true at this point,
///    and when the corners are either 'S' or 'M' and opposite corners are not equal
#[inline(always)]
fn is_x_mas(tl: char, tr: char, bl: char, br: char) -> bool {
    is_s_or_m(tl) && is_s_or_m(tr) && is_s_or_m(bl) && is_s_or_m(br) && tl != br && tr != bl
}

/// ### X-Mas Word Search
/// 
/// Finds all MAS assembled in an X Shape in the puzzle grid
fn solve_second(input: Vec<Vec<char>>) -> i32 {
    let puzzle_height = input.len();
    let puzzle_width = input[0].len();
    
    let mut xmas_count = 0;
    for y in 1..puzzle_height - 1 {
        for x in 1..puzzle_width - 1 {
            // If we land on an 'A', check the surroundings for M's and S'
            if input[y][x] == 'A' {
                let tl = input[y - 1][x - 1];
                let tr = input[y - 1][x + 1];
                let bl = input[y + 1][x - 1];
                let br = input[y + 1][x + 1];

                if is_x_mas(tl, tr, bl, br) {
                    xmas_count += 1;       
                }
            }
        }
    }

    xmas_count
}
