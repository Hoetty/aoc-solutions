use std::{collections::HashSet, fs::{self}, hash::BuildHasherDefault, rc::Rc};

use fxhash::FxHashSet;

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 16}

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

fn right(direction: isize, width: usize) -> isize {
    match direction {
        1 => width as isize,
        _ if direction == width as isize => -1,
        -1 => -(width as isize),
        _ if direction == -(width as isize) => 1,
        dir => panic!("Invalid direction {dir}")
    }
}

fn left(direction: isize, width: usize) -> isize {
    -right(direction, width)
}

// Position, Direction, Length
type State = (usize, isize, usize);

fn solve_first(input: Maze) -> usize {
    let mut visited: FxHashSet<(usize, isize)> = FxHashSet::default();
    let mut newly_visited: FxHashSet<(usize, isize)> = FxHashSet::default();

    let mut current_round: FxHashSet<State> = FxHashSet::default();
    let mut next_round: FxHashSet<State> = FxHashSet::default();
    next_round.insert((input.start, 1, 0));

    loop {
        std::mem::swap(&mut current_round, &mut next_round);
        next_round.clear();

        let mut found: Option<usize> = None;

        for state in &current_round {
            let mut i = 0;
            loop {
                let current_pos = (state.0 as isize + i * state.1) as usize;

                if input.tiles[current_pos] {
                    break;
                }

                if current_pos == input.end {
                    let score = state.2 + i as usize;
                    match found {
                        Some(num) => if num > score { found = Some(score); },
                        None => found = Some(score),
                    };
                }

                let right = right(state.1, input.width);
                let right_pos = (current_pos as isize + right) as usize;
                if !input.tiles[right_pos] {
                    if visited.insert((current_pos, right)) {
                        next_round.insert((current_pos, right, state.2 + 1000 + i as usize));
                        newly_visited.insert((current_pos, right));
                    } else if newly_visited.contains(&(current_pos, right)) {
                        next_round.insert((current_pos, right, state.2 + 1000 + i as usize));
                    }
                }

                let left = left(state.1, input.width);
                let left_pos = (current_pos as isize + left) as usize;
                if !input.tiles[left_pos] {
                    if visited.insert((current_pos, left)) {
                        next_round.insert((current_pos, left, state.2 + 1000 + i as usize));
                        newly_visited.insert((current_pos, left));
                    } else if newly_visited.contains(&(current_pos, left)) {
                        next_round.insert((current_pos, left, state.2 + 1000 + i as usize));
                    }
                }
                i += 1;
            }
        }

        if let Some(score) = found {
            return score;
        }

        newly_visited.clear();
    }
}

#[derive(Clone, Hash)]
struct PathPoint {
    index: usize,
    direction: isize,
    score: usize,
    last: Option<Rc<PathPoint>>
}

fn solve_second(input: Maze) -> usize {
    let mut visited: FxHashSet<(usize, isize)> = FxHashSet::with_capacity_and_hasher(4096, BuildHasherDefault::default());
    let mut newly_visited: FxHashSet<(usize, isize)> = FxHashSet::with_capacity_and_hasher(256, BuildHasherDefault::default());

    let mut current_round: Vec<Rc<PathPoint>> = vec![];
    let mut next_round: Vec<Rc<PathPoint>> = vec![
        Rc::new(PathPoint {
            index: input.start, 
            direction: 1, 
            score: 0,
            last: None 
        })
    ];

    let mut paths: Vec<PathPoint> = Vec::with_capacity(256);

    loop {
        std::mem::swap(&mut current_round, &mut next_round);
        next_round.clear();

        let mut found: Option<usize> = None;

        for state in &current_round {
            let mut i = 0;
            loop {
                let current_pos = (state.index as isize + i * state.direction) as usize;
                
                if input.tiles[current_pos] {
                    break;
                }

                if current_pos == input.end {
                    let score = state.score + i as usize;
                    match found {
                        Some(num) => if score <= num { 
                            found = Some(score);  

                            if score < num {
                                paths.clear();
                            }
                            
                            paths.push(PathPoint {
                                index: current_pos,
                                direction: state.direction,
                                score,
                                last: Some(Rc::clone(&state)),
                            });
                        },
                        None => {
                            found = Some(score);
                            paths.push(PathPoint {
                                index: current_pos,
                                direction: state.direction,
                                score,
                                last: Some(Rc::clone(&state)),
                            });
                        },
                    };
                }

                let right = right(state.direction, input.width);
                let right_pos = (current_pos as isize + right) as usize;
                if !input.tiles[right_pos] {
                    if visited.insert((current_pos, right)) {
                        next_round.push(Rc::new(PathPoint {
                            index: current_pos,
                            direction: right,
                            score: state.score + 1000 + i as usize,
                            last: Some(Rc::clone(&state)),
                        }));
                        newly_visited.insert((current_pos, right));
                    } else if newly_visited.contains(&(current_pos, right)) {
                        next_round.push(Rc::new(PathPoint {
                            index: current_pos,
                            direction: right,
                            score: state.score + 1000 + i as usize,
                            last: Some(Rc::clone(&state)),
                        }));
                    }
                }

                let left = left(state.direction, input.width);
                let left_pos = (current_pos as isize + left) as usize;
                if !input.tiles[left_pos] {
                    if visited.insert((current_pos, left)) {
                        next_round.push(Rc::new(PathPoint {
                            index: current_pos,
                            direction: left,
                            score: state.score + 1000 + i as usize,
                            last: Some(Rc::clone(&state)),
                        }));
                        newly_visited.insert((current_pos, left));
                    } else if newly_visited.contains(&(current_pos, left)) {
                        next_round.push(Rc::new(PathPoint {
                            index: current_pos,
                            direction: left,
                            score: state.score + 1000 + i as usize,
                            last: Some(Rc::clone(&state)),
                        }));
                    }
                }

                i += 1;
            }
        }

        if found.is_some() {
            let mut path_tiles: FxHashSet<usize> = HashSet::default();

            for path in paths {
                let mut path = &path;

                loop {
                    if let Some(previous_path) = &path.last {
                        let mut i = 1;
                        loop {
                            let pos = (previous_path.index as isize + i * previous_path.direction) as usize;
                            path_tiles.insert(pos);

                            if pos == path.index {
                                break;
                            }

                            i += 1;
                        }

                        path = previous_path;
                    } else {
                        path_tiles.insert(path.index);
                        break;
                    }
                }
            }

            return path_tiles.len();
        }

        paths.clear();

        newly_visited.clear();
    }
}
