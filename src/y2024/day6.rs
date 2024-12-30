use std::{fs, hash::Hash};

use rustc_hash::{FxBuildHasher, FxHashSet};

use crate::{solutions, util::flatgrid::FlatGrid};

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


const WIDTH: usize = 130;
const HEIGHT: usize = 130;

type Room = FlatGrid<u32, WIDTH, HEIGHT>;

type DataCollection = (usize, Room, FxHashSet<usize>, FxHashSet<StealthDirection>);

/// Checks if moving in the specified direction will result in the guard leaving the map
#[inline]
fn will_move_out_of_map(position: usize, direction: isize) -> bool {
    if direction.abs() > 1 {
        Room::will_vertical_move_cross_border(position, direction / Room::width() as isize)
    } else {
        Room::will_horizontal_move_cross_border(position, direction)
    }
}

/// Gets the next obstacle to the left of the current location
#[inline(always)]
fn next_left_obstacle(map: &Room, position: usize) -> usize {
    (map[position] >> 24) as usize 
}

/// Gets the next obstacle to the right of the current location
#[inline(always)]
fn next_right_obstacle(map: &Room, position: usize) -> usize {
    ((map[position] >> 16) & 0xFF) as usize 
}

/// Gets the next obstacle below the current location
#[inline(always)]
fn next_down_obstacle(map: &Room, position: usize) -> usize {
    ((map[position] >> 8) & 0xFF) as usize 
}

/// Gets the next obstacle above the current location
#[inline(always)]
fn next_up_obstacle(map: &Room, position: usize) -> usize {
    (map[position] & 0xFF) as usize 
}

/// Checks if the specified position contains an obstacle. 
/// This is indicated by the list having its own y coordinate as next obstacle
#[inline]
fn is_obstacle(map: &Room, point: usize) -> bool {
    next_up_obstacle(map, point) == Room::y_coordinate(point)
}

/// Calculates the next position in front of an obstacle, accepting another obstacle not currently on the map
fn next_position(map: &Room, start: usize, direction: isize, obstacle: usize) -> Option<usize> {
    let start_x = Room::x_coordinate(start);
    let start_y = Room::y_coordinate(start);
    let obstacle_x = Room::x_coordinate(obstacle);
    let obstacle_y = Room::y_coordinate(obstacle);

    if direction == -1 {
        let obstacle_could_be_hit = start_y == obstacle_y && start_x > obstacle_x;

        let next_x = next_left_obstacle(map, start);

        if next_x == 0xFF {
            if obstacle_could_be_hit {
                Some(Room::moved_horizontally(obstacle, 1))
            } else {
                None
            }
        } else if !obstacle_could_be_hit || next_x > obstacle_x {
            Some(start - start_x + next_x + 1)
        } else {
            Some(Room::moved_horizontally(obstacle, 1))
        }
    } else if direction == 1 {
        let obstacle_could_be_hit = start_y == obstacle_y && start_x < obstacle_x;

        let next_x = next_right_obstacle(map, start);

        if next_x == 0 {
            if obstacle_could_be_hit {
                Some(Room::moved_horizontally(obstacle, -1))
            } else {
                None
            }
        } else if !obstacle_could_be_hit || next_x < obstacle_x {
            Some(start - start_x + next_x - 1)
        } else {
            Some(Room::moved_horizontally(obstacle, -1))
        }
    } else if direction < -1 {
        let obstacle_could_be_hit = start_x == obstacle_x && start_y > obstacle_y;

        let next_y = next_down_obstacle(map, start);

        if next_y == 0xFF {
            if obstacle_could_be_hit {
                Some(Room::moved_vertically(obstacle, 1))
            } else {
                None
            }
        } else if !obstacle_could_be_hit || next_y > obstacle_y {
            Some(start_x + (next_y + 1) * Room::width())
        } else {
            Some(Room::moved_vertically(obstacle, 1))
        }
    } else if direction > 1 {
        let obstacle_could_be_hit = start_x == obstacle_x && start_y < obstacle_y;

        let next_y = next_up_obstacle(map, start);

        if next_y == 0 {
            if obstacle_could_be_hit {
                Some(Room::moved_vertically(obstacle, -1))
            } else {
                None
            }
        } else if !obstacle_could_be_hit || next_y < obstacle_y {
            Some(start_x + (next_y - 1) * Room::width())
        } else {
            Some(Room::moved_vertically(obstacle, -1))
        }
    } else {
        panic!()
    }
}

fn get_input(file: &str) -> DataCollection {
    let file = fs::read_to_string(file).expect("No file there");
    let lines: Vec<&str> = file.lines().collect();

    let mut starting: usize = 0;

    
    // Collects all obstacles per axis
    let mut obstacles_x = vec![vec![]; Room::width()];
    let mut obstacles_y = vec![vec![]; Room::height()];
    
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                obstacles_x[x].push(y);
                obstacles_y[y].push(x);
            } else if char == '^' {
                starting = Room::to_index(x, y);
            }
        }
    }
    
    // Stores the next obstacles coordinate on that axis for each direction
    let mut obstacles: Room = vec![0xFF00FF00; WIDTH * HEIGHT].into();

    for (y, row) in obstacles_y.iter().enumerate() {
        let mut last_left = Room::width();
        for x in row.iter().rev() {
            for i in *x..last_left {
                obstacles[Room::to_index(i, y)] = obstacles[Room::to_index(i, y)] & 0x00FFFFFF | (*x as u32) << 24;
            }
            last_left = *x;
        }

        let mut last_right = 0;
        for x in row {
            for i in last_right..=*x {
                obstacles[Room::to_index(i, y)] |= (*x as u32) << 16;
            }
            last_right = x + 1;
        }
    }

    for (x, column) in obstacles_x.iter().enumerate() {

        let mut last_down = Room::height();
        for y in column.iter().rev() {
            for i in *y..last_down {
                obstacles[Room::to_index(x, i)] = obstacles[Room::to_index(x, i)] & 0xFFFF00FF | (*y as u32) << 8;
            }
            last_down = *y;
        }

        let mut last_up = 0;
        for y in column {
            for i in last_up..=*y {
                obstacles[Room::to_index(x, i)] |= *y as u32;
            }
            last_up = y + 1;
        }        
    }

    let output = (
        starting,
        obstacles
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
fn rotate_right(direction: isize) -> isize {
    if direction == 1 {
        Room::width() as isize
    } else if direction == -1 {
        -(Room::width() as isize)
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
fn path_and_directions(input: &(usize, Room)) -> (FxHashSet<usize>, FxHashSet<StealthDirection>) {
    let mut path: FxHashSet<usize> = FxHashSet::with_capacity_and_hasher(5000, FxBuildHasher);
    let mut directions: FxHashSet<StealthDirection> = FxHashSet::with_capacity_and_hasher(5000, FxBuildHasher);
    let (starting_position, map) = input;

    let mut position = *starting_position;
    let mut direction = -(Room::width() as isize);

    loop {
        path.insert(position);
        directions.insert(StealthDirection(position, direction));

        if will_move_out_of_map(position, direction) {
            break;
        }

        let next = (position as isize + direction) as usize;
        
        if is_obstacle(map, next) {
            direction = rotate_right(direction);
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
            trace_direction = rotate_right(trace_direction);
        }
    }

    loop_obstacles.len()
}
