use std::{collections::VecDeque, fs};

use rustc_hash::{FxBuildHasher, FxHashSet};

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 12}

fn get_input(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file).expect("No file there").lines().map(|l| l.chars().collect()).collect()
}

fn solve_first(input: &[Vec<char>]) -> u64 {
    let height = input.len();
    let width = input[0].len();

    let mut visited: FxHashSet<(u16, u16)> = FxHashSet::with_capacity_and_hasher(width * height, FxBuildHasher);

    let mut price = 0;

    for y in 0..height {
        for x in 0..width {
            if visited.contains(&(x as u16, y as u16)) {
                continue;
            }

            let mut queue = VecDeque::from([(x, y)]);
            let mut area = 0;
            let mut perimiter = 0;

            while !queue.is_empty() {
                let element = queue.pop_front().unwrap();

                let (x, y) = element;
                if !visited.insert((x as u16, y as u16)) {
                    continue;
                }

                area += 1;

                let current = input[y][x];

                if x > 0 && input[y][x - 1] == current {
                    queue.push_back((x - 1, y));
                } else {
                    perimiter += 1;
                }

                if x < width - 1 && input[y][x + 1] == current {
                    queue.push_back((x + 1, y));
                } else {
                    perimiter += 1;
                }

                if y > 0 && input[y - 1][x] == current {
                    queue.push_back((x, y - 1));
                } else {
                    perimiter += 1;
                }

                if y < width - 1 && input[y + 1][x] == current {
                    queue.push_back((x, y + 1));
                } else {
                    perimiter += 1;
                }

            }

            price += area * perimiter;
        }
    }

    price
}

fn solve_second(input: &[Vec<char>]) -> u64 {
    let height = input.len();
    let width = input[0].len();

    let mut visited: FxHashSet<(u16, u16)> = FxHashSet::with_capacity_and_hasher(width * height, FxBuildHasher);

    let mut price = 0;

    for y in 0..height {
        for x in 0..width {
            if visited.contains(&(x as u16, y as u16)) {
                continue;
            }

            let mut queue = VecDeque::from([(x, y)]);
            let mut area = 0;
            let mut perimiter = 0;

            while !queue.is_empty() {
                let element = queue.pop_front().unwrap();

                let (x, y) = element;
                if !visited.insert((x as u16, y as u16)) {
                    continue;
                }

                area += 1;

                let current = input[y][x];
                
                if x > 0 && input[y][x - 1] == current {
                    queue.push_back((x - 1, y));
                } else if y == 0 || (input[y - 1][x] != current || (x > 0 && input[y - 1][x - 1] == current)) {
                    perimiter += 1;
                }

                if x < width - 1 && input[y][x + 1] == current {
                    queue.push_back((x + 1, y));
                } else if y == 0 || (input[y - 1][x] != current || (x < width - 1 && input[y - 1][x + 1] == current)) {
                    perimiter += 1;
                }

                if y > 0 && input[y - 1][x] == current {
                    queue.push_back((x, y - 1));
                } else if x == 0 || (input[y][x - 1] != current || (y > 0 && input[y - 1][x - 1] == current)) {
                    perimiter += 1;
                }

                if y < width - 1 && input[y + 1][x] == current {
                    queue.push_back((x, y + 1));
                } else if x == 0 || (input[y][x - 1] != current || (y < height - 1 && input[y + 1][x - 1] == current)) {
                    perimiter += 1;
                }

            }

            price += area * perimiter;
        }
    }

    price
}
