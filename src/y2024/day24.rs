use std::{cmp::Ordering, collections::HashMap, fs::{self, OpenOptions}, hash::BuildHasherDefault, io::Write, mem::swap, rc::Rc, u32};

use fxhash::{FxHashMap, FxHashSet};
use petgraph::graph::{NodeIndex, UnGraph};
use serde::{ser::SerializeMap, Deserialize, Serialize};
use serde_json::Result;

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day24.txt");

    Solution::evaluated(
        "Day 24".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

#[derive(Debug, Clone, Copy)]
enum Equation {
    Value(bool),
    And(u32, u32),
    Or(u32, u32),
    Xor(u32, u32),
}

fn string_to_num(gate: &str) -> u32 {
    let c: Vec<char> = gate.chars().collect();

    ((c[0] as u32) << 16) | ((c[1] as u32) << 8) | (c[2] as u32)
}

fn get_input(file: &'static str) -> FxHashMap<u32, Equation> {
    let mut map = FxHashMap::default();
    let file = fs::read_to_string(file).expect("No file there");
    let (assign, equations) = file.split_once("\n\n").unwrap();

    map.extend(assign.lines().map(|s| {
        let (name, value) = s.split_once(": ").unwrap();
        (string_to_num(name), Equation::Value(value.starts_with('1')))
    }));

    map.extend(equations.lines().map(|e| {
        let (operation, result) = e.split_once(" -> ").unwrap();
        let gate = string_to_num(result);

        let operands: Vec<&str> = operation.split_whitespace().collect();
        let left = string_to_num(operands[0]);
        let right = string_to_num(operands[2]);

        let equation = match operands[1] {
            "AND" => Equation::And(left, right),
            "OR" => Equation::Or(left, right),
            "XOR" => Equation::Xor(left, right),
            other => panic!("Unknown equation {other}",)
        };

        (gate, equation)
    }));

    map
}

macro_rules! cached_or_evaluate {
    ($gate: ident, $table: ident, $cache: ident) => {
        match $cache.get($gate) {
            None => {
                let b = &evaluate($table.get($gate).unwrap(), $table, $cache);
                $cache.insert(*$gate, *b);
                *b
            },
            Some(b) => *b
        }
    };
}

fn evaluate(equation: &Equation, table: &FxHashMap<u32, Equation>, cache: &mut FxHashMap<u32, bool>) -> bool {
    match equation {
        Equation::Value(b) => *b,
        Equation::And(l, r) => {
            cached_or_evaluate!(l, table, cache) && cached_or_evaluate!(r, table, cache)
        },
        Equation::Or(l, r) => {
            cached_or_evaluate!(l, table, cache) || cached_or_evaluate!(r, table, cache)
        },
        Equation::Xor(l, r) => {
            cached_or_evaluate!(l, table, cache) ^ cached_or_evaluate!(r, table, cache)
        },
    }
}


fn solve_first(input: FxHashMap<u32, Equation> ) -> u64 {
    let mut zgates: Vec<u32> = input.keys().filter(|gate| (**gate >> 16) as u8 == b'z').copied().collect();
    zgates.sort();

    let mut num = 0;
    let mut cache: FxHashMap<u32, bool> = FxHashMap::default();

    for z in zgates.iter().rev() {
        num = (num << 1) | evaluate(input.get(z).unwrap(), &input, &mut cache) as u64;
    }

    num
}

fn solve_second(input: FxHashMap<u32, Equation> ) -> u64 {
    0
}
