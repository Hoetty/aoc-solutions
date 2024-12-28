use std::fs;

use crate::solutions;

solutions!{2015, 4}

pub fn get_input(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

pub fn solve_first(input: &str) -> usize {
    let mut i = 0;

    loop {
        let test = format!("{}{}", input, i);
        let hash = md5::compute(test);

        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0b11110000) == 0 {
            return i;  
        }

        i += 1;
    }
}

pub fn solve_second(input: &str) -> usize {
    let mut i = 0;

    loop {
        let test = format!("{}{}", input, i);
        let hash = md5::compute(test);

        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i;  
        }

        i += 1;
    }
}
