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
    let mut tiles: Vec<u32> = input.tiles.iter().map(|b| if *b { u32::MAX - 1 } else { u32::MAX }).collect();
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

        if tiles[pos + 1] == u32::MAX {
            pos += 1;
        } else if tiles[pos - 1] == u32::MAX {
            pos -= 1;
        } else if tiles[pos + input.width] == u32::MAX {
            pos += input.width;
        } else if tiles[pos - input.width] == u32::MAX {
            pos -= input.width;
        } else {
            panic!();
        }
    }

    sub_hundred
}

const CHEAT_DISTANCE: isize = 20;

fn solve_second(input: Maze) -> usize {
    let mut tiles: Vec<u32> = input.tiles.iter().map(|b| if *b { u32::MAX - 1 } else { u32::MAX }).collect();
    let area = tiles.len();
    let height = area / input.width;

    let mut distance = 0;
    let mut sub_hundred = 0;
    let mut pos = input.end;

    loop {
        tiles[pos] = distance;
        
        {
            let middle_x = pos % input.width;
            let middle_y = pos / input.width;

            let min_dx = (-(middle_x as isize)).max(-CHEAT_DISTANCE);
            let min_dy = (-(middle_y as isize)).max(-CHEAT_DISTANCE);

            let max_dx = (input.width - middle_x - 1).min(CHEAT_DISTANCE as usize) as isize;
            let max_dy = (height - middle_y - 1).min(CHEAT_DISTANCE as usize) as isize;

            for x in min_dx..=max_dx {
                let x_abs = x.abs();
                let x_pos = (pos as isize + x) as usize;

                let min_dy = (-CHEAT_DISTANCE + x_abs).max(min_dy);
                let max_dy = (CHEAT_DISTANCE - x_abs).min(max_dy);

                for y in min_dy..=max_dy {
                    let y_abs = y.abs();
                    let final_pos = (x_pos as isize + (input.width as isize * y)) as usize;

                    let length = x_abs + y_abs;

                    if distance >= (MIN_SKIP_DISTANCE + length as u32) && tiles[final_pos] <= distance - (MIN_SKIP_DISTANCE + length as u32) {
                        sub_hundred += 1;
                    }
                }
            }
        }

        if pos == input.start {
            break;
        }

        distance += 1;

        if tiles[pos + 1] == u32::MAX {
            pos += 1;
        } else if tiles[pos - 1] == u32::MAX {
            pos -= 1;
        } else if tiles[pos + input.width] == u32::MAX {
            pos += input.width;
        } else if tiles[pos - input.width] == u32::MAX {
            pos -= input.width;
        } else {
            panic!();
        }
    }

    sub_hundred
}
