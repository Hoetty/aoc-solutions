use std::fs;

pub fn solutions() {
    let input = get_input("inputs/2015/day6.txt");
    println!("2015 Day 6 #1: {}", solve_first(input.clone()));
    println!("2015 Day 6 #2: {}", solve_second(input));
}

const TURN_ON: &str = "turn on";
const TURN_OFF: &str = "turn off";
const TOGGLE: &str = "toggle";

const TURN_ON_LENGTH: usize = TURN_ON.len() + 1;
const TURN_OFF_LENGTH: usize = TURN_OFF.len() + 1;
const TOGGLE_LENGTH: usize = TOGGLE.len() + 1;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Command {
    Set(u8, (u16, u16), (u16, u16)),
    Toggle((u16, u16), (u16, u16))
}

fn parse_area(input: &str) -> ((u16, u16), (u16, u16)) {
    let (left, right) = input.split_once(" through ").unwrap();
    let (lleft, lright) = left.split_once(",").unwrap();
    let (rleft, rright) = right.split_once(",").unwrap();

    ((lleft.parse().unwrap(), lright.parse().unwrap()), (rleft.parse().unwrap(), rright.parse().unwrap()))
}

fn parse_command(input: &str) -> Command {
    match true {
        _ if input.starts_with(TOGGLE) => {
            let (first, second) = parse_area(&input[TOGGLE_LENGTH..]);
            Command::Toggle(first, second)
        },
        _ if input.starts_with(TURN_ON) => {
            let (first, second) = parse_area(&input[TURN_ON_LENGTH..]);
            Command::Set(1, first, second)
        },
        _ => {
            let (first, second) = parse_area(&input[TURN_OFF_LENGTH..]);
            Command::Set(0, first, second)
        }
    }
}

fn get_input(file: &'static str) -> Vec<Command> {
    fs::read_to_string(file).unwrap().lines().map(&parse_command).collect()
}

#[inline]
fn in_area(point: (u16, u16), first: (u16, u16), second: (u16, u16)) -> bool {
    point.0 >= first.0 && point.0 <= second.0 && point.1 >= first.1 && point.1 <= second.1
}

fn solve_first(input: Vec<Command>) -> usize {
    let mut lit: usize = 0;
    let command_length = input.len();

    for x in 0..1000 {
        'middle: for y in 0..1000 {
            let mut invert = 0;

            for i in 0..command_length {
                let command = input[command_length - i - 1];
                match command {
                    Command::Set(value, first, second) if in_area((x, y), first, second) => {
                        lit += (value ^ (invert & 1)) as usize;
                        continue 'middle;
                    },
                    Command::Toggle(first, second) if in_area((x, y), first, second) => invert += 1,
                    _ => { }
                }
            }

            lit += (invert & 1) as usize;
        }
    }

    lit
}


fn solve_second(input: Vec<Command>) -> usize {
    let mut lit: usize = 0;
    let command_length = input.len();

    for x in 0..1000 {
        for y in 0..1000 {
            let mut brightness = 0;

            for i in 0..command_length {
                let command = input[i];
                match command {
                    Command::Set(value, first, second) if in_area((x, y), first, second) => {
                        if value == 1 {
                            brightness += 1;
                        } else if brightness > 0 {
                            brightness -= 1;
                        }
                    },
                    Command::Toggle(first, second) if in_area((x, y), first, second) => brightness += 2,
                    _ => { }
                }
            }

            lit += brightness as usize;
        }
    }

    lit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> (usize, usize) {
        let file = fs::read_to_string("test-inputs/2015/day6-expect.txt").expect("Expect file missing");
        let nums: Vec<&str> = file.split_whitespace().collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }

    #[test]
    fn part1() {
        let result = solve_first(get_input("test-inputs/2015/day6.txt"));
        assert_eq!(result, expected().0);
    }

    #[test]
    fn part2() {
        let result = solve_second(get_input("test-inputs/2015/day6.txt"));
        assert_eq!(result, expected().1);
    }
}