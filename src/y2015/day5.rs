use std::fs;

use rustc_hash::{FxBuildHasher, FxHashSet};

use crate::solutions;

solutions!{2015, 5}

pub fn get_input(file: &str) -> Vec<String> {
    fs::read_to_string(file).unwrap().lines().map(|s| s.to_string()).collect()
}

#[inline(always)]
fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn is_nice(input: &str) -> bool {
    let mut vowels = 0;
    let mut double = false;

    let charlist = input.chars().collect::<Vec<char>>();

    for i in 0..charlist.len() {
        let c = charlist[i];

        if is_vowel(c) {
            vowels += 1;
        }

        if i == 0 {
            continue;
        }

        let last = charlist[i - 1];

        if (last == 'a' && c == 'b') ||
            (last == 'c' && c == 'd') ||
            (last == 'p' && c == 'q') ||
            (last == 'x' && c == 'y') 
        {
            return false
        }


        if last == c {
            double = true;
        }
    }

    vowels >= 3 && double
}

pub fn solve_first(input: &[String]) -> usize {
    input.iter().filter(|s| is_nice(s)).count()
}

fn is_nice2(input: &str) -> bool {
    let charlist = input.chars().collect::<Vec<char>>();
    let mut pairs: FxHashSet<(char, char)> = FxHashSet::with_capacity_and_hasher(32, FxBuildHasher);
    let mut double = false;
    let mut mirrored = false;

    let len = charlist.len();

    for i in 1..len {
        let c = charlist[i];

        let last = charlist[i - 1];

        if !double && (last != c || i == 1 || last != charlist[i - 2] || (i > 2 && charlist[i - 3] == last)) && !pairs.insert((last, c)) {
            double = true;
        }

        if i < (len - 1) && last == charlist[i + 1] {
            mirrored = true;
        }

        if double && mirrored {
            return true;
        }
    }

    false
}

pub fn solve_second(input: &[String]) -> usize {
    input.iter().filter(|s| is_nice2(s)).count()
}