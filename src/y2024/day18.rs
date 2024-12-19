use std::{collections::VecDeque, fs::{self}};

use crate::Solution;

const DIMENSIONS: (usize, usize) = (71, 71);

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day18.txt");

    Solution::evaluated(
        "Day 18".to_owned(), 
        &|| solve_first(input.clone(), DIMENSIONS),
        &|| solve_second(input.clone(), DIMENSIONS)
    )
}


fn get_input(file: &'static str) -> Vec<(u8, u8)> {
    fs::read_to_string(file).expect("No file there").lines().map(|l| l.split_once(",").unwrap()).map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap())).collect()
}

fn solve_n(n: usize, input: &Vec<(u8, u8)>, dimensions: (usize, usize)) -> i64
{
    let mut tiles = Vec::from([0]);
    tiles.resize(dimensions.0 * dimensions.1, i64::MAX);
    
    for &(x, y) in input.iter().take(n) {
        tiles[y as usize * dimensions.0 + x as usize] = -1;
    }

    let mut queue: VecDeque<usize> = VecDeque::from([0]);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let current_value = tiles[current];

        let right = current + 1;
        if right % dimensions.0 != 0 && tiles[right] > current_value + 1 {
            tiles[right] = current_value + 1;
            queue.push_back(right);
        }

        if current > 0 {
            let left = current - 1;
            if left % dimensions.0 != dimensions.0 - 1 && tiles[left] > current_value + 1 {
                tiles[left] = current_value + 1;
                queue.push_back(left);
            }
        }

        let top = current + dimensions.0;
        if top < dimensions.0 * dimensions.1 && tiles[top] > current_value + 1 {
            tiles[top] = current_value + 1;
            queue.push_back(top);
        }

        if current >= dimensions.0 {
            let bottom = current - dimensions.0;
            if tiles[bottom] > current_value + 1 {
                tiles[bottom] = current_value + 1;
                queue.push_back(bottom);
            }
        }
    }

    tiles[dimensions.0 * dimensions.1 - 1]
}

fn solve_first(input: Vec<(u8, u8)>, dimensions: (usize, usize)) -> i64 {
    solve_n(1024, &input, dimensions)
}

fn solve_second(input: Vec<(u8, u8)>, dimensions: (usize, usize)) -> (u8, u8) {
    let mut bottom = 0;
    let mut top = dimensions.0 * dimensions.1;

    while bottom != top - 1 {
        let i = (bottom + top) / 2;
        let n = solve_n(i, &input, dimensions);

        if n == i64::MAX {
            top = i;
        } else {
            bottom = i;
        }
    }

    input[bottom]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (i64, (u8, u8)) {
        let file = fs::read_to_string("test-inputs/2024/day18-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        let (l, r) = nums[1].split_once(",").unwrap();
        (nums[0].parse().unwrap(), (l.parse().unwrap(), r.parse().unwrap()))
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day18.txt"), DIMENSIONS);
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day18.txt"), DIMENSIONS);
        assert_eq!(result, expected().1);
    }
}