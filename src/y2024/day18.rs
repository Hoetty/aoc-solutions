use std::{collections::VecDeque, fs};

use crate::{solutions, util::flatgrid::FlatGrid};

solutions!{2024, 18}

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

type MemorySpace = FlatGrid<i16, WIDTH, HEIGHT>;

fn get_input(file: &str) -> Vec<(u8, u8)> {
    fs::read_to_string(file)
        .expect("No file there")
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect()
}

// Calculates the required steps to go from the top left to the bottom right after n bytes have fallen
// The entire memory space is walked, setting each squares value to the number of steps taken
fn steps_after(n: usize, input: &[(u8, u8)]) -> i16
{
    let mut space = MemorySpace::filled(i16::MAX);
    space[0] = 0;
    
    // Setting the corrupted regions to -1 ensures the walking algorithm never goes there
    // as it only visits spots with a higher score than the current score
    for &(x, y) in input.iter().take(n) {
        space[MemorySpace::to_index(x as usize, y as usize)] = -1;
    }

    // Start at the top left (0)
    let mut queue: VecDeque<usize> = VecDeque::from([0]);

    while let Some(current_index) = queue.pop_front() {
        let current_value = space[current_index];
        let next_value = current_value + 1;

        if !MemorySpace::will_horizontal_move_cross_border(current_index, 1) {
            let right = MemorySpace::moved_horizontally(current_index, 1);
            if space[right] > next_value {
                space[right] = next_value;
                queue.push_back(right);
            }
        }

        if !MemorySpace::will_horizontal_move_cross_border(current_index, -1) {
            let left = MemorySpace::moved_horizontally(current_index, -1);
            if space[left] > next_value {
                space[left] = next_value;
                queue.push_back(left);
            }
        }

        if !MemorySpace::will_vertical_move_cross_border(current_index, 1) {
            let top = MemorySpace::moved_vertically(current_index, 1);
            if space[top] > next_value {
                space[top] = next_value;
                queue.push_back(top);
            }
        }
        

        if !MemorySpace::will_vertical_move_cross_border(current_index, -1) {
            let bottom = MemorySpace::moved_vertically(current_index, -1);
            if space[bottom] > next_value {
                space[bottom] = next_value;
                queue.push_back(bottom);
            }
        }
    }

    space[MemorySpace::last_index()]
}

/// ### Steps after one Kilobyte
/// 
/// Calculates the number of steps needed to reach the bottom right 
/// after 1024 bytes have fallen
fn solve_first(input: &[(u8, u8)]) -> i16 {
    steps_after(1024, input)
}

/// ### Last byte to the finish
/// 
/// Calculate the last byte when the finish line is reachable
/// The next byte blocks the path, indicated by needing i16::MAX steps
fn solve_second(input: &[(u8, u8)]) -> String {
    let first_blocked_index = (0..input.len())
        .map(|bytes_fallen| move || steps_after(bytes_fallen, input))
        .collect::<Vec<_>>()
        .partition_point(|solve_after| solve_after() != i16::MAX);

    let coord = input[first_blocked_index - 1];

    format!("{},{}", coord.0, coord.1)
}
