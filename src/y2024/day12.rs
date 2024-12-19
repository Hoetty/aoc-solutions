use std::{collections::VecDeque, fs, hash::BuildHasherDefault};

use fxhash::FxHashSet;

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day12.txt");

    Solution::evaluated(
        "Day 12".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

fn get_input(file: &'static str) -> Vec<Vec<char>> {
    fs::read_to_string(file).expect("No file there").lines().map(|l| l.chars().collect()).collect()
}

fn solve_first(input: Vec<Vec<char>>) -> u64 {
    let height = input.len();
    let width = input[0].len();

    let mut visited: FxHashSet<(u16, u16)> = FxHashSet::with_capacity_and_hasher(width * height, BuildHasherDefault::default());

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

fn solve_second(input: Vec<Vec<char>>) -> u64 {
    let height = input.len();
    let width = input[0].len();

    let mut visited: FxHashSet<(u16, u16)> = FxHashSet::with_capacity_and_hasher(width * height, BuildHasherDefault::default());

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

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (u64, u64) {
        let file = fs::read_to_string("test-inputs/2024/day12-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day12.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day12.txt"));
        assert_eq!(result, expected().1);
    }
}