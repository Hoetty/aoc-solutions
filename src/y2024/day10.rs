use std::{collections::VecDeque, fs, hash::BuildHasherDefault};

use fxhash::FxHashSet;

pub fn solutions() {
    let input = get_input("inputs/2024/day10.txt");
    println!("2024 Day 10 #1: {}", solve_first(input.clone()));
    println!("2024 Day 10 #2: {}", solve_second(input));
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
struct Spot {
    x: i16,
    y: i16
}

impl Spot {
    pub fn new(x: i16, y: i16) -> Spot {
        Spot { x, y }
    }
}

fn get_input(file: &'static str) -> (Vec<Vec<u8>>, Vec<Spot>) {
    let mut starting = Vec::new();
    let mut map = Vec::new();

    for (y, line) in fs::read_to_string(file).expect("No file there").lines().enumerate() {
        map.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            let level = (c as u8) - 48;
            map[y].push(level);

            if level == 0 {
                starting.push(Spot::new(x as i16, y as i16));
            }
        }
    }

    (map, starting)
}

fn solve_first(input: (Vec<Vec<u8>>, Vec<Spot>)) -> u64 {
    let (map, starting) = input;

    let width = map[0].len();
    let height = map.len();
    let mut sum = 0;

    for base in starting {
        let mut heads: FxHashSet<Spot> = FxHashSet::with_capacity_and_hasher(10, BuildHasherDefault::default());
        let mut queue = VecDeque::from([base]);

        while queue.len() > 0 {
            let current = queue.pop_front().unwrap();
            let value = map[current.y as usize][current.x as usize];

            if value == 9 {
                heads.insert(current);
                continue;
            }

            if (current.x as usize) < width - 1 && map[current.y as usize][current.x as usize + 1] == value + 1 {
                queue.push_back(Spot::new(current.x + 1, current.y));
            }

            if (current.x as usize) > 0 && map[current.y as usize][current.x as usize - 1] == value + 1 {
                queue.push_back(Spot::new(current.x - 1, current.y));
            }

            if (current.y as usize) < height - 1 && map[current.y as usize + 1][current.x as usize] == value + 1 {
                queue.push_back(Spot::new(current.x, current.y + 1));
            }

            if (current.y as usize) > 0 && map[current.y as usize - 1][current.x as usize] == value + 1 {
                queue.push_back(Spot::new(current.x, current.y - 1));
            }
        }

        sum += heads.len();
    }

    sum as u64
}

fn solve_second(input: (Vec<Vec<u8>>, Vec<Spot>)) -> u64 {
    let (map, starting) = input;

    let width = map[0].len();
    let height = map.len();
    let mut sum = 0;

    for base in starting {
        let mut queue = VecDeque::from([base]);

        while queue.len() > 0 {
            let current = queue.pop_front().unwrap();
            let value = map[current.y as usize][current.x as usize];

            if value == 9 {
                sum += 1;
                continue;
            }

            if (current.x as usize) < width - 1 && map[current.y as usize][current.x as usize + 1] == value + 1 {
                queue.push_back(Spot::new(current.x + 1, current.y));
            }

            if (current.x as usize) > 0 && map[current.y as usize][current.x as usize - 1] == value + 1 {
                queue.push_back(Spot::new(current.x - 1, current.y));
            }

            if (current.y as usize) < height - 1 && map[current.y as usize + 1][current.x as usize] == value + 1 {
                queue.push_back(Spot::new(current.x, current.y + 1));
            }

            if (current.y as usize) > 0 && map[current.y as usize - 1][current.x as usize] == value + 1 {
                queue.push_back(Spot::new(current.x, current.y - 1));
            }
        }
    }

    sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (u64, u64) {
        let file = fs::read_to_string("test-inputs/2024/day10-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day10.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day10.txt"));
        assert_eq!(result, expected().1);
    }
}