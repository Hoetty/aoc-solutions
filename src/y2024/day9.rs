use std::{collections::VecDeque, fs};

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day9.txt");

    Solution::evaluated(
        "Day 9".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

fn get_input(file: &'static str) -> String {
    fs::read_to_string(file).expect("No file there")
}

fn solve_first(input: String) -> u64 {
    let mut files: Vec<(u16, usize)> = Vec::with_capacity(64);
    let mut empty_space: Vec<usize> = Vec::with_capacity(64);

    let mut id = 0;
    let mut index: usize = 0;

    for (i, char) in input.chars().enumerate() {
        let count = (char as u8) - 48; // char -> i16

        for _ in 0..count {
            if i & 1 == 0 {
                files.push((id, index));
            } else {
                empty_space.push(index);
            }
            index += 1;
        }

        if i & 1 == 0 {
            id += 1;
        }
    }

    let mut new_filesystem: Vec<(u16, usize)> = Vec::with_capacity(index);
    let mut head = 0;

    'outer: for tail in (0..files.len()).rev() {
        match empty_space.get(head) {
            Some(new_index) if *new_index < files[tail].1 => {
                new_filesystem.push((files[tail].0, *new_index));
                head += 1;
            },
            _ => {
                let mut tail = tail;
                loop {
                    new_filesystem.push(files[tail]);
                    if tail == 0 {
                        break;
                    }

                    tail -= 1;
                }
                break 'outer;
            },
        }
    }

    new_filesystem.iter().map(|(id, i)| *id as usize * *i).sum::<usize>() as u64
}

fn solve_second(input: String) -> u64 {
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
