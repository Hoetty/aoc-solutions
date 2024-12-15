use std::fs;

pub fn solutions() {
    let input = get_input("inputs/2024/day13.txt");
    println!("2024 Day 13 #1: {}", solve_first(input.clone()));
    println!("2024 Day 13 #2: {}", solve_second(input));
}

const LEN_A: usize = "Button A: X+".len();
const LEN_B: usize = "Button B: X+".len();
const LEN_P: usize = "Prize: X=".len();

fn get_input(file: &'static str) -> Vec<((u64, u64), (u64, u64), (u64, u64))> {
    fs::read_to_string(file).expect("No file there").split_terminator("\n\n").map(|block| {
        let split: Vec<&str> = block.split_terminator("\n").collect();
        let (ax, ay) = split[0][LEN_A..].split_once(", Y+").unwrap();
        let (bx, by) = split[1][LEN_B..].split_once(", Y+").unwrap();
        let (px, py) = split[2][LEN_P..].split_once(", Y=").unwrap();

        ((ax.parse().unwrap(), ay.parse().unwrap()), (bx.parse().unwrap(), by.parse().unwrap()), (px.parse().unwrap(), py.parse().unwrap()))
    }).collect()
}

#[inline]
fn is_integer(num: f64) -> bool {
    (num - num.round()).abs() < 0.001
}

fn solve_first(input: Vec<((u64, u64), (u64, u64), (u64, u64))>) -> u64 {
    input.iter().map(|((ax, ay), (bx, by), (px, py))| {
        let ((ax, ay), (bx, by), (px, py)) = ((*ax as f64, *ay as f64), (*bx as f64, *by as f64), (*px as f64, *py as f64));
        
        let tt = ay * px;
        let bt = ay * bx;

        let t = tt / ax - py;
        let b = bt / ax - by;

        let button_b = t / b; 

        let at = px - button_b * bx;

        let button_a = at / ax;

        if button_a > 100.0 || button_b > 100.0 {
            return 0;
        }

        if button_a < 0.0 || button_b < 0.0 {
            return 0;
        }

        let cost = button_a * 3.0 + button_b;

        if is_integer(button_a) && is_integer(button_b) {
            cost.round() as u64
        } else {
            0
        }
    }).sum()
}

fn solve_second(input: Vec<((u64, u64), (u64, u64), (u64, u64))>) -> u64 {
    input.iter().map(|((ax, ay), (bx, by), (px, py))| {
        let ((ax, ay), (bx, by), (px, py)) = ((*ax as f64, *ay as f64), (*bx as f64, *by as f64), (*px as f64 + 10000000000000.0, *py as f64 + 10000000000000.0));
        
        let tt = ay * px;
        let bt = ay * bx;

        let t = tt / ax - py;
        let b = bt / ax - by;

        let button_b = t / b; 

        let at = px - button_b * bx;

        let button_a = at / ax;

        if button_a < 0.0 || button_b < 0.0 {
            return 0;
        }

        let cost = button_a * 3.0 + button_b;

        if is_integer(button_a) && is_integer(button_b) {
            cost.round() as u64
        } else {
            0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (u64, u64) {
        let file = fs::read_to_string("test-inputs/2024/day13-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day13.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day13.txt"));
        assert_eq!(result, expected().1);
    }
}