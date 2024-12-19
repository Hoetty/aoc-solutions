use std::fs;

use crate::Solution;

type List = Vec<Vec<i32>>;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day2.txt");

    Solution::evaluated(
        "Day 2".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

fn get_input(file: &'static str) -> List {
    let content = fs::read_to_string(file).expect("No file there");
    content.lines().map(|l| l.split_whitespace().map(|s| s.parse::<i32>().expect("Input must be numeric")).collect::<Vec<i32>>()).collect::<List>()
}

fn solve_first(input: List) -> i32 {
    input.iter()
        .map(|report| report.windows(2).map(|a| a[0] - a[1]))
        .filter(|report| report.clone().all(|diff| diff.is_negative()) || report.clone().all(|diff| diff.is_positive()))
        .filter(|report| report.clone().all(|diff| diff != 0 && diff.abs() <= 3))
        .count() as i32
}

fn solve_second(input: List) -> i32 {
    let count_all = input.len();

    let bad: Vec<Vec<i32>> = input.iter()
        .map(|report| (report, report.windows(2).map(|a| a[0] - a[1])))
        .filter(|(_, diffs)| (!diffs.clone().all(|diff| diff.is_negative()) && !diffs.clone().all(|diff| diff.is_positive())) || !diffs.clone().all(|diff| diff != 0 && diff.abs() <= 3))
        .map(|(report, _)| report)
        .map(|v| v.clone())
        .collect();

    let mut bad_count = bad.len();

    for report in &bad {
        for (i, _) in report.iter().enumerate() {
            let mut new_report = report.clone();
            new_report.remove(i);

            let diffs = new_report.windows(2).map(|a| a[0] - a[1]);
            if (
                diffs.clone().all(|num| num.is_positive()) || 
                diffs.clone().all(|num| num.is_negative())
            ) && diffs.clone().all(|diff| diff != 0 && diff.abs() <= 3) {
                bad_count -= 1;
                break;
            }    
        }
    }

    return (count_all - bad_count) as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (i32, i32) {
        let file = fs::read_to_string("test-inputs/2024/day2-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day2.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day2.txt"));
        assert_eq!(result, expected().1);
    }
}