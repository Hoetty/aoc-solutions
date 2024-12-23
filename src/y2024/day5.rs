use std::fs;

use crate::Solution;

type Rules = Vec<Vec<bool>>;
type Orders = Vec<Vec<usize>>;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day5-1.txt", "inputs/2024/day5-2.txt");

    Solution::evaluated(
        "Day 5".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

fn get_input(file: &'static str, file2: &'static str) -> (Rules, Orders) {

    let r = fs::read_to_string(file).expect("No file there").lines().map(|line| {
        let (a, b) = line.split_once("|").unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }).collect::<Vec<(usize, usize)>>();

    let mut rules: Rules = Vec::new();

    for (left, right) in r {
        if rules.len() < (left + 1) {
            rules.resize(left + 1, Vec::new());
        }
        
        if rules[left].len() < (right + 1) {
            rules[left].resize(right + 1, false);
        }

        rules[left][right] = true;
    }

    (
        rules,
        fs::read_to_string(file2).expect("No file there").lines().map(|line| line.split_terminator(",").map(|s| s.parse().unwrap()).collect()).collect()
    )
}

fn solve_first(input: (Rules, Orders)) -> i32 {
    let (rules, orders) = input;

    let mut sum = 0;

    'outer: for order in orders {
        for i in 0..order.len() {
            for j in 0..i {
                if *rules[order[i]].get(order[j]).unwrap_or(&false) {
                    continue 'outer;
                }
            }
        }

        sum += order[order.len() / 2];
    }

    sum as i32
}

fn solve_second(input: (Rules, Orders)) -> i32 {
    let (rules, orders) = input;

    let mut sum = 0;

    for mut order in orders {
        let mut swapped = false;
        let mut i = 0;
        'outer: while i < order.len() {
            for j in 0..i {
                if *rules[order[i]].get(order[j]).unwrap_or(&false) {
                    order.swap(i, j);
                    swapped = true;
                    i -= 1;
                    continue 'outer;
                }
            }
            
            i += 1;
        }
        
        if swapped {
            sum += order[order.len() / 2];
        }
    }

    sum as i32
}
