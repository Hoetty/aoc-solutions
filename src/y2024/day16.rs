use std::{collections::HashSet, fs::{self}, rc::Rc};

use fxhash::FxHashSet;

pub fn solutions() {
    let input = get_input("inputs/2024/day16.txt",);
    println!("2024 Day 16 #1: {}", solve_first(input.clone()));
    println!("2024 Day 16 #2: {}", solve_second(input));
}

#[derive(Clone)]
struct Maze {
    tiles: Vec<bool>,
    start: usize,
    end: usize,
    width: usize
}

fn get_input(file: &'static str) -> Maze {
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
    match direction as isize {
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
    let mut visited: FxHashSet<(usize, isize)> = FxHashSet::default();
    let mut newly_visited: FxHashSet<(usize, isize)> = FxHashSet::default();

    let mut current_round: Vec<PathPoint> = Vec::new();
    let mut next_round: Vec<PathPoint> = Vec::from([
        PathPoint {
            index: input.start, 
            direction: 1, 
            score: 0,
            last: None 
        }
    ]);


    loop {
        std::mem::swap(&mut current_round, &mut next_round);
        next_round.clear();

        let mut found: Option<usize> = None;
        let mut paths: Vec<PathPoint> = Vec::new();

        for state in current_round.clone() {
            let reference = Rc::new(state);

            let mut i = 0;
            loop {
                let current_pos = (reference.index as isize + i * reference.direction) as usize;
                
                if input.tiles[current_pos] {
                    break;
                }

                if current_pos == input.end {
                    let score = reference.score + i as usize;
                    match found {
                        Some(num) => if score <= num { 
                            found = Some(score);  

                            if score < num {
                                paths.clear();
                            }
                            
                            paths.push(PathPoint {
                                index: current_pos,
                                direction: reference.direction,
                                score,
                                last: Some(Rc::clone(&reference)),
                            });
                        },
                        None => {
                            found = Some(score);
                            paths.push(PathPoint {
                                index: current_pos,
                                direction: reference.direction,
                                score,
                                last: Some(Rc::clone(&reference)),
                            });
                        },
                    };
                }

                let right = right(reference.direction, input.width);
                let right_pos = (current_pos as isize + right) as usize;
                if !input.tiles[right_pos] {
                    if visited.insert((current_pos, right)) {
                        next_round.push(PathPoint {
                            index: current_pos,
                            direction: right,
                            score: reference.score + 1000 + i as usize,
                            last: Some(Rc::clone(&reference)),
                        });
                        newly_visited.insert((current_pos, right));
                    } else if newly_visited.contains(&(current_pos, right)) {
                        next_round.push(PathPoint {
                            index: current_pos,
                            direction: right,
                            score: reference.score + 1000 + i as usize,
                            last: Some(Rc::clone(&reference)),
                        });
                    }
                }

                let left = left(reference.direction, input.width);
                let left_pos = (current_pos as isize + left) as usize;
                if !input.tiles[left_pos] {
                    if visited.insert((current_pos, left)) {
                        next_round.push(PathPoint {
                            index: current_pos,
                            direction: left,
                            score: reference.score + 1000 + i as usize,
                            last: Some(Rc::clone(&reference)),
                        });
                        newly_visited.insert((current_pos, left));
                    } else if newly_visited.contains(&(current_pos, left)) {
                        next_round.push(PathPoint {
                            index: current_pos,
                            direction: left,
                            score: reference.score + 1000 + i as usize,
                            last: Some(Rc::clone(&reference)),
                        });
                    }
                }

                i += 1;
            }
        }

        if let Some(_) = found {
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

        newly_visited.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (usize, usize) {
        let file = fs::read_to_string("test-inputs/2024/day16-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day16.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day16.txt"));
        assert_eq!(result, expected().1);
    }
}