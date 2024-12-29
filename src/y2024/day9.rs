use std::{collections::VecDeque, fs};

use crate::solutions;

solutions!{2024, 9}

fn get_input(file: &str) -> String {
    fs::read_to_string(file).expect("No file there")
}

/// ### Disk Compression
/// 
/// Move parts of files into the first empty space available,
///    creating a big blob of files followed by a big blob of empty space
fn solve_first(input: &str) -> u64 {
    let mut file_system: Vec<Option<u16>> = Vec::with_capacity(1024);

    let mut current_file_id = 0;

    // For each character we calculate its value and keep a cumulative sum
    // The sum represents the size of the filesystem at that point
    let files = input
        .chars()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .scan(0, |total_length, length| {
            *total_length += length;
            Some(*total_length)
        })
        .enumerate();

    // For each entry we resize the filesystem to the new length and fill with
    // A: If the iteration index is even -> The current file id
    // B: Else -> None
    for (i, length) in files {
        if i & 1 == 0 {
            file_system.resize(length, Some(current_file_id));
            current_file_id += 1;
        } else {
            file_system.resize(length, None);
        }
    }

    // The filesystem is traversed by two pointers
    // Head starts at the front of the filesystem, tail at the end
    // They move towards each other, when they meet, all files have "moved"
    // Note: The files arent actually moved, rather the head pointer is used as the current index,
    //    and the sum is calculated while traversing with the file id being either the non empty 
    //    file at head or the non empty file at tail. When a file at tail "filled" an empty spot
    //    both pointers move towards each other
    let mut head = 0;
    let mut tail = file_system.len() - 1;
    let mut sum = 0;

    loop {
        // Count all files that dont need to be moved
        while let Some(id) = file_system[head] {
            sum += id as u64 * head as u64;
            head += 1;
        }

        // Find the last file
        while file_system[tail].is_none() {
            tail -= 1;
        }

        if head > tail {
            break;
        }

        // Calculate the files score as if it had moved to head
        sum += file_system[tail].unwrap() as u64 * head as u64;

        head += 1;
        tail -= 1;
    }

    sum
}

/// ### Disk Defragmentation
/// 
/// Move parts of files into the first empty space available,
///    however, now files cant be cut in half. If a file has nowhere to go
///    it does not move, even if space is later available
/// 
/// A sorted list is kept with all positions of empty space that can be indexed by filesize
/// When a file wants to move, only the empty spaces at least the size of the file need to be checked,
///     with the earliest in the system being chosen
fn solve_second(input: &str) -> u64 {
    let mut free_space: [VecDeque<usize>; 10] = Default::default();
    let mut files: Vec<(u16, usize, u8)> = Vec::new();
    
    let mut current_file_id = 0;
    let mut index = 0;
    for (i, length) in input.chars().map(|character| character.to_digit(10).unwrap() as u8).enumerate() {
        if i & 1 == 0 {
            files.push((current_file_id, index, length));
            current_file_id += 1;
        } else if length == 0 {
            continue;
        } else {
            free_space[length as usize].push_back(index);
        }

        index += length as usize;
    }

    let mut new_files = Vec::with_capacity(files.len());

    for file in files.iter().rev() {
        let mut earliest_empty_space: Option<(usize, u8)> = None;

        // Find the earliest block of empty space, that is at least the size of the file
        for file_size in file.2..10 {

            let Some(empty_space_index) = free_space[file_size as usize].front() else {
                continue;
            };

            if earliest_empty_space.is_none() || *empty_space_index < earliest_empty_space.unwrap().0 {
                earliest_empty_space = Some((*empty_space_index, file_size));
            }
        }

        match earliest_empty_space {
            Some((new_file_index, empty_block_size)) if new_file_index < file.1 => {
                // Move the file to the empty space's index
                new_files.push((file.0, new_file_index, file.2));

                let remaining_space = empty_block_size - file.2;
                let new_empty_index = new_file_index + file.2 as usize;

                // Remove the old free space
                free_space[empty_block_size as usize].pop_front();

                // Add new free space and maintain sorted order
                if remaining_space > 0 {
                    let pos = free_space[remaining_space as usize].binary_search(&new_empty_index);
                    free_space[remaining_space as usize].insert(pos.unwrap_or_else(|index| index), new_empty_index);
                }
            },
            _ => {
                // If no empty space was found or the earliest one is after the file,
                //    the file is added as is to the new filesystem
                new_files.push(*file);
            },
        }
    }

    let mut sum = 0u64;

    for file in new_files {
        for i in 0..file.2 {
            sum += file.0 as u64 * (file.1 as u64 + i as u64);
        }
    }

    sum
}
