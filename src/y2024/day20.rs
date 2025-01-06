use std::fs;

use crate::{solutions, util::flatgrid::FlatGrid};

solutions!{2024, 20}

type Maze = FlatGrid<u16, 141, 141>;

fn get_input(file: &str) -> (Maze, usize, usize) {
    let file = fs::read_to_string(file).expect("No file there");
    
    let mut maze: Maze = Maze::new();
    let mut start = 0;
    let mut end = 0;

    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            maze.push(if c == '#' { u16::MAX - 1 } else { u16::MAX });

            if c == 'S' {
                start = Maze::to_index(x, y);
            } else if c == 'E' {
                end = Maze::to_index(x, y);
            }
        }
    }

    (maze, start, end)
}

const MIN_SKIP_DISTANCE: u32 = 100;

fn solve_first(input: &(Maze, usize, usize)) -> usize {
    let (maze, start, end) = input;
    let mut maze = maze.clone();
    let (start, end) = (*start, *end);

    let mut distance = 0;
    let mut sub_hundred = 0;
    let mut pos = end;

    loop {
        maze[pos] = distance;

        if distance >= 102 {
            if !Maze::will_horizontal_move_cross_border(pos, -2) && maze[Maze::moved_horizontally(pos, -2)] <= distance - 102 {
                sub_hundred += 1;
            }

            if !Maze::will_horizontal_move_cross_border(pos, 2) && maze[Maze::moved_horizontally(pos, 2)] <= distance - 102 {
                sub_hundred += 1;
            }

            if !Maze::will_vertical_move_cross_border(pos, -2) && maze[Maze::moved_vertically(pos, -2)] <= distance - 102 {
                sub_hundred += 1;
            }

            if !Maze::will_vertical_move_cross_border(pos, 2) && maze[Maze::moved_vertically(pos, 2)] <= distance - 102 {
                sub_hundred += 1;
            }
        }

        if pos == start {
            break;
        }

        distance += 1;

        if maze[Maze::moved_horizontally(pos, 1)] == u16::MAX {
            pos += 1;
        } else if maze[Maze::moved_horizontally(pos, -1)] == u16::MAX {
            pos -= 1;
        } else if maze[Maze::moved_vertically(pos, 1)] == u16::MAX {
            pos += Maze::width();
        } else if maze[Maze::moved_vertically(pos, -1)] == u16::MAX {
            pos -= Maze::width();
        } else {
            panic!();
        }
    }

    sub_hundred
}

const CHEAT_DISTANCE: usize = 20;

fn solve_second(input: &(Maze, usize, usize)) -> usize {
    let (maze, start, end) = input;
    let mut maze = maze.clone();
    let (start, end) = (*start, *end);

    let mut pos = end;
    let mut path: Vec<(u8, u8)> = Vec::with_capacity(Maze::area() / 4);

    loop {
        maze[pos] = 0;

        let (x, y) = Maze::to_coordinates(pos);

        path.push((x as u8, y as u8));

        if pos == start {
            break;
        }

        if maze[Maze::moved_horizontally(pos, 1)] == u16::MAX {
            pos += 1;
        } else if maze[Maze::moved_horizontally(pos, -1)] == u16::MAX {
            pos -= 1;
        } else if maze[Maze::moved_vertically(pos, 1)] == u16::MAX {
            pos += Maze::width();
        } else if maze[Maze::moved_vertically(pos, -1)] == u16::MAX {
            pos -= Maze::width();
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

        let mut higher_index = 0;
        let remaining_path = &path[lower_index + MIN_SKIP_DISTANCE as usize..];
        while higher_index < remaining_path.len() {
            let higher_coords = remaining_path[higher_index];
            let distance = lower_coords.0.abs_diff(higher_coords.0) as usize + lower_coords.1.abs_diff(higher_coords.1) as usize;

            if distance <= CHEAT_DISTANCE && higher_index >= distance {
                sub_hundred += 1;
            }

            higher_index += 1.max(distance as isize - CHEAT_DISTANCE as isize) as usize;
        }
    }

    sub_hundred
}
