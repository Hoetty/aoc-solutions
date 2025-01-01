use std::{cmp::Ordering, collections::{HashSet, VecDeque}, fs::{self}, hash::Hash, io::stdin, rc::Rc};

use rustc_hash::{FxBuildHasher, FxHashSet};

use crate::{solutions, util::flatgrid::FlatGrid};

solutions!{2024, 16}

type Maze = FlatGrid<u8, 141, 141>;
type LastPoints = FlatGrid<StateSave, 141, 141>;

const WALL_MASK: u8 = 1;

fn get_input(file: &str) -> (Maze, usize, usize) {
    let file = fs::read_to_string(file).expect("No file there");
    
    let mut maze = Maze::new();
    let mut start = 0;
    let mut end = 0;

    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            maze.push((c == '#') as u8);

            if c == 'S' {
                start = Maze::to_index(x, y);
            } else if c == 'E' {
                end = Maze::to_index(x, y);
            }
        }
    }

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
fn direction_index(direction: isize) -> usize {
    match direction {
        1 => 1,
        _ if direction == Maze::width() as isize => 2,
        -1 => 3,
        _ if direction == -(Maze::width() as isize) => 4,
        dir => panic!("Invalid direction {dir}")
    }
}


#[inline(always)]
fn left(direction: isize) -> isize {
    -right(direction)
}

// Position, Direction, Length
#[derive(Eq, Debug, Clone, Copy)]
struct State(usize, isize, usize);

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

fn solve_first(input: &(Maze, usize, usize)) -> usize {
    let (maze, start, end) = input;

    let mut maze = maze.clone();

    let mut current_round: Vec<State> = Vec::default();
    let mut next_round: Vec<State> = Vec::default();
    next_round.push(State(*start, 1, 0));

    loop {
        std::mem::swap(&mut current_round, &mut next_round);
        next_round.clear();

        let mut found: Option<usize> = None;

        for &State(initial_position, direction, current_score) in &current_round {
            let mut i = 0;
            loop {
                let current_position = (initial_position as isize + i * direction) as usize;

                if maze[current_position] & WALL_MASK == 1 {
                    break;
                }

                if current_position == *end {
                    let score = current_score + i as usize;
                    match found {
                        Some(num) => if num > score { found = Some(score); },
                        None => found = Some(score),
                    };
                }

                let right = right(direction);
                let right_position = (current_position as isize + right) as usize;
                if maze[right_position] & WALL_MASK == 0 && maze[current_position] & (1 << direction_index(right)) == 0  {
                    let right_state = State(current_position, right, current_score + 1000 + i as usize);
                    if !next_round.contains(&right_state) {
                        next_round.push(right_state);
                    }
                }

                let left = left(direction);
                let left_position = (current_position as isize + left) as usize;
                if maze[left_position] & WALL_MASK == 0 && maze[current_position] & (1 << direction_index(left)) == 0 {
                    let left_state = State(current_position, left, current_score + 1000 + i as usize);
                    if !next_round.contains(&left_state) {
                        next_round.push(left_state);
                    }
                }

                i += 1;
            }
        }

        if let Some(score) = found {
            return score;
        }

        for &State(position, direction, _) in &next_round {
            maze[position] |= 1 << direction_index(direction);
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum StateSave {
    Zero,
    One(State),
    Two(State, State)
}

impl StateSave {
    fn upgrade(&mut self, other: &State) {
        *self = match self {
            StateSave::Zero => StateSave::One(*other),
            StateSave::One(state) if state.2 / 1000 < other.2 / 1000 => *self,
            StateSave::One(state) if state.2 == other.2 && *state == *other => *self,
            StateSave::One(state) if state.2 / 1000 == other.2 / 1000 => StateSave::Two(*state, *other),
            StateSave::One(_) => StateSave::One(*other),
            StateSave::Two(state, _) if state.2 / 1000 > other.2 / 1000 => StateSave::One(*other),
            StateSave::Two(state, _) if state.2 / 1000 <= other.2 / 1000 => *self,
            _ => panic!("Unupgradable {self:?} with updgrade {other:?}")
        }
    } 

    fn has_some(&self) -> bool {
        match self {
            StateSave::Zero => false,
            _ => true
        }
    }

    fn get_first(&self) -> State {
        match self {
            StateSave::Zero => panic!(),
            StateSave::One(state) => *state,
            StateSave::Two(state, _) => *state,
        }
    }

    fn pop(&mut self) -> StateSave {
        let value = *self;
        *self = StateSave::Zero;
        value
    }
}

impl Default for StateSave {
    fn default() -> Self {
        return Self::Zero
    }
}

fn solve_second(input: &(Maze, usize, usize)) -> usize {
    let (maze, start, end) = input;

    let mut maze = maze.clone();
    let mut last_points: LastPoints = FlatGrid::default();

    let mut current_round: Vec<State> = Vec::default();
    let mut next_round: Vec<State> = Vec::default();
    next_round.push(State(*start, 1, 0));

    loop {
        std::mem::swap(&mut current_round, &mut next_round);
        next_round.clear();

        let mut found: StateSave = StateSave::Zero;

        for initial_state in &current_round {
            let &State(initial_position, direction, current_score) = initial_state;
            let mut i = 0;
            loop {
                let current_position = (initial_position as isize + i * direction) as usize;

                if maze[current_position] & WALL_MASK == 1 {
                    break;
                }

                if current_position == *end {
                    let score = current_score + i as usize + 1000;
                    found.upgrade(&State(current_position, direction, score));
                }

                let right = right(direction);
                let right_position = (current_position as isize + right) as usize;
                if maze[right_position] & WALL_MASK == 0 && maze[current_position] & (1 << direction_index(right)) == 0  {
                    let right_state = State(current_position, right, current_score + 1000 + i as usize);
                    if !next_round.contains(&right_state) {
                        if current_position != initial_position {
                            last_points[current_position].upgrade(&initial_state);
                        }
                        next_round.push(right_state);
                    }
                }

                let left = left(direction);
                let left_position = (current_position as isize + left) as usize;
                if maze[left_position] & WALL_MASK == 0 && maze[current_position] & (1 << direction_index(left)) == 0 {
                    let left_state = State(current_position, left, current_score + 1000 + i as usize);
                    if !next_round.contains(&left_state) {
                        if current_position != initial_position {
                            last_points[current_position].upgrade(&initial_state);
                        }
                        next_round.push(left_state);
                    }
                }

                i += 1;
            }
        }

        if found.has_some() {
            let mut score = 1;

            let mut queue = VecDeque::from([found.get_first()]);

            while let Some(next) = queue.pop_front() {
                match last_points[next.0].pop() {
                    StateSave::Zero => { },
                    StateSave::One(state) => {
                        queue.push_back(state);

                        score += next.2 - state.2 - 1000;
                    },
                    StateSave::Two(state, state1) => {
                        queue.push_back(state);
                        queue.push_back(state1);

                        score += next.2 - state.2 - 1000;
                        score += next.2 - state1.2 - 1000;
                    },
                }
            }

            return score;
        }

        for &State(position, direction, _) in &next_round {
            maze[position] |= 1 << direction_index(direction);
        }
    }
}
