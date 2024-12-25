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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Value(bool, u32),
    And(Rc<Expr>, Rc<Expr>, u32),
    Or(Rc<Expr>, Rc<Expr>, u32),
    Xor(Rc<Expr>, Rc<Expr>, u32),
}

impl Serialize for Expr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            
        match self {
            Expr::Value(value, num) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("type", "value")?;
                map.serialize_entry("value", value)?;
                map.serialize_entry("num", num)?;
                map.end()
            },
            Expr::And(expr, expr1, num) => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("type", "and")?;
                map.serialize_entry("left", expr.as_ref())?;
                map.serialize_entry("right", expr1.as_ref())?;
                map.serialize_entry("num", num)?;
                map.end()
            },
            Expr::Or(expr, expr1, num) => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("type", "or")?;
                map.serialize_entry("left", expr.as_ref())?;
                map.serialize_entry("right", expr1.as_ref())?;
                map.serialize_entry("num", num)?;
                map.end()
            },
            Expr::Xor(expr, expr1, num) => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("type", "xor")?;
                map.serialize_entry("left", expr.as_ref())?;
                map.serialize_entry("right", expr1.as_ref())?;
                map.serialize_entry("num", num)?;
                map.end()
            },
        }
    }
}

impl Expr {
    fn num(&self) -> u32 {
        *match self {
            Expr::Value(_, num) => num,
            Expr::And(_, _, num) => num,
            Expr::Or(_, _, num) => num,
            Expr::Xor(_, _, num) => num,
        }
    }

    fn left(&self) -> &Rc<Expr> {
        match self {
            Expr::Value(_, _) => panic!("Called left on value"),
            Expr::And(expr, _, _) => expr,
            Expr::Or(expr, _, _) => expr,
            Expr::Xor(expr, _, _) => expr,
        }
    }

    fn right(&self) -> &Rc<Expr> {
        match self {
            Expr::Value(_, _) => panic!("Called right on value"),
            Expr::And(_, expr, _) => expr,
            Expr::Or(_, expr, _) => expr,
            Expr::Xor(_, expr, _) => expr,
        }
    }

    fn enum_value(&self) -> u32 {
        match self {
            Expr::Value(_, _) => 1,
            Expr::And(_, _, _) => 2,
            Expr::Or(_, _, _) => 3,
            Expr::Xor(_, _, _) => 4,
        }
    }

    fn deep_value(&self) -> u32 {
        match self {
            Expr::Value(_, num) => *num,
            Expr::And(expr, expr1, _) => expr.enum_value() + expr1.enum_value(),
            Expr::Or(expr, expr1, _) => expr.enum_value() + expr1.enum_value(),
            Expr::Xor(expr, expr1, _) => expr.enum_value() + expr1.enum_value(),
        }
    }

