use std::{collections::VecDeque, fs::{self}};

use crate::{solutions, util::flatgrid::FlatGrid};

solutions!{2024, 16}

type Maze = FlatGrid<isize, 141, 141>;

/// The grid is represented by isizes. 0 means unexplored/start and isize::MIN means wall
/// The grid is painted so that all tiles on the path between start and end have the current score
fn get_input(file: &str) -> (Maze, usize, usize) {
    let file = fs::read_to_string(file).expect("No file there");
    
    let mut maze = Maze::new();
    let mut start = 0;
    let mut end = 0;

    for (y, line) in file.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            maze.push(if character == '#' { isize::MIN } else { 0 });

            if character == 'S' {
                start = Maze::to_index(x, y);
            } else if character == 'E' {
                end = Maze::to_index(x, y);
            }
        }
    }

    paint_maze(&mut maze, start, end);

    (maze, start, end)
}

/// Rotate the direction to the right
#[inline(always)]
fn right(direction: isize) -> isize {
    match direction {
        1 => Maze::width() as isize,
        _ if direction == Maze::width() as isize => -1,
        -1 => -(Maze::width() as isize),
        _ if direction == -(Maze::width() as isize) => 1,
        dir => panic!("Invalid direction {dir}")
    }
}

/// Rotate the direction to the left
#[inline(always)]
fn left(direction: isize) -> isize {
    -right(direction)
}

// Position, Direction, Length
#[derive(Debug, Clone, Copy)]
struct State(usize, isize, isize);

/// Paints the maze, so that all tiles on the optimal path have their score as value
/// Unexplored tiles are left at 0, walls at isize::MIN
fn paint_maze(maze: &mut Maze, start: usize, end: usize) {
    let mut queue = VecDeque::from([State(start, 1, 0)]);

    let mut found = false;

    while let Some(State(initial_position, direction, initial_score)) = queue.pop_front() {
        let mut i = 0;
        // For each position we walk in the current direction
        loop {
            let current_position = (initial_position as isize + i * direction) as usize;
            let current_score = initial_score + i;

            let inplace_score = maze[current_position];

            // If the tile is not unexplored and the current score is not less than the score already present,
            // we stop the loop here. This also stops in walls, as they are isize::MIN
            if inplace_score != 0 && current_score >= inplace_score {
                break;
            }

            // We update the current score
            maze[current_position] = current_score;

            // If we found the end, we can be sure that after this round no better path will be discoverd
            if current_position == end {
                found = true;
                break;
            }

            if !found {
                // If the end hasn't been found, we add possible turns to the end of the queue
                // This way, the queue is stacked so that the entries are sorted by the number of turns
                // This means, that we only need to check all positions of the current turn, once a solutions is found
                let turn_score = current_score + 1000 + 1;
    
                let right = right(direction);
                let right_position = (current_position as isize + right) as usize;
                let right_inplace = maze[right_position];
    
                if right_inplace == 0 || turn_score < right_inplace {
                    let right_state = State(right_position, right, turn_score);
                    queue.push_back(right_state);
                }
    
                let left = left(direction);
                let left_position = (current_position as isize + left) as usize;
                let left_inplace = maze[left_position];
                
                if left_inplace == 0 || turn_score < left_inplace {
                    let right_state = State(left_position, left, turn_score);
                    queue.push_back(right_state);
                }
            }


            i += 1;
        }
    }
}

/// ### Final Score
/// 
/// Get the score at the end position and return it
fn solve_first(input: &(Maze, usize, usize)) -> isize {
    input.0[input.2]
}

const VISITED: isize = -2;

/// ### Length of best Path Tiles
/// 
/// We step backwards the precalculated path and count all tiles that could form an optimal path
fn solve_second(input: &(Maze, usize, usize)) -> usize {
    let (maze, _, end) = input;
    let mut maze = maze.clone();

    let mut tiles = 0;

    let mut queue = Vec::from([*end]);

    while let Some(current_position) = queue.pop() {
        let current_value = maze[current_position];

        let right_value = maze[Maze::moved_horizontally(current_position, 1)];
        let left_value = maze[Maze::moved_horizontally(current_position, -1)];
        let top_value = maze[Maze::moved_vertically(current_position, 1)];
        let bottom_value = maze[Maze::moved_vertically(current_position, -1)];

        // The adjacent tile is part of the path, if it is one less, 1001 less or 999 more and the opposite tile has already been visited
        // 5 4 3 2 1
        //
        // 1005 1004
        //         3
        //         2
        //
        // 4005 3004 4003 3002
        //      3003      3001
        //      2002 2001 2000
        if right_value == current_value - 1 || right_value == current_value - 1001 || (right_value == current_value + 999 && left_value == VISITED) {
            queue.push(Maze::moved_horizontally(current_position, 1));
        }

        if left_value == current_value - 1 || left_value == current_value - 1001 || (left_value == current_value + 999 && right_value == VISITED) {
            queue.push(Maze::moved_horizontally(current_position, -1));
        }

        if top_value == current_value - 1 || top_value == current_value - 1001 || (top_value == current_value + 999 && bottom_value == VISITED) {
            queue.push(Maze::moved_vertically(current_position, 1));
        }

        if bottom_value == current_value - 1 || bottom_value == current_value - 1001 || (bottom_value == current_value + 999 && top_value == VISITED) {
            queue.push(Maze::moved_vertically(current_position, -1));
        }

        tiles += 1;

        // After processing the tile, we set it to visited as to not process it again
        maze[current_position] = VISITED;
    }

    tiles
}