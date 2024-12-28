use std::{fs, hash::Hash};

use rustc_hash::{FxBuildHasher, FxHashSet};

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 6}

/// Saves a point and a direction, but hashes only the point
/// This means when putting these into a hashset, the same point will be skipped,
///   if inserted a second time even for a different direction
#[derive(Clone, Copy, Debug)]
struct StealthDirection(usize, isize);

impl Hash for StealthDirection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.0);
    }
}

impl PartialEq for StealthDirection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for StealthDirection {

}

type DataCollection = (usize, Map, FxHashSet<usize>, FxHashSet<StealthDirection>);

#[derive(Clone)]
pub struct Map {
    width: usize,
    height: usize,
    obstacles_right: Vec<usize>,
    obstacles_left: Vec<usize>,
    obstacles_up: Vec<usize>,
    obstacles_down: Vec<usize>,
}

/// Checks if moving in the specified direction will result in the guard leaving the map
#[inline]
fn will_move_out_of_map(map: &Map, position: usize, direction: isize) -> bool {
    (x_coordinate(position, map.width) == 0 && direction == -1) ||
    (x_coordinate(position, map.width) == map.width - 1 && direction == 1) ||
    (y_coordinate(position, map.width) == 0 && direction < -1) ||
    (y_coordinate(position, map.width) == map.height - 1 && direction > 1) 
}

/// Checks if the specified position contains an obstacle. 
/// This is indicated by the list having its own x coordinate as next obstacle
#[inline]
fn is_obstacle(map: &Map, point: usize) -> bool {
    map.obstacles_left[point] == x_coordinate(point, map.width)
}

/// Splits the x coordinate off the index
#[inline(always)]
fn x_coordinate(position: usize, width: usize) -> usize {
    position - y_coordinate(position, width) * width
}

/// Splits the y coordinate off the index
#[inline(always)]
fn y_coordinate(position: usize, width: usize) -> usize {
    position / width
}

/// Calculates the next position in front of an obstacle, accepting another obstacle not currently on the map
fn next_position(map: &Map, start: usize, direction: isize, obstacle: usize) -> Option<usize> {
    let start_x = x_coordinate(start, map.width);
    let start_y = y_coordinate(start, map.width);
    let obstacle_x = x_coordinate(obstacle, map.width);
    let obstacle_y = y_coordinate(obstacle, map.width);

    if direction == -1 {
        let obstacle_could_be_hit = start_y == obstacle_y && start_x > obstacle_x;

        let next_x = map.obstacles_left[start];

        if next_x == usize::MAX {
            if obstacle_could_be_hit {
                Some(obstacle + 1)
            } else {
                None
            }
        } else if !obstacle_could_be_hit || next_x > obstacle_x {
            Some(start - start_x + next_x + 1)
        } else {
            Some(obstacle + 1)
        }
    } else if direction == 1 {
        let obstacle_could_be_hit = start_y == obstacle_y && start_x < obstacle_x;

        let next_x = map.obstacles_right[start];

        if next_x == 0 {
            if obstacle_could_be_hit {
                Some(obstacle - 1)
            } else {
                None
            }
        } else if !obstacle_could_be_hit || next_x < obstacle_x {
            Some(start - start_x + next_x - 1)
        } else {
            Some(obstacle - 1)
        }
    } else if direction < -1 {
        let obstacle_could_be_hit = start_x == obstacle_x && start_y > obstacle_y;

        let next_y = map.obstacles_down[start];

        if next_y == usize::MAX {
            if obstacle_could_be_hit {
                Some(obstacle + map.width)
            } else {
                None
            }
        } else if !obstacle_could_be_hit || next_y > obstacle_y {
            Some(start_x + (next_y + 1) * map.width)
        } else {
            Some(obstacle + map.width)
        }
    } else if direction > 1 {
        let obstacle_could_be_hit = start_x == obstacle_x && start_y < obstacle_y;

        let next_y = map.obstacles_up[start];

        if next_y == 0 {
            if obstacle_could_be_hit {
                Some(obstacle - map.width)
            } else {
                None
            }
        } else if !obstacle_could_be_hit || next_y < obstacle_y {
            Some(start_x + (next_y - 1) * map.width)
        } else {
            Some(obstacle - map.width)
        }
    } else {
        panic!()
    }
}


