use std::{collections::VecDeque, fs, usize};

pub fn solutions() {
    let input = get_input();
    println!("Day 9, #1: {}", solve_first(input.clone()));
    println!("Day 9, #2: {}", solve_second(input.clone()));
}

pub fn get_input() -> String {
    fs::read_to_string("inputs/day9.txt").expect("No file there")
}

pub fn solve_first(input: String) -> u64 {
    let mut filesystem: Vec<Option<u16>> = Vec::new();

    let mut id = 0;

    for (i, char) in input.chars().enumerate() {
        let count = (char as u8) - 48; // char -> i16

        for _ in 0..count {
            if i % 2 == 0 {
                filesystem.push(Some(id));
            } else {
                filesystem.push(None);
            }
        }

        if i % 2 == 0 {
            id += 1;
        }
    }

    let mut head = 0;
    let mut tail = filesystem.len() - 1;

    while tail > head {
        if filesystem[head].is_some() {
            head += 1;
            continue;
        }

        if filesystem[tail].is_none() {
            tail -= 1;
            continue;
        }

        filesystem[head] = filesystem[tail];
        filesystem[tail] = None;
    }

    let mut sum: u64 = 0;
    for i in 0..tail {
        sum += i as u64 * filesystem[i].expect("Must be some") as u64;
    }

    sum
}

pub fn solve_second(input: String) -> u64 {
    let mut free_space: Vec<VecDeque<usize>> = Vec::new();
    free_space.resize(10, VecDeque::new());
    let mut files: Vec<(u16, usize, u8)> = Vec::new();
    
    let mut id = 0;
    let mut index = 0;
    for (i, char) in input.chars().enumerate() {
        let count = (char as u8) - 48; // char -> i16

        if i % 2 == 0 {
            files.push((id, index, count));
            id += 1;
        } else if count == 0 {
            continue;
        } else {
            free_space[count as usize].push_back(index);
        }

        index += count as usize;
    }

    let mut new_files = Vec::with_capacity(files.len());

    for file in files.iter().rev() {
        let mut empty_block_size = 11;
        let mut earliest_empty_space = None;

        // Find the earliest block of empty space, that is at least the size of the file
        for space in file.2..10 {
            let possible_place = free_space[space as usize].front();

            match possible_place {
                Some(possible_index) if earliest_empty_space.is_none() || possible_index < earliest_empty_space.unwrap() => {
                    earliest_empty_space = possible_place;
                    empty_block_size = space;
                },
                _ => { },
            }            
        }

        match earliest_empty_space {
            Some(new_file_index) if *new_file_index < file.1 => {
                // Move the file to the empty space
                new_files.push((file.0, *new_file_index, file.2));

                let remaining_space = empty_block_size - file.2;
                let new_empty_index = new_file_index + file.2 as usize;

                // Remove the old free space
                free_space[empty_block_size as usize].pop_front();

                // Add new free space
                if remaining_space > 0 {
                    let pos = free_space[remaining_space as usize].binary_search(&new_empty_index);
                    free_space[remaining_space as usize].insert(pos.unwrap_or_else(|x| x), new_empty_index);
                }
            },
            _ => {
                new_files.push(*file);
            },
        }
    }

    let mut sum = 0;

    for file in new_files {
        for i in 0..file.2 {
            sum += file.0 as usize * (file.1 + i as usize);
        }
    }

    sum as u64
}