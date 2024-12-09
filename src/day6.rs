use std::{collections::HashSet, fs, hash::Hash, ops::Add, thread};

pub fn solutions() {
    let input = get_input();
    println!("Day 6, #1: {}", solve_first(input.clone()));
    println!("Day 6, #2: {}", solve_second(input.clone()));
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PointInfo {
    Air,
    Obstacle,
    OutOfMap
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct Point {
    x: i32, 
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }

    pub fn rotate_right(&self) -> Point {
        return Point::new(-self.y, self.x);
    }

    pub fn mirrored(&self) -> Point {
        return Point::new(-self.x, -self.y);
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        return Point::new(self.x + rhs.x, self.y + rhs.y);
    }
}

#[derive(Clone)]
pub struct Map {
    dimensions: Point,
    obstacles: Vec<Vec<i32>>
}


fn point_info_for_map(map: &Map, point: &Point) -> PointInfo {
    if point.x < 0 || point.y < 0 || point.x >= map.dimensions.x || point.y >= map.dimensions.y {
        return PointInfo::OutOfMap;
    }

    if map.obstacles[point.y as usize].contains(&point.x) { PointInfo::Obstacle } else { PointInfo::Air }
}

pub fn get_input() -> (Point, Map) {
    let file = fs::read_to_string("inputs/day6.txt").expect("No file there");
    let lines = file.lines();

    let mut width = 0;
    let mut height = 0;

    let mut starting: Option<Point> = None;

    for (y, line) in lines.clone().enumerate() {
        height += 1;
        for (x, char) in line.chars().enumerate() {
            if y == 0 {
                width += 1;
            }
            if char == '^' {
                starting = Some(Point {x: x as i32, y: y as i32});
            }
        }
    }

    return (
        starting.unwrap(),
        Map {
            dimensions: Point::new(width, height),
            obstacles: lines.map(|l| l.chars().enumerate().filter(|(_, e)| *e == '#').map(|(i, _)| i as i32).collect()).collect()
        }
    );
}

pub fn solve_first(input: (Point, Map)) -> i32 {
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

pub fn solve_second(input: (Point, Map)) -> i32 {
    let mut visited: HashSet<Point> = HashSet::new();
    let (starting, map) = input;

    let mut position = starting;
    let mut direction = Point::new(0, -1);

    loop {
        let next = position + direction;
        match point_info_for_map(&map, &next) {
            PointInfo::Air => position = next,
            PointInfo::Obstacle => direction = direction.rotate_right(),
            PointInfo::OutOfMap => break,
        }

        visited.insert(position);
    }

    let mut threads = Vec::new();

    for potential_obstacle in visited {
        let map = map.clone();
        let job = thread::spawn(move || {
            if potential_obstacle == starting {
                return None;
            }
    
            let mut trace_position = starting;
            let mut trace_direction = Point::new(0, -1);
            let mut trace_visited: HashSet<(Point, Point)> = HashSet::new();
    
            loop {
                let trace_next = trace_position + trace_direction;
                match point_info_for_map(&map, &trace_next) {
                    PointInfo::Air if trace_next != potential_obstacle => trace_position = trace_next,
                    PointInfo::Air | PointInfo::Obstacle => trace_direction = trace_direction.rotate_right(),
                    PointInfo::OutOfMap => {
                        return None;
                    },
                }
    
                if !trace_visited.insert((trace_position, trace_direction)) {
                    return Some(potential_obstacle);
                }
            }
        });

        threads.push(job);
    }

    let mut obstacles = HashSet::new();

    for t in threads {
        match t.join().unwrap() {
            Some(p) => {obstacles.insert(p);},
            None => (),
        }
    }

    obstacles.len() as i32
}