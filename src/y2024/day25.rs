use std::fs;

use crate::solutions;

solutions!{2024, 25}

type Heights = [u8; 5];

fn get_input(file: &str) -> (Vec<Heights>, Vec<Heights>) {
    let file = fs::read_to_string(file).unwrap();
    let patterns = file.split("\n\n");

    let mut templates = Vec::with_capacity(64);
    let mut keys = Vec::with_capacity(64);

    for pattern in patterns {
        let (destination, iter) = if pattern.starts_with('#') {
            (&mut templates, pattern.lines().collect::<Vec<&str>>())
        } else {
            (&mut keys, pattern.lines().rev().collect())
        };

        let mut heights = [0; 5];
        for (y, line) in iter.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '.' && heights[x] == 0 {
                    heights[x] = y as u8;
                }
            }
        }

        destination.push(heights);
    }

    (templates, keys)
}

const HEIGHT: u8 = 7;

fn solve_first(input: &(Vec<Heights>, Vec<Heights>)) -> u64 {
    let mut fitting = 0;

    for template in &input.0 {
        for key in &input.1 {
            if template[0] + key[0] <= HEIGHT && template[1] + key[1] <= HEIGHT && template[2] + key[2] <= HEIGHT && template[3] + key[3] <= HEIGHT && template[4] + key[4] <= HEIGHT {
                fitting += 1;
            }
        }
    }

    fitting
}

fn solve_second(_input: &(Vec<Heights>, Vec<Heights>)) -> &'static str {
    ""
}
