use std::fs;

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 5}

type Rules = Vec<Vec<bool>>;
type Orders = Vec<Vec<usize>>;

fn get_input(file: &str) -> (Rules, Orders) {
    let file = fs::read_to_string(file).expect("No file there");
    let (first, second) = file.split_once("\n\n").unwrap();

    let r = first.lines().map(|line| {
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
        second.lines().map(|line| line.split_terminator(",").map(|s| s.parse().unwrap()).collect()).collect()
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
