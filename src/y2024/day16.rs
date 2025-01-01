use std::fs::{self};

use crate::{solutions, util::flatgrid::FlatGrid};

solutions!{2024, 16}

type Maze = FlatGrid<isize, 141, 141>;

fn get_input(file: &str) -> (Maze, usize, usize) {
    let file = fs::read_to_string(file).expect("No file there");
    
    let mut maze = Maze::new();
    let mut start = 0;
    let mut end = 0;

    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            maze.push(if c == '#' { isize::MIN } else { 0 });

            if c == 'S' {
                start = Maze::to_index(x, y);
            } else if c == 'E' {
                end = Maze::to_index(x, y);
            }
        }
    }

    paint_maze(&mut maze, start, end);

    (maze, start, end)
}

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

#[inline(always)]
fn left(direction: isize) -> isize {
    -right(direction)
}

// Position, Direction, Length
#[derive(Eq, Debug, Clone, Copy)]
struct State(usize, isize, isize);

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

fn paint_maze(maze: &mut Maze, start: usize, end: usize) {
    let mut current_round: Vec<State> = Vec::default();
    let mut next_round: Vec<State> = Vec::default();
    next_round.push(State(start, 1, 0));

    loop {
        std::mem::swap(&mut current_round, &mut next_round);
        next_round.clear();

        let mut found: Option<isize> = None;

        for &State(initial_position, direction, initial_score) in &current_round {
            let mut i = 0;
            loop {
                let current_position = (initial_position as isize + i * direction) as usize;
                let current_score = initial_score + i;

                let inplace_score = maze[current_position];

                if inplace_score != 0 && current_score >= inplace_score {
                    break;
                }

                maze[current_position] = current_score;

                if current_position == end {
                    found = Some(current_score);
                    break;
                }

                let turn_score = current_score + 1000 + 1;

                let right = right(direction);
                let right_position = (current_position as isize + right) as usize;
                let right_inplace = maze[right_position];

                if right_inplace == 0 || turn_score < right_inplace {
                    let right_state = State(right_position, right, turn_score);
                    if !next_round.contains(&right_state) {
                        next_round.push(right_state);
                    }
                }

                let left = left(direction);
                let left_position = (current_position as isize + left) as usize;
                let left_inplace = maze[left_position];
                
                if left_inplace == 0 || turn_score < left_inplace {
                    let right_state = State(left_position, left, turn_score);
                    if !next_round.contains(&right_state) {
                        next_round.push(right_state);
                    }
                }

                i += 1;
            }
        }

        if found.is_some() {
            return;
        }
    }
}

fn solve_first(input: &(Maze, usize, usize)) -> isize {
    input.0[input.2]
}

const VISITED: isize = -2;

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
        maze[current_position] = VISITED;
    }

    tiles
}