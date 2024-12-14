use std::{fs::{self, OpenOptions}, io::Write};

const DIMENSION: (i16, i16) = (101, 103);
const TREE_THRESHOLD: f64 = 700.0;

pub fn solutions() {
    let input = get_input("inputs/2024/day14.txt", DIMENSION);
    println!("2024 Day 14 #1: {}", solve_first(input.clone()));
    println!("2024 Day 14 #2: {}", solve_second(input.clone()));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Robot((i16, i16), (i16, i16));
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Map {
    robots: Vec<Robot>,
    dimension: (i16, i16),
}

impl Map {
    
    fn step(&mut self) {
        for robot in &mut self.robots {
            robot.0 = ((self.dimension.0 + robot.0.0 + robot.1.0) % self.dimension.0, (self.dimension.1 + robot.0.1 + robot.1.1) % self.dimension.1);
        }
    }

    fn calculate_safety_score(&self) -> u64 {
        let mut tl = 0;
        let mut tr = 0;
        let mut bl = 0;
        let mut br = 0;

        let middle_x = (self.dimension.0 + 1) / 2 - 1;
        let middle_y = (self.dimension.1 + 1) / 2 - 1;

        for robot in self.robots.iter() {
            if robot.0.0 < middle_x && robot.0.1 < middle_y {
                tl += 1;
            } else if robot.0.0 > middle_x && robot.0.1 < middle_y {
                tr += 1;
            } else if robot.0.0 < middle_x && robot.0.1 > middle_y {
                bl += 1;
            } else if robot.0.0 > middle_x && robot.0.1 > middle_y {
                br += 1;
            }
        }

        tl * tr * bl * br
    }

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

    fn v(&self) -> (f64, f64) {
        let mut vx = 0.0;
        let mut vy = 0.0;
        let mu = self.mu();

        let count = self.robots.len() as f64;

        for robot in &self.robots {
            let diff_x = robot.0.0 as f64 - mu.0;
            let diff_y = robot.0.1 as f64 - mu.1;
            vx += diff_x * diff_x;
            vy += diff_y * diff_y;
        }

        (vx / count, vy / count)
    }

    #[allow(unused)]
    fn dump(&self) {
        let mut grid: Vec<Vec<u8>> = Vec::new();

        grid.resize(self.dimension.0 as usize, Vec::new());

        for x in 0..self.dimension.0 {
            grid[x as usize].resize(self.dimension.1 as usize, 0);
        }

        for robot in &self.robots {
            grid[robot.0.0 as usize][robot.0.1 as usize] += 1;
        }

        fs::remove_file("dump.txt").unwrap();

        let mut file = OpenOptions::new().create(true).write(true).append(true).open("dump.txt").unwrap();

        for y in 0..self.dimension.1 {
            for x in 0..self.dimension.0 {
                write!(file, "{}", grid[x as usize][y as usize]).unwrap();
            }
            write!(file, "\n").unwrap();
        }
    }
}

fn get_input(file: &'static str, dimension: (i16, i16)) -> Map {
    Map {
        robots: fs::read_to_string(file).expect("No file there").lines().map(|l| {
            let l = l.replace("p=", "").replace("v=", "");
            let (position, velocity) = l.split_once(" ").unwrap();

            let (px, py) = position.split_once(",").unwrap();
            let (vx, vy) = velocity.split_once(",").unwrap();

            Robot((px.parse().unwrap(), py.parse().unwrap()), (vx.parse().unwrap(), vy.parse().unwrap()))
        }).collect(),
        dimension
    }
}

fn solve_first(input: Map) -> u64 {
    let mut input = input;
    for _ in 0..100 {
        input.step();
    }

    input.calculate_safety_score()
}

// The drones have repeating cycles of low variance with a frequency equal to the length of the dimension:
// In x direction -> width
// In y direction -> height
// We find both first spots and then calculate when the two frequencies will meet. Thats the christmas tree
fn solve_second(input: Map) -> u64 {
    let mut input = input;
    let mut i: u64 = 0;

    let mut x = 0;
    let mut y = 0;

    loop {
        let v = input.v();
        if x == 0 && v.0 < TREE_THRESHOLD {
            x = i;
        }

        if y == 0 && v.1 < TREE_THRESHOLD {
            y = i;
        }

        if x > 0 && y > 0 {
            let (width, height) = input.dimension;

            if width < height {
                if x < y {
                    x += width as u64;
                }

                let mut diff = x - y;

                if diff & 1 == 1 {
                    diff += width as u64;
                }

                return y + diff / 2 * height as u64;
            } else {
                if x > y {
                    y += height as u64;
                }

                let mut diff = y - x;

                if diff & 1 == 1 {
                    diff += height as u64;
                }

                return x + diff / 2 * width as u64;
            }
        }

        i += 1;

        input.step();
    }
}

#[cfg(test)]
mod tests {
    const TEST_DIMENSION: (i16, i16) = (101, 103);

    use super::*;

    fn expected() -> (u64, u64) {
        let file = fs::read_to_string("test-inputs/2024/day14-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2024/day14.txt", TEST_DIMENSION));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2024/day14.txt", TEST_DIMENSION));
        assert_eq!(result, expected().1);
    }
}