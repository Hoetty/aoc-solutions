use std::{collections::VecDeque, fs::{self}};

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 18}

const DIMENSIONS: (usize, usize) = (71, 71);

fn get_input(file: &str) -> Vec<(u8, u8)> {
    fs::read_to_string(file).expect("No file there").lines().map(|l| l.split_once(",").unwrap()).map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap())).collect()
}

fn solve_n(n: usize, input: &[(u8, u8)], dimensions: (usize, usize)) -> i64
{
    let mut tiles = vec![i64::MAX; dimensions.0 * dimensions.1];
    tiles[0] = 0;
    
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

fn solve_first(input: &[(u8, u8)]) -> i64 {
    solve_n(1024, input, DIMENSIONS)
}

fn solve_second(input: &[(u8, u8)]) -> String {
    let mut bottom = 0;
    let mut top = DIMENSIONS.0 * DIMENSIONS.1;

    while bottom != top - 1 {
        let i = (bottom + top) / 2;
        let n = solve_n(i, input, DIMENSIONS);

        if n == i64::MAX {
            top = i;
        } else {
            bottom = i;
        }
    }

    let coord = input[bottom];

    format!("{},{}", coord.0, coord.1)
}
