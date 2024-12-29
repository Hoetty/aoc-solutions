use std::{fs, ops::{Add, Sub}};

use rustc_hash::{FxHashMap, FxHashSet};

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

fn get_input(file: &str) -> (FxHashMap<char, Vec<Vector2>>, Vector2) {
    let file: Vec<Vec<char>> = fs::read_to_string(file)
        .expect("No file there")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antennas = FxHashMap::default();
    let mut width = 0;
    let height = file.len();

    for (y, char_list) in file.iter().enumerate() {
        if y == 0 {
            width = char_list.len();
        }

        for (x, antenna) in char_list.iter().enumerate() {
            if !antenna.is_alphanumeric() {
                continue;
            }

            antennas.entry(*antenna)
                .or_insert(Vec::new())
                .push(Vector2::new(x as i32, y as i32))
        }
    }

    (antennas, Vector2::new(width as i32, height as i32))
}

/// Checks if the position is still in bounds
#[inline]
fn in_bounds(position: &Vector2, dimensions: &Vector2) -> bool {
    position.x >= 0 && position.y >= 0 && position.x < dimensions.x && position.y < dimensions.y
}

/// ### Antenna Antinodes
/// 
/// For each pair of antennas of the same pair, calculate the antinode locations
/// Return the number of unique antinode locations that are in bounds
fn solve_first(input: &(FxHashMap<char, Vec<Vector2>>, Vector2)) -> usize {
    let mut stations: FxHashSet<Vector2> = FxHashSet::default();

    let (antennas, dimensions) = input;

    for locations in antennas.values() {
        for (i, first_location) in locations.iter().enumerate() {
            for second_location in locations.iter().take(i) {
                // Calculate the vector from the second to the first station
                // S -> F
                // Add this vector to the first stations position
                // S    F -> I
                // And subtract it from the second station
                // I <- S    F
                // These are the antinode locations
                let second_to_first = *first_location - *second_location;
                stations.insert(*first_location + second_to_first);
                stations.insert(*second_location - second_to_first);
            }
        }
    }

    stations
        .iter()
        .filter(|station| in_bounds(station, dimensions))
        .count()
}

/// ### Antenna Antinode Rows
/// 
/// For each pair of antennas of the same pair, calculate the rows of antinode locations they invoke
/// Return the number of unique antinode locations that are in bounds
fn solve_second(input: &(FxHashMap<char, Vec<Vector2>>, Vector2)) -> usize {
    let mut stations: FxHashSet<Vector2> = FxHashSet::default();

    let (antennas, dimensions) = input;

    for locations in antennas.values() {
        for (i, first_location) in locations.iter().enumerate() {
            for second_location in locations.iter().take(i) {
                let second_to_first = *first_location - *second_location;
                let mut direction = 1;
                let mut i = 0;
                loop {
                    let current_antinode = *first_location + second_to_first.scaled(i);
                    let not_in_bounds = !in_bounds(&current_antinode, dimensions);

                    if direction == 1 && not_in_bounds {
                        // If the first direction has been plotted, continue with the second
                        direction = -1;
                        i = 0;
                        continue;
                    } else if direction == -1 && not_in_bounds {
                        break;
                    }

                    stations.insert(current_antinode);
                    i += direction;
                }
            }
        }
    }

    stations.len()
}
