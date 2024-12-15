use std::{collections::HashMap, fs};

type List = (Vec<i32>, Vec<i32>);

pub fn solutions() {
    let input = get_input("inputs/2024/day1.txt");
    println!("2024 Day 1 #1: {}", solve_first(input.clone()));
    println!("2024 Day 1 #2: {}", solve_second(input));
}

fn get_input(file: &'static str) -> List {
    let content = fs::read_to_string(file).expect("No file there");

    let (first, second): List = content
        .lines()
        .map(|line| line.split_once("   ").expect("Must be splittable"))
        .map(|(s1, s2)| (s1.parse::<i32>().expect("Must be number"), s2.parse::<i32>().expect("Must be a number")))
        .unzip();

    (first, second)
}

fn solve_first(input: List) -> i32 {

    let (mut first, mut second) = input;

    first.sort();
    second.sort();

    first.iter()
        .zip(second.iter())
        .map(|(first, second)| (first - second).abs())
        .sum()
}

fn solve_second(input: List) -> i32 {
    let (first, second) = input;

    let mut frequency: HashMap<i32, i32> = HashMap::new();

    for num in second {
        match frequency.get_mut(&num) {
            Some(count) => { *count = *count + 1; },
            None => { frequency.insert(num, 1); }
        }
    }

    first.iter()
        .map(|i| i * frequency.get(i).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (i32, i32) {
        let file = fs::read_to_string("test-inputs/2024/day1-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day1.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day1.txt"));
        assert_eq!(result, expected().1);
    }
}