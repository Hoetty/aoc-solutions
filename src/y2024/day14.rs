use std::fs;

use crate::solutions;

solutions!{2024, 14}

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

const MIDDLE_X: usize = (WIDTH + 1) / 2 - 1;
const MIDDLE_Y: usize = (HEIGHT + 1) / 2 - 1;

const TREE_THRESHOLD: f64 = 700.0;

/// A robot has a position and a velocity vector
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Robot((usize, usize), (isize, isize));

// The map stores all the robots
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Map {
    robots: Vec<Robot>,
}

impl Map {
    /// Update the robots positions using their velocity
    /// If they move out of bounds they wrap around
    fn step<const N: isize>(&mut self) {
        for robot in &mut self.robots {
            robot.0 = (
                (robot.0.0 as isize + robot.1.0 * N).rem_euclid(WIDTH as isize) as usize, 
                (robot.0.1 as isize + robot.1.1 * N).rem_euclid(HEIGHT as isize) as usize
            );
        }
    }

    /// Calculates the safety score
    /// The score is the product of the number of robots in each quadrant
    /// The robots in the lines in the middle on any axis are ignored
    fn calculate_safety_score(&self) -> u64 {
        let mut tl = 0;
        let mut tr = 0;
        let mut bl = 0;
        let mut br = 0;

        for robot in self.robots.iter() {
            if robot.0.0 < MIDDLE_X && robot.0.1 < MIDDLE_Y {
                tl += 1;
            } else if robot.0.0 > MIDDLE_X && robot.0.1 < MIDDLE_Y {
                tr += 1;
            } else if robot.0.0 < MIDDLE_X && robot.0.1 > MIDDLE_Y {
                bl += 1;
            } else if robot.0.0 > MIDDLE_X && robot.0.1 > MIDDLE_Y {
                br += 1;
            }
        }

        tl * tr * bl * br
    }

    /// Calculates the mean value of all the robots positions
    fn mu(&self) -> (f64, f64) {
        let mut x: u64 = 0;
        let mut y: u64 = 0;
        let count: f64 = self.robots.len() as f64;

        for i in 0..self.robots.len() {
            x += self.robots[i].0.0 as u64;
            y += self.robots[i].0.1 as u64;
        }

        (x as f64 / count, y as f64 / count)
    }

    /// Calculates the variance of the robots positions in both x and y direction
    fn variance(&self) -> (f64, f64) {
        let mut variance_x = 0.0;
        let mut variance_y = 0.0;
        let mu = self.mu();

        let count = self.robots.len() as f64;

        for robot in &self.robots {
            let diff_x = robot.0.0 as f64 - mu.0;
            let diff_y = robot.0.1 as f64 - mu.1;
            variance_x += diff_x * diff_x;
            variance_y += diff_y * diff_y;
        }

        (variance_x / count, variance_y / count)
    }
}

fn get_input(file: &str) -> Map {
    Map {
        robots: fs::read_to_string(file).expect("No file there").lines().map(|l| {
            let (position, velocity) = l[2..].split_once(" v=").unwrap();

            let (px, py) = position.split_once(",").unwrap();
            let (vx, vy) = velocity.split_once(",").unwrap();

            Robot((px.parse().unwrap(), py.parse().unwrap()), (vx.parse().unwrap(), vy.parse().unwrap()))
        }).collect()
    }
}

/// ### Safety Score
/// 
/// Calculates the safety score after 100 steps
fn solve_first(input: &Map) -> u64 {
    let mut input = input.clone();
    input.step::<100>();

    input.calculate_safety_score()
}

/// ### Drone Christmas Tree
/// 
/// The drones have repeating cycles of low variance with a frequency equal to the length of the dimension:
/// In x direction -> width
/// In y direction -> height
/// We find both first spots and then calculate when the two frequencies will meet,
/// this gives us the location of the christmas tree
fn solve_second(input: &Map) -> u64 {
    let mut input = input.clone();
    let mut i: u64 = 0;

    let mut x = 0;
    let mut y = 0;

    loop {
        let (variance_x, variance_y) = input.variance();
        if x == 0 && variance_x < TREE_THRESHOLD {
            x = i;
        }

        if y == 0 && variance_y < TREE_THRESHOLD {
            y = i;
        }

        if x > 0 && y > 0 {
            if x < y {
                x += WIDTH as u64;
            }

            let mut diff = x - y;

            if diff & 1 == 1 {
                diff += WIDTH as u64;
            }

            return y + diff / 2 * HEIGHT as u64;
        }

        i += 1;

        input.step::<1>();
    }
}
