use std::fs;

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 20}

#[derive(Clone)]
struct Maze {
    tiles: Vec<bool>,
    start: usize,
    end: usize,
    width: usize
}

fn get_input(file: &str) -> Maze {
    let file = fs::read_to_string(file).expect("No file there");
    
    let mut tiles: Vec<bool> = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut width = 0;

    for (y, line) in file.lines().enumerate() {
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            tiles.push(c == '#');

            if c == 'S' {
                start = y * width + x;
            } else if c == 'E' {
                end = y * width + x;
            }
        }
    }

    Maze {
        tiles,
        start,
        end,
        width
    }
}

const MIN_SKIP_DISTANCE: u32 = 100;

fn solve_first(input: Maze) -> usize {
    let mut tiles: Vec<u16> = input.tiles.iter().map(|b| if *b { u16::MAX - 1 } else { u16::MAX }).collect();
    let area = tiles.len();
    let floor = input.width * 2;
    let ceiling = area - floor;

    let mut distance = 0;
    let mut sub_hundred = 0;
    let mut pos = input.end;

    loop {
        tiles[pos] = distance;

        if distance >= 102 {
            if pos % input.width > 1 && tiles[pos - 2] <= distance - 102 {
                sub_hundred += 1;
            }

            if pos % input.width < input.width - 2 && tiles[pos + 2] <= distance - 102 {
                sub_hundred += 1;
            }

            if pos >= floor && tiles[pos - floor] <= distance - 102 {
                sub_hundred += 1;
            }

            if pos < ceiling && tiles[pos + floor] <= distance - 102 {
                sub_hundred += 1;
            }
        }

        if pos == input.start {
            break;
        }

        distance += 1;

        if tiles[pos + 1] == u16::MAX {
            pos += 1;
        } else if tiles[pos - 1] == u16::MAX {
            pos -= 1;
        } else if tiles[pos + input.width] == u16::MAX {
            pos += input.width;
        } else if tiles[pos - input.width] == u16::MAX {
            pos -= input.width;
        } else {
            panic!();
        }
    }

    sub_hundred
}

const CHEAT_DISTANCE: u8 = 20;

fn solve_second(input: Maze) -> usize {
    let mut tiles: Vec<u16> = input.tiles.iter().map(|b| if *b { 0 } else { u16::MAX }).collect();
    let area = tiles.len();

    let mut pos = input.end;

    let mut path: Vec<(u8, u8)> = Vec::with_capacity(area / 4);
    let mut chunks: Vec<Vec<((u8, u8), usize)>> = vec![vec![]; 49];

    let mut distance_from_end = 0;

    loop {
        tiles[pos] = 0;

        let x = pos % input.width;
        let y = pos / input.width;

        chunks[x / 20 + y / 20 * 7].push(((x as u8, y as u8), distance_from_end));
        path.push((x as u8, y as u8));
        distance_from_end += 1;

        if pos == input.start {
            break;
        }

        if tiles[pos + 1] == u16::MAX {
            pos += 1;
        } else if tiles[pos - 1] == u16::MAX {
            pos -= 1;
        } else if tiles[pos + input.width] == u16::MAX {
            pos += input.width;
        } else if tiles[pos - input.width] == u16::MAX {
            pos -= input.width;
        } else {
            panic!();
        }
    }

    let last_possible_index = path.len() - MIN_SKIP_DISTANCE as usize;
    let mut sub_hundred = 0;

    for (lower_index, lower_coords) in path.iter().enumerate() {
        if lower_index >= last_possible_index {
            break;
        }

        let middle_chunk = lower_coords.0 / 20 + lower_coords.1 / 20 * 7;
        let middle_x = middle_chunk % 7;
        let middle_y = middle_chunk / 7;

        let min_dx = if middle_x > 0 { -1 } else { 0 };
        let max_dx = if middle_x < 6 { 1 } else { 0 };
        let min_dy = if middle_y > 0 { -1 } else { 0 };
        let max_dy = if middle_y < 6 { 1 } else { 0 };

        for chunk_dx in min_dx..=max_dx {
            for chunk_dy in min_dy..max_dy {
                for (higher_coords, higher_index) in &chunks[(middle_chunk as isize + chunk_dx + 7 * chunk_dy) as usize] {
                    let distance = lower_coords.0.abs_diff(higher_coords.0) as usize + lower_coords.1.abs_diff(higher_coords.1) as usize;

                    if distance <= CHEAT_DISTANCE as usize && *higher_index >= distance + lower_index + MIN_SKIP_DISTANCE as usize {
                        sub_hundred += 1;
                    }
                }
            }
        }
    }



    // for (lower_index, lower_coords) in path.iter().enumerate() {
    //     if lower_index >= last_possible_index {
    //         break;
    //     }

    //     for (higher_index, higher_coords) in path[lower_index + MIN_SKIP_DISTANCE as usize..].iter().enumerate() {
    //         let distance = lower_coords.0.abs_diff(higher_coords.0) as usize + lower_coords.1.abs_diff(higher_coords.1) as usize;

    //         if distance <= CHEAT_DISTANCE as usize && higher_index >= distance {
    //             sub_hundred += 1;
    //         }
    //     }
    // }

    sub_hundred
}
