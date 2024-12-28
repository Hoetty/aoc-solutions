use std::{fs, hash::Hash, num::NonZero, ops::{Add, Sub}, time::Instant, usize};

use rustc_hash::{FxBuildHasher, FxHashSet};
use smallvec::SmallVec;

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 6}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PointInfo {
    Air,
    Obstacle,
    OutOfMap
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
struct Vector2(i16, i16);

impl Vector2 {
    pub fn new(x: i16, y: i16) -> Vector2 {
        Vector2(x, y)
    }

    pub fn rotate_right(&self) -> Vector2 {
        Vector2::new(-self.1, self.0)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.0 - rhs.0, self.1 - rhs.1)
    }
}

// Saves a point and a direction, but hashes only the point
#[derive(Clone, Copy, Debug)]
struct StealthDirection(Vector2, Vector2);

impl Hash for StealthDirection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl PartialEq for StealthDirection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for StealthDirection {

}

#[derive(Clone)]
pub struct Map {
    dimensions: Vector2,
    obstacles_right: Vec<i16>,
    obstacles_left: Vec<i16>,
    obstacles_up: Vec<i16>,
    obstacles_down: Vec<i16>,
}

fn point_info_for_map(map: &Map, point: &Vector2) -> PointInfo {
    if point.0 < 0 || point.1 < 0 || point.0 >= map.dimensions.0 || point.1 >= map.dimensions.1 {
        return PointInfo::OutOfMap;
    }

    if map.obstacles_left[point.1 as usize * map.dimensions.0 as usize + point.0 as usize] == point.0 { PointInfo::Obstacle } else { PointInfo::Air }
}

fn next_position(map: &Map, start: &Vector2, direction: &Vector2, obstacle: &Vector2) -> Option<Vector2> {
    match direction {
        Vector2(0, 1) => {
            let obstacle_could_be_hit = start.0 == obstacle.0 && start.1 < obstacle.1;
            let next_y = map.obstacles_up[start.0 as usize + start.1 as usize * map.dimensions.0 as usize] as i16;

            if next_y == 0 {
                if obstacle_could_be_hit {
                    Some(Vector2(obstacle.0, obstacle.1 - 1))
                } else {
                    None
                }
            } else {
                if !obstacle_could_be_hit || next_y < obstacle.1 {
                    Some(Vector2(start.0, next_y - 1))
                } else {
                    Some(Vector2(obstacle.0, obstacle.1 - 1))
                }
            }
        },
        Vector2(0, -1) => {
            let obstacle_could_be_hit = start.0 == obstacle.0 && start.1 > obstacle.1;
            let next_y = map.obstacles_down[start.0 as usize + start.1 as usize * map.dimensions.0 as usize] as i16;

            if next_y == i16::MAX {
                if obstacle_could_be_hit {
                    Some(Vector2(obstacle.0, obstacle.1 + 1))
                } else {
                    None
                }
            } else {
                if !obstacle_could_be_hit || next_y > obstacle.1 {
                    Some(Vector2(start.0, next_y + 1))
                } else {
                    Some(Vector2(obstacle.0, obstacle.1 + 1))
                }
            }
        },
        Vector2(1, 0) => {
            let obstacle_could_be_hit = start.1 == obstacle.1 && start.0 < obstacle.0;
            let next_x = map.obstacles_right[start.1 as usize * map.dimensions.0 as usize + start.0 as usize] as i16;

            if next_x == 0 {
                if obstacle_could_be_hit {
                    Some(Vector2(obstacle.0 - 1, obstacle.1))
                } else {
                    None
                }
            } else {
                if !obstacle_could_be_hit || next_x < obstacle.0 {
                    Some(Vector2(next_x - 1, start.1))
                } else {
                    Some(Vector2(obstacle.0 - 1, obstacle.1))
                }
            }
        },
        Vector2(-1, 0) => {
            let obstacle_could_be_hit = start.1 == obstacle.1 && start.0 > obstacle.0;
            let next_x = map.obstacles_left[start.1 as usize * map.dimensions.0 as usize + start.0 as usize] as i16;

            if next_x == i16::MAX {
                if obstacle_could_be_hit {
                    Some(Vector2(obstacle.0 + 1, obstacle.1))
                } else {
                    None
                }
            } else {
                if !obstacle_could_be_hit || next_x > obstacle.0 {
                    Some(Vector2(next_x + 1, start.1))
                } else {
                    Some(Vector2(obstacle.0 + 1, obstacle.1))
                }
            }
        },
        _ => panic!()
    }
}


