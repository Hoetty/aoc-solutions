use std::{collections::HashSet, fs::{self}};

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day15.txt");

    Solution::evaluated(
        "Day 15".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Tile {
    Robot,
    Obstacle,
    Box,
    Air,
}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Map {
    tiles: Vec<Tile>,
    moves: Vec<isize>,
    robot_position: usize,
    width: usize
}

fn get_input(file: &'static str) -> Map {
    let file = fs::read_to_string(file).expect("No file there");
    let (map, moves) = file.split_once("\n\n").unwrap();

    let mut tiles: Vec<Tile> = Vec::with_capacity(map.len());
    let mut robot_position = None;
    let mut width = 0;

    for (y, line) in map.lines().enumerate() {
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            tiles.push(match c {
                '.' => Tile::Air,
                'O' => Tile::Box,
                '#' => Tile::Obstacle,
                '@' => {
                    robot_position = Some((y * line.len() + x) as usize);
                    Tile::Robot
                },
                _ => panic!("Unknown char {:?}", c)
            });
        }
    }

    let mut mov: Vec<isize> = Vec::with_capacity(moves.len());

    for c in moves.replace("\n", "").chars() {
        mov.push(match c {
            '^' => -(width as isize),
            '<' => -1,
            '>' => 1,
            'v' => width as isize,
            _ => panic!("Unknown move char {:?}", c)
        });
    }

    Map {
        tiles,
        moves: mov,
        robot_position: robot_position.unwrap(),
        width
    }
}

fn solve_first(input: Map) -> usize {
    let mut input = input;
    let mut robot_position = input.robot_position;
    for move_index in 0..input.moves.len() {
        let direction = input.moves[move_index];

        let mut i: isize = 1;
        loop {
            let index = (robot_position as isize + i * direction) as usize;
            match input.tiles[index] {
                Tile::Robot => panic!("Hit myself"),
                Tile::Obstacle => break,
                Tile::Box => i += 1,
                Tile::Air => {
                    let next_index = (robot_position as isize + direction) as usize;
                    input.tiles.swap(index, next_index);
                    input.tiles.swap(next_index, robot_position);
                    robot_position = next_index;
                    break;
                },
            }
        }
    }

    let mut sum = 0;
    for (i, tile) in input.tiles.iter().enumerate() {
        if *tile == Tile::Box {
            sum += i / input.width * 100 + i % input.width;
        }
    }

    sum
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum WideTile {
    Robot,
    Obstacle,
    LeftBox,
    RightBox,
    Air,
}

#[inline(always)]
fn is_horizontal_movement(direction: isize) -> bool {
    direction == 1 || direction == -1
}

fn can_move(position: usize, direction: isize, tiles: &Vec<WideTile>) -> bool {
    let next_index = (position as isize + direction) as usize;
    match tiles[next_index] {
        WideTile::Robot => panic!("Moved self"),
        WideTile::Obstacle => false,
        WideTile::LeftBox => if is_horizontal_movement(direction) {
            can_move(next_index, direction, tiles)
        } else {
            can_move(next_index, direction, tiles) && can_move(next_index + 1, direction, tiles)
        },
        WideTile::RightBox => if is_horizontal_movement(direction) {
            can_move(next_index, direction, tiles)
        } else {
            can_move(next_index, direction, tiles) && can_move(next_index - 1, direction, tiles)
        },
        WideTile::Air => true,
    }
}

fn do_move(position: usize, direction: isize, tiles: &mut Vec<WideTile>, moved: &mut HashSet<usize>) {
    if !moved.insert(position) {
        return;
    }

    let next_index = (position as isize + direction) as usize;
    match tiles[position] {
        WideTile::Robot => do_move(next_index, direction, tiles, moved),
        WideTile::Obstacle => panic!("Moved wall"),
        WideTile::LeftBox => if is_horizontal_movement(direction) {
            do_move(next_index, direction, tiles, moved);
        } else {
            do_move(next_index, direction, tiles, moved); 
            do_move(position + 1, direction, tiles, moved);
        },
        WideTile::RightBox => if is_horizontal_movement(direction) {
            do_move(next_index, direction, tiles, moved);
        } else {
            do_move(next_index, direction, tiles, moved);
            do_move(position - 1, direction, tiles, moved);
        },
        WideTile::Air => return,
    }

    tiles.swap(position, next_index);
}

#[allow(unused)]
fn dump_map(tiles: &Vec<WideTile>, width: usize) {
    for (i, tile) in tiles.iter().enumerate() {
        if i % width == 0 {
            print!("\n");
        }

        print!("{}", match tile {
            WideTile::Robot => '@',
            WideTile::Obstacle => '#',
            WideTile::LeftBox => '[',
            WideTile::RightBox => ']',
            WideTile::Air => '.',
        });
    }
}

fn solve_second(input: Map) -> usize {
    let width = input.width * 2;
    let mut tiles: Vec<WideTile> = Vec::with_capacity(input.tiles.len() * 2);

    for tile in input.tiles {
        let (left, right) = match tile {
            Tile::Robot => (WideTile::Robot, WideTile::Air),
            Tile::Obstacle => (WideTile::Obstacle, WideTile::Obstacle),
            Tile::Box => (WideTile::LeftBox, WideTile::RightBox),
            Tile::Air => (WideTile::Air, WideTile::Air),
        };
        tiles.push(left);
        tiles.push(right);
    }

    let moves: Vec<isize> = input.moves.iter().map(|delta| if is_horizontal_movement(*delta) { *delta } else { delta * 2}).collect();

    let mut robot_position = input.robot_position * 2;

    for direction in moves {
        if can_move(robot_position, direction, &tiles) {
            do_move(robot_position, direction, &mut tiles, &mut HashSet::new());
            robot_position = (robot_position as isize + direction) as usize;
        }
    }

    let mut sum = 0;
    for (i, tile) in tiles.iter().enumerate() {
        if *tile == WideTile::LeftBox {
            sum += i / width * 100 + i % width;
        }
    }

    sum
}
