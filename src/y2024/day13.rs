use std::fs;

use crate::solutions;

solutions!{2024, 13}

const LEN_A: usize = "Button A: X+".len();
const LEN_B: usize = "Button B: X+".len();
const LEN_P: usize = "Prize: X=".len();

/// Holds the position of the A and B Buttons and the Price
type MachineLayout = ((u64, u64), (u64, u64), (u64, u64)); 

fn get_input(file: &str) -> Vec<MachineLayout> {
    fs::read_to_string(file).expect("No file there").split_terminator("\n\n").map(|block| {
        let split: Vec<&str> = block.split_terminator("\n").collect();
        let (ax, ay) = split[0][LEN_A..].split_once(", Y+").unwrap();
        let (bx, by) = split[1][LEN_B..].split_once(", Y+").unwrap();
        let (px, py) = split[2][LEN_P..].split_once(", Y=").unwrap();

        ((ax.parse().unwrap(), ay.parse().unwrap()), (bx.parse().unwrap(), by.parse().unwrap()), (px.parse().unwrap(), py.parse().unwrap()))
    }).collect()
}

/// Because of floating point incaccuracy we have a small margin of error
#[inline]
fn is_integer(num: f64) -> bool {
    (num - num.round()).abs() < 0.001
}

/// To calculate the cost of the total button presses we solve the system of equations
/// button_a * ax + button_b * bx = px
/// button_a * ay + button_b * by = py
/// 
/// Then we return button_a * 3 + button_b as the total token cost
/// 
/// If B is true, only solutions where each button is pressed less than 100 times are counted
/// O is an optional offset to the prize position in both directions
fn calculate_cost<const B: bool, const O: u64>(layout: &MachineLayout) -> u64 {
    let ((ax, ay), (bx, by), (px, py)) = layout;
    let ((ax, ay), (bx, by), (px, py)) = ((*ax as f64, *ay as f64), (*bx as f64, *by as f64), ((*px + O) as f64, (*py + O) as f64));
        
    let tt = ay * px;
    let bt = ay * bx;

    let t = tt / ax - py;
    let b = bt / ax - by;

    let button_b = t / b; 

    let at = px - button_b * bx;

    let button_a = at / ax;

    if B && (button_a > 100.0 || button_b > 100.0) {
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
}

/// ### Hacking the slot machine
/// 
/// We use the algorithm described above with 0 offset and the 100 press threshold and sum the results 
fn solve_first(input: &[MachineLayout]) -> u64 {
    input.iter().map(|layout| {
        calculate_cost::<true, 0>(layout)
    }).sum()
}

const OFFSET: u64 = 10000000000000;

/// ### Hacking the offset slot machine
/// 
/// We use the algorithm described above with a big offset and no press threshold and sum the results
fn solve_second(input: &[MachineLayout]) -> u64 {
    input.iter().map(|layout| {
        calculate_cost::<false, OFFSET>(layout)
    }).sum()
}
