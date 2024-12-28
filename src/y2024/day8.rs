use std::{collections::{HashMap, HashSet}, fs, ops::{Add, Sub}};

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 8}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }

    fn scaled(&self, scale: i32) -> Vector2 {
        Vector2::new(self.x * scale, self.y * scale)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

fn get_input(file: &str) -> (HashMap<char, Vec<Vector2>>, Vector2) {
    let file: Vec<Vec<char>> = fs::read_to_string(file).expect("No file there").lines().map(|l| l.chars().collect()).collect();

    let mut antennas = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, char_list) in file.iter().enumerate() {
        for (x, c) in char_list.iter().enumerate() {
            if y == 0 {
                width += 1;
            }
            if !c.is_alphanumeric() {
                continue;
            }

            if !antennas.contains_key(c) {
                antennas.insert(*c, Vec::new());
            }

            antennas.get_mut(c).unwrap().push(Vector2::new(x as i32, y as i32));
        }
        height += 1;
    }

    (antennas, Vector2::new(width, height))
}

fn solve_first(input: &(HashMap<char, Vec<Vector2>>, Vector2)) -> i32 {
    let mut stations: HashSet<Vector2> = HashSet::new();

    let (antennas, dimensions) = input;

    for locations in antennas.values() {
        for i in 0..locations.len() {
            for j in 0..i {
                let first = locations[i];
                let second = locations[j];

                let diff = first - second;
                stations.insert(first + diff);
                stations.insert(second - diff);
            }
        }
    }

    stations.iter().filter(|v| v.x >= 0 && v.y >= 0 && v.x < dimensions.x && v.y < dimensions.y).count() as i32
}

fn solve_second(input: &(HashMap<char, Vec<Vector2>>, Vector2)) -> i32 {
    let mut stations: HashSet<Vector2> = HashSet::new();

    let (antennas, dimensions) = input;

    for locations in antennas.values() {
        for i in 0..locations.len() {
            for j in 0..i {
                let first = locations[i];
                let second = locations[j];

                // This would include 2, 2 spacings narrowed down to 1, 1. This case is guaranteed not to happen
                // let diff = (*first - *second).scaled_down();
                let diff = first - second;
                let mut i = 0;
                loop {
                    let current = first + diff.scaled(i);
                    if !(current.x >= 0 && current.y >= 0 && current.x < dimensions.x && current.y < dimensions.y) {
                        break;
                    }

                    stations.insert(current);
                    i += 1;
                }

                let mut i = 0;
                loop {
                    let current = first + diff.scaled(i);
                    if !(current.x >= 0 && current.y >= 0 && current.x < dimensions.x && current.y < dimensions.y) {
                        break;
                    }

                    stations.insert(current);
                    i -= 1;
                }
            }
        }
    }

    stations.len() as i32
}
