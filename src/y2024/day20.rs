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

    for character in file.chars() {
        if character == FINISH_CHAR {
            finish = maze.len();
        }

        if character != '\n' {
            maze.push(if character == WALL_CHAR { WALL } else { AIR });
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

    // Go through the maze and keep track of the path
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

    // Iterate through the entire path, except the last 100, where a good skip from a location further up is no longer possible
    // We walk through the path with a location where we skip to. For each location to skip to, we check the path from 100 tiles further up
    // for valid locations to jump from
    for (to_index, to_coords) in path.iter().enumerate().take(path.len() - MIN_SKIP_DISTANCE) {
        let mut from_index = 0;

        // The remaining path, where we can skip from, starts 100 tiles further up
        let remaining_path = &path[to_index + MIN_SKIP_DISTANCE..];


        while from_index < remaining_path.len() {
            let from_coords = remaining_path[from_index];
            // The distance of the two points is the manhattan distance
            let distance = to_coords.0.abs_diff(from_coords.0) + to_coords.1.abs_diff(from_coords.1);

            // If the distance between the points is greate than the index from the start of the remaining path,
            // then we can't skip as the extra distance we need to walk puts us under 100 net save
            if distance <= from_index {
                // We can always safely move at least the distance to the outer end of the diamond area, that marks where cheats are possible
                // From the inside we can be sure, that when we walk CHEAT_DISTANCE - distance + 1 tiles we will at most be on the rim of the area
                // Thus we don't need to count all tiles individually, and instead add them in bulk
                // When we are outside of the area we can walk at least CHEAT_DISTANCE - distance to end up at most right in front of the cheat area
                if distance <= CHEAT_DISTANCE {
                    // Safely walk inside the cheat area
                    good_cheats += CHEAT_DISTANCE - distance + 1;
                    from_index += CHEAT_DISTANCE - distance + 1;
                } else {
                    // Safely walk outside the cheat are
                    from_index += distance - CHEAT_DISTANCE;
                }
            } else {
                // If were are close to the lower end of the remaining path and can't skip yet, advance slowly
                from_index += 1;
            }
        }

        // It is possible to overshoot the remaining path while bulk processing the cheats
        if from_index > remaining_path.len() {
            let higher_coords = remaining_path.last().unwrap();
            let distance = to_coords.0.abs_diff(higher_coords.0) + to_coords.1.abs_diff(higher_coords.1);
            
            // If we overshot we need to check if the last point was a valid cheat
            if distance <= CHEAT_DISTANCE {
                // If the last point was counting cheats we need to subtract the overflow
                good_cheats -= from_index - remaining_path.len();
            }
        }

    }

    good_cheats
}
