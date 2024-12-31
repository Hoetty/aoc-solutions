use std::fs;

use crate::{solutions, util::flatgrid::FlatGrid};

solutions!{2024, 15}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Tile {
    Robot,
    Obstacle,
    Box,
    Air,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum WideTile {
    Robot,
    Obstacle,
    LeftBox,
    RightBox,
    Air,
}

const WIDTH: usize = 50;
const WIDE_WIDTH: usize = WIDTH * 2;
const HEIGHT: usize = 50;

/// The standard warehouse
type Warehouse = FlatGrid<Tile, WIDTH, HEIGHT>;
/// A warehouse twice as wide
type WideWarehouse = FlatGrid<WideTile, WIDE_WIDTH, HEIGHT>;
/// A grid to store, when the block was last moved to avoid duplicate moves
type LastMoved = FlatGrid<u16, WIDE_WIDTH, HEIGHT>;
/// A grid to calculate GPS coordinates
type Gps = FlatGrid<bool, 100, HEIGHT>;
type Moves = Vec<isize>;

fn get_input(file: &str) -> (Warehouse, Moves, usize) {
    let file = fs::read_to_string(file).expect("No file there");
    let (map, move_list) = file.split_once("\n\n").unwrap();

    let mut warehouse: Warehouse = Warehouse::new();
    let mut robot_position = 0;

    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            warehouse.push(match c {
                '.' => Tile::Air,
                'O' => Tile::Box,
                '#' => Tile::Obstacle,
                '@' => {
                    robot_position = Warehouse::to_index(x, y);
                    Tile::Robot
                },
                _ => panic!("Unknown char {:?}", c)
            });
        }
    }

    let mut moves: Vec<isize> = Vec::with_capacity(move_list.len());

    for c in move_list.chars() {
        moves.push(match c {
            '^' => -(Warehouse::width() as isize),
            '<' => -1,
            '>' => 1,
            'v' => Warehouse::width() as isize,
            '\n' => continue,
            _ => panic!("Unknown move char {:?}", c)
        });
    }

    (warehouse, moves, robot_position)
}

/// ### Robot Chaos
/// 
/// Simulates a robot pushing boxes in a warehouse.
/// The robot can push boxes that have boxes behind them, pushing entire box stacks
/// If a box has an obstacle, nothing moves
/// 
/// After doing every move, the gps is calculated, it is the index in a 100 tile wide warehouse
fn solve_first(input: &(Warehouse, Moves, usize)) -> usize {
    let (warehouse, moves, mut robot_position) = input;
    let mut warehouse = warehouse.clone();

    for direction in moves {
        // For each move, we scan in the travel direction
        let mut i: isize = 1;
        loop {
            let index = (robot_position as isize + i * direction) as usize;
            match warehouse[index] {
                Tile::Robot => panic!("Hit myself"),
                // If we hit an obstacle, the current move is cancelled
                Tile::Obstacle => break,
                // If we hit a box, we scan further
                Tile::Box => i += 1,
                // When we hit air, we swap the air with the position in front of the robot
                // and the robot with the position in front of itself
                // In case the robot moves to an air spot in front of it, the air doesn't move,
                // then the robot swaps with the air
                // In case the robots moves any number of boxes, the air swaps with the first box,
                // and then the robot and the air swap
                // This way, scanning is O(n) but moving is O(1) as we don't shift all boxes
                Tile::Air => {
                    let next_index = (robot_position as isize + direction) as usize;
                    warehouse.swap(index, next_index);
                    warehouse.swap(next_index, robot_position);
                    robot_position = next_index;
                    break;
                },
            }
        }
    }

    warehouse
        .iter()
        .enumerate()
        .filter(|(_, tile)| **tile == Tile::Box)
        .map(|(i, _)| {
            let (x, y) = Warehouse::to_coordinates(i);
            Gps::to_index(x, y)
        }).sum()
}

/// Checks if a move is horizotal
#[inline(always)]
fn is_horizontal_movement(direction: isize) -> bool {
    direction == 1 || direction == -1
}