fn get_input(file: &str) -> DataCollection {
    let file = fs::read_to_string(file).expect("No file there");
    let lines: Vec<&str> = file.lines().collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut starting: usize = 0;

    // Collects all obstacles per axis
    let mut obstacles_x = vec![vec![]; width];
    let mut obstacles_y = vec![vec![]; height];

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                obstacles_x[x].push(y);
                obstacles_y[y].push(x);
            } else if char == '^' {
                starting = x + y * width;
            }
        }
    }

    // Stores the next obstacles coordinate on that axis for each direction
    let mut obstacles_right = vec![0; width * height];
    let mut obstacles_left = vec![usize::MAX; width * height];
    let mut obstacles_up = vec![0; height * width];
    let mut obstacles_down = vec![usize::MAX; height * width];

    for (y, row) in obstacles_y.iter().enumerate() {
        let mut last_right = 0;
        for x in row {
            for i in last_right..=*x {
                obstacles_right[y * width + i] = *x;
            }
            last_right = x + 1;
        }

        let mut last_left = width;
        for x in row.iter().rev() {
            for i in *x..last_left {
                obstacles_left[y * width + i] = *x;
            }
            last_left = *x;
        }
    }

    for (x, column) in obstacles_x.iter().enumerate() {
        let mut last_up = 0;
        for y in column {
            for i in last_up..=*y {
                obstacles_up[x + i * width] = *y;
            }
            last_up = y + 1;
        }

        let mut last_down = height;
        for y in column.iter().rev() {
            for i in *y..last_down {
                obstacles_down[x + i * width] = *y;
            }
            last_down = *y;
        }
    }

    let output = (
        starting,
        Map {
            width,
            height,
            obstacles_right,
            obstacles_left,
            obstacles_down,
            obstacles_up
        }
    );

    // Precalculate part 1 to reuse it in part 2
    let shared_path = path_and_directions(&output);

    (
        output.0,
        output.1,
        shared_path.0,
        shared_path.1
    )

    
}

/// Rotates the given direction 90° right
#[inline(always)]
fn rotate_right(map: &Map, direction: isize) -> isize {
    if direction == 1 {
        map.width as isize
    } else if direction == -1 {
        -(map.width as isize)
    } else if direction > 1 {
        -1
    } else if direction < -1 {
        1
    } else {
        panic!()
    }
}

/// Walks the guard over the map, if the guard hits an obstacle, they turn right
/// Once they exit the map, the number of unique visited tiles is counted
fn path_and_directions(input: &(usize, Map)) -> (FxHashSet<usize>, FxHashSet<StealthDirection>) {
    let mut path: FxHashSet<usize> = FxHashSet::with_capacity_and_hasher(5000, FxBuildHasher);
    let mut directions: FxHashSet<StealthDirection> = FxHashSet::with_capacity_and_hasher(5000, FxBuildHasher);
    let (starting_position, map) = input;

    let mut position = *starting_position;
    let mut direction = -(map.width as isize);

    loop {
        path.insert(position);
        directions.insert(StealthDirection(position, direction));

        if will_move_out_of_map(map, position, direction) {
            break;
        }

        let next = (position as isize + direction) as usize;
        
        if is_obstacle(map, next) {
            direction = rotate_right(map, direction);
        } else {
            position = next;
        }
    }

    (path, directions)
}

/// ### Path of the Guard
/// 
/// Uses the precomputed path and counts the tiles
/// For the algorithm look at path_and_directions
fn solve_first(input: &DataCollection) -> usize {
    let (_, _, path, _) = input;

    path.len()
}

/// ### Infinite Loops
/// 
/// Searches how many obstacles could be placed on the guards path to create an infinite loop
fn solve_second(input: &DataCollection) -> usize {
    let (starting_position, map, _, path) = input;

    let mut loop_obstacles = Vec::with_capacity(1024);
    let mut trace_visited: Vec<u16> = Vec::with_capacity(32);

    for &StealthDirection(potential_obstacle, starting_direction) in path {
        // Don't place an obstacle at the guards starting position
        if potential_obstacle == *starting_position {
            continue;
        }

        let mut trace_position = (potential_obstacle as isize - starting_direction) as usize;
        let mut trace_direction = starting_direction;
        trace_visited.clear();

        let mut turns = 0;

        // While on the map, skip straight in front of the next obstacle
        while let Some(trace_next) = next_position(map, trace_position, trace_direction, potential_obstacle) {
            trace_position = trace_next;

            // The guard can only be on a repeating position in the same direction every 4 turns - 360°, a full turn
            if turns & 3 == 0 {
                if trace_visited.contains(&(trace_position as u16)) {
                        loop_obstacles.push(potential_obstacle);
                        break;
                } else {
                    trace_visited.push(trace_position as u16);
                }
            }

            turns += 1;
            trace_direction = rotate_right(map, trace_direction);
        }
    }

    loop_obstacles.len()
}