fn get_input(file: &str) -> (Vector2, Map) {
    let file = fs::read_to_string(file).expect("No file there");
    let lines = file.lines();

    let mut width = 0;
    let mut height = 0;

    let mut starting: Option<Vector2> = None;

    let mut obstacles_x = Vec::new();
    let mut obstacles_y = Vec::new();

    for (y, line) in lines.clone().enumerate() {
        height += 1;
        obstacles_y.push(Vec::new());
        for (x, char) in line.chars().enumerate() {
            if y == 0 {
                width += 1;
                obstacles_x.push(Vec::new());
            }
            if char == '#' {
                obstacles_x[x].push(y);
                obstacles_y[y].push(x);
            } else if char == '^' {
                starting = Some(Vector2(x as i16, y as i16));
            }
        }
    }

    let mut obstacles_right = vec![0; width * height];
    let mut obstacles_left = vec![i16::MAX; width * height];
    let mut obstacles_up = vec![0; height * width];
    let mut obstacles_down = vec![i16::MAX; height * width];

    for (y, lane) in obstacles_y.iter().enumerate() {
        let mut last = 0;
        for x in lane {
            for i in last..=*x {
                obstacles_right[y * width + i] = *x as i16;
            }
            last = x + 1;
        }
    }

    for (x, lane) in obstacles_x.iter().enumerate() {
        let mut last = 0;
        for y in lane {
            for i in last..=*y {
                obstacles_up[x + i * width] = *y as i16;
            }
            last = y + 1;
        }
    }

    for (y, lane) in obstacles_y.iter().enumerate() {
        let mut last = width;
        for x in lane.iter().rev() {
            for i in *x..last {
                obstacles_left[y * width + i] = *x as i16;
            }
            last = *x;
        }
    }

    for (x, lane) in obstacles_x.iter().enumerate() {
        let mut last = height;
        for y in lane.iter().rev() {
            for i in *y..last {
                obstacles_down[x + i * width] = *y as i16;
            }
            last = *y;
        }
    }

    (
        starting.unwrap(),
        Map {
            dimensions: Vector2::new(width as i16, height as i16),
            obstacles_right,
            obstacles_left,
            obstacles_down,
            obstacles_up
        }
    )
}

/// ### Path of the Guard
/// 
/// Walks the guard over the map, if the guard hits an obstacle, they turn right
/// Once they exit the map, the number of unique visited tiles is counted
fn solve_first(input: (Vector2, Map)) -> usize {
    let mut path: FxHashSet<Vector2> = FxHashSet::with_capacity_and_hasher(5000, FxBuildHasher);
    let (starting_position, map) = input;

    let mut position = starting_position;
    let mut direction = Vector2::new(0, -1);

    loop {
        path.insert(position);

        let next = position + direction;
        match point_info_for_map(&map, &next) {
            PointInfo::Air => position = next,
            PointInfo::Obstacle => direction = direction.rotate_right(),
            PointInfo::OutOfMap => break,
        }
    }

    path.len()
}

/// ### Infinite Loops
/// 
/// Searches how many obstacles could be placed on the guards path to create an infinite loop
fn solve_second(input: (Vector2, Map)) -> usize {
    let mut path: FxHashSet<StealthDirection> = FxHashSet::with_capacity_and_hasher(6000, FxBuildHasher);
    let (starting_position, map) = input;
    {
        let mut position = starting_position;
        let mut direction = Vector2::new(0, -1);
    
        loop {
            let next = position + direction;
            match point_info_for_map(&map, &next) {
                PointInfo::Air => {
                    path.insert(StealthDirection(next, direction));
                    position = next;
                },
                PointInfo::Obstacle => direction = direction.rotate_right(),
                PointInfo::OutOfMap => break,
            }
        }
    }

    let mut loop_obstacles = Vec::with_capacity(1024);

    for StealthDirection(potential_obstacle, starting_direction) in path {
        if potential_obstacle == starting_position {
            continue;
        }

        let mut trace_position = potential_obstacle - starting_direction;
        let mut trace_direction = starting_direction;
        let mut trace_visited: SmallVec<[i16; 32]> = SmallVec::new();
        
        let mut turns = 0;

        loop {
            let Some(trace_next) = next_position(&map, &trace_position, &trace_direction, &potential_obstacle) else {
                break;
            };

            trace_position = trace_next;

            if turns & 3 == 0 {
                let position_index = trace_position.0 + trace_position.1 * map.dimensions.0;

                if trace_visited.contains(&position_index) {
                        loop_obstacles.push(potential_obstacle);
                        break;
                } else {
                    trace_visited.push(position_index);
                }
            }

            turns += 1;
            trace_direction = trace_direction.rotate_right();
        }
    }

    loop_obstacles.len()
}
