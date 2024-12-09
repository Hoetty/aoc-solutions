use std::fs;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum File {
    Present(u16, u8),
    Empty(u8)
}

pub fn solve_second(input: String) -> u64 {
    let mut filesystem: Vec<File> = Vec::new();

    let mut id = 0;

    for (i, char) in input.chars().enumerate() {
        let count = (char as u8) - 48; // char -> i16

        if i % 2 == 0 {
            filesystem.push(File::Present(id, count));
        } else {
            filesystem.push(File::Empty(count));
        }
        
        if i % 2 == 0 {
            id += 1;
        }
    }

    let mut minimum: Vec<usize> = Vec::new();
    minimum.resize(11, 0);  

    for file_head in (0..filesystem.len()).rev() {
        let (id, file_size) = match filesystem[file_head] {
            File::Empty(_) => continue,
            File::Present(id, size) => (id, size),
        };

        for empty_head in minimum[file_size as usize]..file_head {
            match filesystem[empty_head] {
                File::Present(_, _) => continue,
                File::Empty(space) if space >= file_size => {
                    filesystem[file_head] = File::Empty(file_size);
                    filesystem[empty_head] = File::Present(id, file_size);
                    filesystem.insert(empty_head + 1, File::Empty(space - file_size));
                    minimum[file_size as usize] = empty_head + 1;
                    break;
                },
                _ => continue
            }
        }
    }

    let mut sum = 0;
    let mut i: u64 = 0;

    for file in filesystem {
        match file {
            File::Present(id, size) => {
                for _ in 0..size {
                    sum += id as u64 * i as u64;
                    i += 1;
                }
            },
            File::Empty(size) => {
                for _ in 0..size {
                    i += 1;
                }
            },
        }
    }

    sum as u64
}