/// Recursively checks if a move can be performed for a tile
/// This accounts for two wide boxes having multiple tiles to check
fn can_move(position: usize, direction: isize, warehouse: &WideWarehouse) -> bool {
    let next_index = (position as isize + direction) as usize;
    match warehouse[next_index] {
        WideTile::Robot => panic!("Moved self"),
        WideTile::Obstacle => false,
        WideTile::LeftBox => if is_horizontal_movement(direction) {
            can_move(next_index, direction, warehouse)
        } else {
            // When moving vertically, not only check the next one for this tile, but also the connected one
            can_move(next_index, direction, warehouse) && can_move(next_index + 1, direction, warehouse)
        },
        WideTile::RightBox => if is_horizontal_movement(direction) {
            can_move(next_index, direction, warehouse)
        } else {
            // Same story here
            can_move(next_index, direction, warehouse) && can_move(next_index - 1, direction, warehouse)
        },
        // Only if all movement chains end in air is the move possible
        WideTile::Air => true,
    }
}

/// Performs a move, that has already been validated by can_move
/// This is a bit different, as can_move is started from the tile the robot wants to move to,
/// but do_move is started at the robot
fn do_move(position: usize, direction: isize, warehouse: &mut WideWarehouse, current_move: u16, moved: &mut LastMoved) {
    // Check if the tile was already moved this turn
    if moved[position] == current_move {
        return;
    }
    moved[position] = current_move;

    let next_index = (position as isize + direction) as usize;
    match warehouse[position] {
        WideTile::Robot => do_move(next_index, direction, warehouse, current_move, moved),
        WideTile::Obstacle => panic!("Moved wall"),
        WideTile::LeftBox => if is_horizontal_movement(direction) {
            do_move(next_index, direction, warehouse, current_move, moved);
        } else {
            do_move(next_index, direction, warehouse, current_move, moved); 
            do_move(position + 1, direction, warehouse, current_move, moved);
        },
        WideTile::RightBox => if is_horizontal_movement(direction) {
            do_move(next_index, direction, warehouse, current_move, moved);
        } else {
            do_move(next_index, direction, warehouse, current_move, moved);
            do_move(position - 1, direction, warehouse, current_move, moved);
        },
        WideTile::Air => return,
    }

    // After performing all cascading moves, the current tile can now swap
    warehouse.swap(position, next_index);
}

/// ### Wide Warehouse Chaos
/// 
/// The warehouse is twice as wide as in part 1, allowing boxes to become unaligned an push multiple other boxes
///         [][]
/// [][] /\  []
///  []  |   @
///  @
/// 
/// The robot still follows its moves from part 1 and the gps calculation is unchanged
fn solve_second(input: &(Warehouse, Moves, usize)) -> usize {
    let (warehouse, moves, mut robot_position) = input;

    robot_position *= 2;

    // Enlarge the warehouse
    let mut warehouse: WideWarehouse = warehouse.iter().flat_map(|tile| match tile {
        Tile::Robot => [WideTile::Robot, WideTile::Air],
        Tile::Obstacle => [WideTile::Obstacle, WideTile::Obstacle],
        Tile::Box => [WideTile::LeftBox, WideTile::RightBox],
        Tile::Air => [WideTile::Air, WideTile::Air],
    }).collect::<Vec<WideTile>>().into();

    let mut last_moved: LastMoved = LastMoved::default();

    // Vertical moves need double the distance now
    let moves = moves
        .iter()
        .map(|delta| if is_horizontal_movement(*delta) { *delta } else { delta * 2 })
        .enumerate()
        .map(|(i, direction)| (i as u16 + 1, direction));

    for (i, direction) in moves {
        if can_move(robot_position, direction, &warehouse) {
            do_move(robot_position, direction, &mut warehouse, i, &mut last_moved);
            robot_position = (robot_position as isize + direction) as usize;
        }
    }

    warehouse.iter()
        .enumerate()
        .filter(|(_, tile)| **tile == WideTile::LeftBox)
        .map(|(i, _)| i)
        .sum()
}