    fn sort(&mut self) {
        match self {
            Expr::Value(_, _) => { },
            Expr::And(expr, expr1, _) => {
                if expr > expr1 {
                    swap(expr, expr1);
                }
            },
            Expr::Or(expr, expr1, _) => {
                if expr > expr1 {
                    swap(expr, expr1);
                }
            },
            Expr::Xor(expr, expr1, _) => {
                if expr > expr1 {
                    swap(expr, expr1);
                }
            },
        }
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.enum_value().cmp(&other.enum_value()) {
            Ordering::Equal => self.deep_value().cmp(&other.deep_value()),
            cmp => cmp
        })
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

macro_rules! cached_or_parse {
    ($gate: ident, $table: ident, $cache: ident) => {
        match $cache.get($gate) {
            None => {
                let b = Rc::new(parse(*$gate, $table, $cache));
                $cache.insert(*$gate, Rc::clone(&b));
                b
            },
            Some(b) => Rc::clone(b)
        }
    };
}

fn parse(gate: u32, table: &FxHashMap<u32, Equation>, cache: &mut FxHashMap<u32, Rc<Expr>>) -> Expr {
    let mut expr = match table.get(&gate).unwrap() {
        Equation::Value(b) => Expr::Value(*b, gate),
        Equation::And(l, r) => {
            Expr::And(cached_or_parse!(l, table, cache), cached_or_parse!(r, table, cache), gate)
        },
        Equation::Or(l, r) => {
            Expr::Or(cached_or_parse!(l, table, cache), cached_or_parse!(r, table, cache), gate)
        },
        Equation::Xor(l, r) => {
            Expr::Xor(cached_or_parse!(l, table, cache), cached_or_parse!(r, table, cache), gate)
        },
    };
    expr.sort();
    expr
}

fn prefixed_to_num(prefix: char, num: u8) -> u32 {
    ((prefix as u32) << 16) | ((num as u32 / 10 + b'0' as u32) << 8) | (num as u32 % 10 + b'0' as u32)
}

fn build_perfect_adder(nums: u8) -> FxHashMap<u32, Rc<Expr>> {
    let mut built = FxHashMap::default();

    for i in 0..nums {
        let num_xi = prefixed_to_num('x', i);
        let xi = Rc::new(Expr::Value(true, num_xi));
        built.insert(num_xi, Rc::clone(&xi));

        let num_yi = prefixed_to_num('y', i);
        let yi = Rc::new(Expr::Value(true, num_yi));
        built.insert(num_yi, Rc::clone(&yi));

        if i > 0 {
            let num_bi = prefixed_to_num('b', i);
            let bi = Rc::new(Expr::Xor(Rc::clone(&xi), Rc::clone(&yi), num_bi));
            built.insert(num_bi, Rc::clone(&bi));
    
            let num_zi = prefixed_to_num('z', i);
            let zi = Rc::new(Expr::Xor(Rc::clone(built.get(&prefixed_to_num('c', i - 1)).unwrap()), Rc::clone(&bi), num_zi));
            built.insert(num_zi, Rc::clone(&zi));

            if i < nums - 1 {
                let num_ai = prefixed_to_num('a', i);
                let ai = Rc::new(Expr::And(Rc::clone(&xi), Rc::clone(&yi), num_ai));
                built.insert(num_ai, Rc::clone(&ai));

                let num_di = prefixed_to_num('d', i);
                let di = Rc::new(Expr::And(Rc::clone(&ai), Rc::clone(built.get(&prefixed_to_num('c', i - 1)).unwrap()), num_di));
                built.insert(num_di, Rc::clone(&di));

                let num_ci = prefixed_to_num('c', i);
                let ci = Rc::new(Expr::Or(Rc::clone(&ai), Rc::clone(&di), num_ci));
                built.insert(num_ci, Rc::clone(&ci));
            }
        } else {
            let num_bi = prefixed_to_num('z', i);
            let bi = Rc::new(Expr::Xor(Rc::clone(&xi), Rc::clone(&yi), num_bi));
            built.insert(num_bi, Rc::clone(&bi));

            let num_ai = prefixed_to_num('c', i);
            let ai = Rc::new(Expr::And(Rc::clone(&xi), Rc::clone(&yi), num_ai));
            built.insert(num_ai, Rc::clone(&ai));
        }
    }

    built
}

fn compare(test: &Rc<Expr>, valid: &Rc<Expr>, depth: u8) -> Vec<(u8, u32)> {
    if depth == 0 {
        return vec![];
    }

    match test.as_ref() {
        Expr::Value(_, num) => if valid.enum_value() == test.enum_value() && valid.num() == test.num() {
            vec![]
        } else {
            vec![(depth, *num)]
        },
        _ => {
            if valid.enum_value() != test.enum_value() {
                vec![(depth, test.num())]
            } else {
                let mut first = compare(test.left(), valid.left(), depth - 1);
                first.extend(compare(test.right(), valid.right(), depth - 1));

                let mut second = compare(test.right(), valid.left(), depth - 1);
                second.extend(compare(test.left(), valid.right(), depth - 1));

                if first.len() == second.len() {
                    if first.iter().map(|(depth, _)| *depth as u32).sum::<u32>() < second.iter().map(|(depth, _)| *depth as u32).sum() {
                        first
                    } else {
                        second
                    }
                } else if first.len() < second.len() {
                    first
                } else {
                    second
                }
            }
        }
    }
}

fn solve_second(input: FxHashMap<u32, Equation> ) -> u64 {
    let mut zgates: Vec<u32> = input.keys().filter(|gate| (**gate >> 16) as u8 == b'z').copied().collect();
    zgates.sort();

    fs::remove_file("test.json");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("test.json")
        .unwrap();

    fs::remove_file("valid.json");

    let mut file2 = OpenOptions::new()
        .append(true)
        .create(true)
        .open("valid.json")
        .unwrap();

    let mut cache: FxHashMap<u32, Rc<Expr>> = FxHashMap::default();
    let perfect = build_perfect_adder(zgates.len() as u8);

    let mut wrong: FxHashSet<u32> = FxHashSet::with_capacity_and_hasher(8, BuildHasherDefault::default());

    for z in zgates.iter().rev() {
        let tree = Rc::new(parse(*z, &input, &mut cache));

        let this_wrong: Vec<u32> = compare(&tree, perfect.get(z).unwrap(), 6).iter().map(|(_, wrong)| *wrong).collect();

        wrong.extend(
            &this_wrong
        );

        writeln!(file, "{}", serde_json::to_string_pretty(tree.as_ref()).unwrap());
        writeln!(file2, "{}", serde_json::to_string_pretty(perfect.get(z).unwrap().as_ref()).unwrap());
        writeln!(file, "{:?}", this_wrong);
        writeln!(file, "");
        writeln!(file2, "");
    }

    println!("{:?}", wrong);

    0
}
