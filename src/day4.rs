use std::{collections::HashSet, fs};

pub fn solutions() {
    let input = get_input();
    println!("Day 4, #1: {}", solve_first(input.clone()));
    println!("Day 4, #2: {}", solve_second(input.clone()));
}

pub fn get_input() -> Vec<Vec<char>> {
    return fs::read_to_string("inputs/day4.txt").expect("No file there").lines().map(|line| line.chars().collect()).collect();
}

pub fn solve_first(input: Vec<Vec<char>>) -> i32 {
    let height: i32 = input.len() as i32;
    let width: i32 = input[0].len() as i32;
    let mut num = 0;
    for y in 0..height {
        for x in 0..width {
            if input[y as usize][x as usize] == 'X' {
                for dy in (-1 as i32)..=1 {
                    for dx in (-1 as i32)..=1 {
                        if x + 3 * dx >= 0 && x + 3 * dx < width 
                            && y + 3 * dy >= 0 && y + 3 * dy < height
                            && input[(y + dy) as usize][(x + dx) as usize] == 'M'
                            && input[(y + 2 * dy) as usize][(x + 2 * dx) as usize] == 'A'
                            && input[(y + 3 * dy) as usize][(x + 3 * dx) as usize] == 'S' {
                                num += 1;
                            }
                    }
                }
            }
        }
    }

    num
}

pub fn solve_second(input: Vec<Vec<char>>) -> i32 {
    let height: i32 = input.len() as i32;
    let width: i32 = input[0].len() as i32;
    let sm: HashSet<char> = HashSet::from_iter(['S', 'M'].iter().cloned());
    let mut num = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if input[y as usize][x as usize] == 'A' {
                let tl = input[(y - 1) as usize][(x - 1) as usize];
                let tr = input[(y - 1) as usize][(x + 1) as usize];
                let bl = input[(y + 1) as usize][(x - 1) as usize];
                let br = input[(y + 1) as usize][(x + 1) as usize];

                if sm.intersection(&HashSet::from_iter([tl, br].iter().cloned())).count() == 2 &&
                    sm.intersection(&HashSet::from_iter([tr, bl].iter().cloned())).count() == 2 {
                    num += 1;       
                }
            }
        }
    }

    num
}