use std::fs;

use crate::{solutions, util::flatgrid::FlatGrid};

solutions!{2024, 20}

type Maze = FlatGrid<u16, 141, 141>;

const WALL_CHAR: char = '#';
const FINISH_CHAR: char = 'E';

const WALL: u16 = u16::MAX - 1;
const AIR: u16 = u16::MAX;

fn get_input(file: &str) -> (Maze, usize) {
    let file = fs::read_to_string(file).expect("No file there");
    
    let mut maze: Maze = Maze::new();
    let mut finish = 0;

    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            maze.push(if c == WALL_CHAR { WALL } else { AIR });

            if c == FINISH_CHAR {
                finish = Maze::to_index(x, y);
            }
        }
    }

    (maze, finish)
}

const MIN_SKIP_DISTANCE: usize = 100;
const MIN_DISTANCE_DIFFERENCE: u16 = MIN_SKIP_DISTANCE as u16 + 2;

/// ### Short Skips
/// Finds all skips through a single wall where the time save is at least 100
/// The algorithm starts at the finish and counts up the current distance, setting it to the maze
/// It then checks the four possible jump locations to see if their distance is at least 102 less, 
/// to also account for the distance travelled through the wall
fn solve_first(input: &(Maze, usize)) -> usize {
    let mut maze = input.0.clone();
    let finish = input.1;

    let mut distance = 0;
    let mut good_cheats = 0;

    // Start at the finish
    let mut pos = finish;

    loop {
        // Set the current locations value to its distance from the finish
        maze[pos] = distance;

        // Only check when at least a good cheat away from the finish
        if distance >= MIN_DISTANCE_DIFFERENCE {
            // For each direction check if the cheat saves at least 100 picoseconds
            if !Maze::will_horizontal_move_cross_border(pos, -2) && maze[Maze::moved_horizontally(pos, -2)] <= distance - MIN_DISTANCE_DIFFERENCE {
                good_cheats += 1;
            }

            if !Maze::will_horizontal_move_cross_border(pos, 2) && maze[Maze::moved_horizontally(pos, 2)] <= distance - MIN_DISTANCE_DIFFERENCE {
                good_cheats += 1;
            }

            if !Maze::will_vertical_move_cross_border(pos, -2) && maze[Maze::moved_vertically(pos, -2)] <= distance - MIN_DISTANCE_DIFFERENCE {
                good_cheats += 1;
            }

            if !Maze::will_vertical_move_cross_border(pos, 2) && maze[Maze::moved_vertically(pos, 2)] <= distance - MIN_DISTANCE_DIFFERENCE {
                good_cheats += 1;
            }
        }

        distance += 1;

        // Check where the next path tile is,
        // if there is none we reached the start
        if maze[Maze::moved_horizontally(pos, 1)] == AIR {
            pos += 1;
        } else if maze[Maze::moved_horizontally(pos, -1)] == AIR {
            pos -= 1;
        } else if maze[Maze::moved_vertically(pos, 1)] == AIR {
            pos += Maze::width();
        } else if maze[Maze::moved_vertically(pos, -1)] == AIR {
            pos -= Maze::width();
        } else {
            break;
        }
    }

    good_cheats
}

const VISITED: u16 = 0;
const CHEAT_DISTANCE: usize = 20;

/// ### Long Skips
/// 
/// Finds all cheats that travel at most 20 tiles and save at least 100 picoseconds
/// The entire path is precalculated from finish to start like in part one
/// Then the entire path is retraced, for each path tile the remaining path is scanned for skips
fn solve_second(input: &(Maze, usize)) -> usize {
    let mut maze = input.0.clone();
    let finish = input.1;

    let mut pos = finish;
    let mut path: Vec<(usize, usize)> = Vec::with_capacity(Maze::area() / 4);

    loop {
        maze[pos] = VISITED;

        path.push(Maze::to_coordinates(pos));

        if maze[Maze::moved_horizontally(pos, 1)] == AIR {
            pos += 1;
        } else if maze[Maze::moved_horizontally(pos, -1)] == AIR {
            pos -= 1;
        } else if maze[Maze::moved_vertically(pos, 1)] == AIR {
            pos += Maze::width();
        } else if maze[Maze::moved_vertically(pos, -1)] == AIR {
            pos -= Maze::width();
        } else {
            // The start position has been found, as the path has ended
            break;
        }
    }

    let mut good_cheats = 0;

    // Iterate through the entire path, except the last 100, where a good skip to a location further up is no longer possible
    for (lower_index, lower_coords) in path.iter().enumerate().take(path.len() - MIN_SKIP_DISTANCE as usize) {
        let mut higher_index = 0;
        let remaining_path = &path[lower_index + MIN_SKIP_DISTANCE as usize..];
        while higher_index < remaining_path.len() {
            let higher_coords = remaining_path[higher_index];
            let distance = lower_coords.0.abs_diff(higher_coords.0) as usize + lower_coords.1.abs_diff(higher_coords.1) as usize;

            if distance <= higher_index {
                if distance <= CHEAT_DISTANCE {
                    good_cheats += CHEAT_DISTANCE - distance + 1;
                    higher_index += CHEAT_DISTANCE - distance + 1;
                } else {
                    higher_index += distance - CHEAT_DISTANCE;
                }
            } else {
                higher_index += 1;
            }
        }

        let higher_coords = remaining_path.last().unwrap();
        let distance = lower_coords.0.abs_diff(higher_coords.0) as usize + lower_coords.1.abs_diff(higher_coords.1) as usize;

        if distance <= CHEAT_DISTANCE {
            good_cheats -= higher_index - remaining_path.len();
        }

    }

    good_cheats
}
