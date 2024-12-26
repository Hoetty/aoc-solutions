use std::{collections::HashSet, fs, hash::Hash, ops::{Add, Sub}};

use rustc_hash::{FxBuildHasher, FxHashSet};

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
struct Point(i16, i16);

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point(x, y)
    }

    pub fn rotate_right(&self) -> Point {
        Point::new(-self.1, self.0)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.0 - rhs.0, self.1 - rhs.1)
    }
}

// Saves a point and a direction, but hashes only the point
#[derive(Clone, Copy, Debug)]
struct StealthDirection(Point, Point);

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
    dimensions: Point,
    obstacles_x: Vec<Vec<usize>>,
    obstacles_y: Vec<Vec<usize>>
}

fn point_info_for_map(map: &Map, point: &Point) -> PointInfo {
    if point.0 < 0 || point.1 < 0 || point.0 >= map.dimensions.0 || point.1 >= map.dimensions.1 {
        return PointInfo::OutOfMap;
    }

    if map.obstacles_y[point.1 as usize].contains(&(point.0 as usize)) { PointInfo::Obstacle } else { PointInfo::Air }
}

fn next_position(map: &Map, start: &Point, direction: &Point, obstacle: Option<&Point>) -> Option<Point> {
    match direction {
        Point(0, 1) => {
            for y in &map.obstacles_x[start.0 as usize] {
                if *y > start.1 as usize {
                    return match obstacle {
                        Some(p) if p.0 == start.0 && p.1 < *y as i16 && p.1 > start.1 => Some(Point(p.0, p.1 - 1)),
                        _ => Some(Point(start.0, *y as i16 - 1))
                    };
                }
            }
            match obstacle {
                Some(p) if p.0 == start.0 && p.1 > start.1 => Some(Point(p.0, p.1 - 1)),
                _ => { None }
            }
        },
        Point(0, -1) => {
            for y in map.obstacles_x[start.0 as usize].iter().rev() {
                if *y < start.1 as usize {
                    return match obstacle {
                        Some(p) if p.0 == start.0 && p.1 > *y as i16 && p.1 < start.1 => Some(Point(p.0, p.1 + 1)),
                        _ => Some(Point(start.0, *y as i16 + 1))
                    };
                }
            }
            match obstacle {
                Some(p) if p.0 == start.0 && p.1 < start.1 => Some(Point(p.0, p.1 + 1)),
                _ => { None }
            }
        },
        Point(1, 0) => {
            for x in &map.obstacles_y[start.1 as usize] {
                if *x > start.0 as usize {
                    return match obstacle {
                        Some(p) if p.1 == start.1 && p.0 < *x as i16 && p.0 > start.0 => Some(Point(p.0 - 1, p.1)),
                        _ => Some(Point(*x as i16 - 1, start.1))
                    };
                }
            }
            match obstacle {
                Some(p) if p.1 == start.1 && p.0 > start.0 => Some(Point(p.0 - 1, p.1)),
                _ => { None }
            }
        },
        Point(-1, 0) => {
            for x in map.obstacles_y[start.1 as usize].iter().rev() {
                if *x < start.0 as usize {
                    return match obstacle {
                        Some(p) if p.1 == start.1 && p.0 > *x as i16 && p.0 < start.0 => Some(Point(p.0 + 1, p.1)),
                        _ => Some(Point(*x as i16 + 1, start.1))
                    };
                }
            }
            match obstacle {
                Some(p) if p.1 == start.1 && p.0 < start.0 => Some(Point(p.0 + 1, p.1)),
                _ => { None }
            }
        },
        _ => panic!()
    }
}

fn get_input(file: &str) -> (Point, Map) {
    let file = fs::read_to_string(file).expect("No file there");
    let lines = file.lines();

    let mut width = 0;
    let mut height = 0;

    let mut starting: Option<Point> = None;

    let mut obstacles_x = Vec::new();
    let mut obstacles_y = Vec::new();

    obstacles_x.push(Vec::new());
    obstacles_y.push(Vec::new());

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
                starting = Some(Point(x as i16, y as i16));
            }
        }
    }

    (
        starting.unwrap(),
        Map {
            dimensions: Point::new(width as i16, height as i16),
            obstacles_x,
            obstacles_y
        }
    )
}

fn solve_first(input: (Point, Map)) -> i32 {
    let mut visited: HashSet<Point> = HashSet::new();
    let (starting, map) = input;

    let mut position = starting;
    let mut direction = Point::new(0, -1);

    loop {
        visited.insert(position);

        let next = position + direction;
        match point_info_for_map(&map, &next) {
            PointInfo::Air => position = next,
            PointInfo::Obstacle => direction = direction.rotate_right(),
            PointInfo::OutOfMap => break,
        }
    }

    visited.len() as i32
}

fn solve_second(input: (Point, Map)) -> i32 {
    let mut visited: FxHashSet<StealthDirection> = FxHashSet::with_capacity_and_hasher(6000, FxBuildHasher);
    let (starting, map) = input;
    {
        let mut position = starting;
        let mut direction = Point::new(0, -1);
    
        loop {
            let next = position + direction;
            match point_info_for_map(&map, &next) {
                PointInfo::Air => {
                    visited.insert(StealthDirection(next, direction));
                    position = next;
                },
                PointInfo::Obstacle => direction = direction.rotate_right(),
                PointInfo::OutOfMap => break,
            }
        }
    }

    let mut obstacles = FxHashSet::with_capacity_and_hasher(2024, FxBuildHasher);

    for StealthDirection(potential_obstacle, starting_direction) in visited {
        if potential_obstacle == starting {
            continue;
        }

        let mut trace_position = potential_obstacle - starting_direction;
        let mut trace_direction = starting_direction;
        let mut trace_visited: FxHashSet<(Point, Point)> = FxHashSet::with_capacity_and_hasher(200, FxBuildHasher);

        loop {
            let trace_next = next_position(&map, &trace_position, &trace_direction, Some(&potential_obstacle));

            match trace_next {
                Some(p) => {
                    trace_position = p;
                    if !trace_visited.insert((trace_position, trace_direction)) {
                        obstacles.insert(potential_obstacle);
                        break;
                    }
                },
                None => break,
            }   

            trace_direction = trace_direction.rotate_right();
        }
    }

    obstacles.len() as i32
}
