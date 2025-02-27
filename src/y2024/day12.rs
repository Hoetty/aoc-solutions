use std::fs;

use crate::{solutions, util::flatgrid::FlatGrid};

solutions!{2024, 12}

const WIDTH: usize = 140;
const HEIGHT: usize = 140;

type Garden = FlatGrid<u8, WIDTH, HEIGHT>;

fn get_input(file: &str) -> FlatGrid<u8, WIDTH, HEIGHT> {
    fs::read_to_string(file)
        .expect("No file there")
        .lines()
        .flat_map(|l| l.chars().map(|c| c as u8).collect::<Vec<u8>>())
        .collect()
}

const VISITED: u8 = 0x80;
const VALUE_MASK: u8 = 0x7F;

/// ### Garden prices
/// 
/// The garden prices are calculated per crop. A crop is marked as a single number on the grid
/// For each position in the grid we scan for all neighbor cells if they have the same crop,
///    if so, they are added to the queue, else the perimiter is increased by one. The area is also increased every time
fn solve_first(input: &Garden) -> u64 {
    let mut total_price = 0;
    let mut grid = input.clone();
    let mut queue = Vec::with_capacity(64);

    for start_index in Garden::indices() {
        // If this plot has been visited by another scan, we skip it entirely
        if grid[start_index] & VISITED != 0 {
            continue;
        }

        let mut plot_area = 0;
        let mut plot_perimiter = 0;

        queue.push(start_index);

        while let Some(index) = queue.pop() {
            let current_crop = grid[index];
            
            // If the visited bit is set, we don't process this plot a second time
            if current_crop & VISITED != 0 {
                continue;
            }
            
            plot_area += 1;

            if !Garden::will_horizontal_move_cross_border(index, -1) && 
                grid[Garden::moved_horizontally(index, -1)] & VALUE_MASK == current_crop 
            {
                queue.push(Garden::moved_horizontally(index, -1));
            } else {
                plot_perimiter += 1;
            }

            if !Garden::will_horizontal_move_cross_border(index, 1) && 
                grid[Garden::moved_horizontally(index, 1)] & VALUE_MASK == current_crop 
            {
                queue.push(Garden::moved_horizontally(index, 1));
            } else {
                plot_perimiter += 1;
            }

            if !Garden::will_vertical_move_cross_border(index, -1) && 
                grid[Garden::moved_vertically(index, -1)] & VALUE_MASK == current_crop 
            {
                queue.push(Garden::moved_vertically(index, -1));
            } else {
                plot_perimiter += 1;
            }

            if !Garden::will_vertical_move_cross_border(index, 1) && 
                grid[Garden::moved_vertically(index, 1)] & VALUE_MASK == current_crop 
            {
                queue.push(Garden::moved_vertically(index, 1));
            } else {
                plot_perimiter += 1;
            }

            // After processing we set the visited bit, as to not process it again
            grid[index] |= VISITED;
        }

        total_price += plot_area * plot_perimiter;
    }
    
    total_price
}

/// ### Discounted Garden Price
/// 
/// This performs the same algorithm, but only counts the left and top most fence edges
/// This is done by checking if the above or left plot is of a different type, outside the map,
///    or if the plot where the edge is leading into is of the same type
fn solve_second(input: &FlatGrid<u8, WIDTH, HEIGHT>) -> u64 {
    let mut price = 0;
    let mut grid = input.clone();
    let mut queue = Vec::with_capacity(64);

    for start_index in Garden::indices() {
        if grid[start_index] & VISITED != 0 {
            continue;
        }

        queue.push(start_index);

        let mut plot_area = 0;
        let mut plot_perimiter = 0;

        while let Some(index) = queue.pop() {
            let current_crop = grid[index];
            
            if current_crop & VISITED != 0 {
                continue;
            }

            plot_area += 1;
            
            let (x, y) = Garden::to_coordinates(index);

            if !Garden::will_horizontal_move_cross_border(index, -1) && 
                grid[Garden::moved_horizontally(index, -1)] & VALUE_MASK == current_crop 
            {
                queue.push(Garden::moved_horizontally(index, -1));
            } else if y == 0 || (grid[Garden::moved_vertically(index, -1)] & VALUE_MASK != current_crop || (x > 0 && grid[Garden::moved(index, -1, -1)] & VALUE_MASK == current_crop)) {
                plot_perimiter += 1;
            }

            if !Garden::will_horizontal_move_cross_border(index, 1) && 
                grid[Garden::moved_horizontally(index, 1)] & VALUE_MASK == current_crop 
            {
                queue.push(Garden::moved_horizontally(index, 1));
            } else if y == 0 || (grid[Garden::moved_vertically(index, -1)] & VALUE_MASK != current_crop || (x < Garden::width() - 1 && grid[Garden::moved(index, 1, -1)] & VALUE_MASK == current_crop)) {
                plot_perimiter += 1;
            }

            if !Garden::will_vertical_move_cross_border(index, -1) && 
                grid[Garden::moved_vertically(index, -1)] & VALUE_MASK == current_crop 
            {
                queue.push(Garden::moved_vertically(index, -1));
            } else if x == 0 || (grid[Garden::moved_horizontally(index, -1)] & VALUE_MASK != current_crop || (y > 0 && grid[Garden::moved(index, -1, -1)] & VALUE_MASK == current_crop)) {
                plot_perimiter += 1;
            }

            if !Garden::will_vertical_move_cross_border(index, 1) && 
                grid[Garden::moved_vertically(index, 1)] & VALUE_MASK == current_crop 
            {
                queue.push(Garden::moved_vertically(index, 1));
            } else if x == 0 || (grid[Garden::moved_horizontally(index, -1)] & VALUE_MASK != current_crop || (y < Garden::height() - 1 && grid[Garden::moved(index, -1, 1)] & VALUE_MASK == current_crop)) {
                plot_perimiter += 1;
            }

            grid[index] |= VISITED;
        }

        price += plot_area * plot_perimiter;
    }
    
    price
}
