use std::fs;

type Rules = Vec<Vec<bool>>;
type Orders = Vec<Vec<usize>>;

pub fn solutions() {
    let input = get_input();
    println!("Day 5, #1: {}", solve_first(input.clone()));
    println!("Day 5, #2: {}", solve_second(input.clone()));
}

pub fn get_input() -> (Rules, Orders) {

    let r = fs::read_to_string("inputs/day5-1.txt").expect("No file there").lines().map(|line| {
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

    return (
        rules,
        fs::read_to_string("inputs/day5-2.txt").expect("No file there").lines().map(|line| line.split_terminator(",").map(|s| s.parse().unwrap()).collect()).collect()
    )
}

pub fn solve_first(input: (Rules, Orders)) -> i32 {
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

pub fn solve_second(input: (Rules, Orders)) -> i32 {
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

    return sum as i32;
}