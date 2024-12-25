use std::{cmp::Ordering, collections::HashMap, fs::{self, OpenOptions}, hash::BuildHasherDefault, io::Write, mem::swap, rc::Rc, u32};

use fxhash::{FxHashMap, FxHashSet};
use petgraph::graph::{NodeIndex, UnGraph};

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
    Value(u32, bool),
    And(u32, Rc<Expr>, Rc<Expr>),
    Or(u32, Rc<Expr>, Rc<Expr>),
    Xor(u32, Rc<Expr>, Rc<Expr>),
}

impl Expr {

    const VALUE: u32 = 1;
    const AND: u32 = 2;
    const OR: u32 = 3;
    const XOR: u32 = 4;

    fn eval(&self) -> bool {
        match self {
            Expr::Value(_, v) => *v,
            Expr::And(_, expr, expr1) => expr.eval() && expr1.eval(),
            Expr::Or(_, expr, expr1) => expr.eval() || expr1.eval(),
            Expr::Xor(_, expr, expr1) => expr.eval() ^ expr1.eval(),
        }
    }

    fn num(&self) -> u32 {
        *match self {
            Expr::Value(num, _) => num,
            Expr::And(num, _, _) => num,
            Expr::Or(num, _, _) => num,
            Expr::Xor(num, _, _) => num,
        }
    }

    fn left(&self) -> &Rc<Expr> {
        match self {
            Expr::Value(_, _) => panic!("Called left on value"),
            Expr::And(_, expr, _) => expr,
            Expr::Or(_, expr, _) => expr,
            Expr::Xor(_, expr, _) => expr,
        }
    }

    fn right(&self) -> &Rc<Expr> {
        match self {
            Expr::Value(_, _) => panic!("Called right on value"),
            Expr::And(_, _, expr) => expr,
            Expr::Or(_, _, expr) => expr,
            Expr::Xor(_, _, expr) => expr,
        }
    }

    fn enum_value(&self) -> u32 {
        match self {
            Expr::Value(_, _) => Self::VALUE,
            Expr::And(_, _, _) => Self::AND,
            Expr::Or(_, _, _) => Self::OR,
            Expr::Xor(_, _, _) => Self::XOR,
        }
    }

    fn deep_value(&self) -> u32 {
        match self {
            Expr::Value(num, _) => *num,
            Expr::And(_, expr, expr1) => expr.enum_value() + expr1.enum_value(),
            Expr::Or(_, expr, expr1) => expr.enum_value() + expr1.enum_value(),
            Expr::Xor(_, expr, expr1) => expr.enum_value() + expr1.enum_value(),
        }
    }

    fn sort(&mut self) {
        match self {
            Expr::Value(_, _) => { },
            Expr::And(_, expr, expr1) => {
                if expr > expr1 {
                    swap(expr, expr1);
                }
            },
            Expr::Or(_, expr, expr1) => {
                if expr > expr1 {
                    swap(expr, expr1);
                }
            },
            Expr::Xor(_, expr, expr1) => {
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
        Equation::Value(b) => Expr::Value(gate, *b),
        Equation::And(l, r) => {
            Expr::And(gate, cached_or_parse!(l, table, cache), cached_or_parse!(r, table, cache))
        },
        Equation::Or(l, r) => {
            Expr::Or(gate, cached_or_parse!(l, table, cache), cached_or_parse!(r, table, cache))
        },
        Equation::Xor(l, r) => {
            Expr::Xor(gate, cached_or_parse!(l, table, cache), cached_or_parse!(r, table, cache))
        },
    };
    expr.sort();
    expr
}

fn prefixed_to_num(prefix: char, num: u8) -> u32 {
    ((prefix as u32) << 16) | ((num as u32 / 10 + b'0' as u32) << 8) | (num as u32 % 10 + b'0' as u32)
}

fn build_perfect_adder(nums: u8, x: u64, y: u64) -> FxHashMap<u32, Rc<Expr>> {
    let mut built = FxHashMap::default();

    for i in 0..nums {
        let num_xi = prefixed_to_num('x', i);
        let xi = Rc::new(Expr::Value(num_xi, (x >> i) & 1 == 1));
        built.insert(num_xi, Rc::clone(&xi));

        let num_yi = prefixed_to_num('y', i);
        let yi = Rc::new(Expr::Value(num_yi, (y >> i) & 1 == 1));
        built.insert(num_yi, Rc::clone(&yi));

        if i > 0 {
            let num_bi = prefixed_to_num('b', i);
            let bi = Rc::new(Expr::Xor(num_bi, Rc::clone(&xi), Rc::clone(&yi)));
            built.insert(num_bi, Rc::clone(&bi));
    
            let num_zi = prefixed_to_num('z', i);
            let zi = Rc::new(Expr::Xor(num_zi, Rc::clone(built.get(&prefixed_to_num('c', i - 1)).unwrap()), Rc::clone(&bi)));
            built.insert(num_zi, Rc::clone(&zi));

            if i < nums - 1 {
                let num_ai = prefixed_to_num('a', i);
                let ai = Rc::new(Expr::And(num_ai, Rc::clone(&xi), Rc::clone(&yi)));
                built.insert(num_ai, Rc::clone(&ai));

                let num_di = prefixed_to_num('d', i);
                let di = Rc::new(Expr::And(num_di, Rc::clone(built.get(&prefixed_to_num('c', i - 1)).unwrap()), Rc::clone(&bi)));
                built.insert(num_di, Rc::clone(&di));

                let num_ci = prefixed_to_num('c', i);
                let ci = Rc::new(Expr::Or(num_ci, Rc::clone(&ai), Rc::clone(&di)));
                built.insert(num_ci, Rc::clone(&ci));
            }
        } else {
            let num_zi = prefixed_to_num('z', i);
            let zi = Rc::new(Expr::Xor(num_zi, Rc::clone(&xi), Rc::clone(&yi)));
            built.insert(num_zi, Rc::clone(&zi));

            let num_ci = prefixed_to_num('c', i);
            let ci = Rc::new(Expr::And(num_ci, Rc::clone(&xi), Rc::clone(&yi)));
            built.insert(num_ci, Rc::clone(&ci));
        }
    }

    built
}

fn compare(test: &Rc<Expr>, valid: &Rc<Expr>, depth: u8) -> Vec<(u8, u32)> {
    if depth == 0 {
        return vec![];
    }

    match test.as_ref() {
        Expr::Value(num, _) => if valid.enum_value() == test.enum_value() && valid.num() == test.num() {
            vec![]
        } else {
            vec![(depth, *num)]
        },
        _ => {
            let wrong = if valid.enum_value() != test.enum_value() {
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
                } else if first.len() <= second.len() {
                    first
                } else {
                    second
                }
            };

            if wrong.iter().filter(|(test_depth, _)| *test_depth == depth - 1).count() > 1 {
                vec![(depth, test.num())]
            } else {
                wrong
            }
        }
    }
}

fn compare_v2(test: &Rc<Expr>, valid: &Rc<Expr>, depth: u8, known_problems: &mut FxHashSet<u32>) -> Vec<(u8, u32)> {
    if depth == 0 {
        return vec![];
    }

    if known_problems.contains(&test.num()) {
        return vec![];
    }

    if let Expr::Value(num, _) = test.as_ref() {
        if *num == valid.num() {
            return vec![];
        } else {
            return vec![(depth, *num)];
        }
    }

    if test.enum_value() != valid.enum_value() {
        return vec![(depth, test.num())];
    }

    let linear_left = compare_v2(test.left(), valid.left(), depth - 1, known_problems);
    let linear_right = compare_v2(test.right(), valid.right(), depth - 1, known_problems);

    let linear = [linear_left.clone(), linear_right.clone()].concat();

    if linear_left.is_empty() || linear_right.is_empty() {
        return linear;
    }

    let swapped_left = compare_v2(test.left(), valid.right(), depth - 1, known_problems);
    let swapped_right = compare_v2(test.right(), valid.left(), depth - 1, known_problems);
    let swapped = [swapped_left.clone(), swapped_right.clone()].concat();

    if swapped_left.is_empty() || swapped_right.is_empty() {
        return swapped;
    }

    // return linear;
    return vec![(depth, test.num())];

    // if linear.len() < swapped.len() {
    //     return linear;
    // } else if swapped.len() < linear.len() {
    //     return swapped;
    // }

    // if linear[0].0 >= swapped[0].0 {
    //     return linear;
    // } else {
    //     return swapped;
    // }
}

const Z45: u32 = 8008757;
const Z01: u32 = 8007729;
const X01: u32 = 7876657;

#[inline(always)]
fn is_z_num(num: u32) -> bool {
    (num >> 16) & 0xFF == b'z' as u32
}

fn validate(gates: &FxHashMap<u32, Rc<Expr>>) -> Vec<u32> {
    let mut wrong = vec![];

    wrong.extend(
        gates.iter().filter(|(num, expr)| is_z_num(**num) && **num != Z45 && expr.enum_value() != Expr::XOR).map(|(num, _)| num)
    );

    wrong.extend(
        gates.iter().filter(|(num, expr)| expr.enum_value() == Expr::XOR && (**num >> 16) & 0xFF != b'z' as u32 && !(expr.left().enum_value() == Expr::VALUE && expr.right().enum_value() == Expr::VALUE)).map(|(num, _)| num)
    );

    wrong.extend(
        gates.iter().filter(|(num, expr)| (expr.enum_value() == Expr::AND || expr.enum_value() == Expr::XOR) && expr.left().enum_value() == Expr::AND && **num != Z01 && expr.right().left().num() != X01).map(|(_, expr)| expr.left().num())
    );

    wrong.extend(
        gates.iter().filter(|(_, expr)| expr.enum_value() == Expr::OR && expr.right().enum_value() == Expr::XOR).map(|(_, expr)| expr.right().num())
    );

    wrong
}

fn num_to_str(num: u32) -> String {
    [(num >> 16) as u8 as char, ((num >> 8) & 0xFF) as u8 as char, (num & 0xFF) as u8 as char].iter().collect()
}

fn solve_second(input: FxHashMap<u32, Equation> ) -> String {
    println!("{}", string_to_num("z01"));

    let mut zgates: Vec<u32> = input.keys().filter(|gate| (**gate >> 16) as u8 == b'z').copied().collect();
    zgates.sort();

    // let mask = (1 << zgates.len()) - 1;

    // for _ in 0..10000 {
    //     let num1 = rand::random::<u64>() & mask;
    //     let num2 = rand::random::<u64>() & mask;

    //     let adder = build_perfect_adder(zgates.len() as u8, num1, num2);

    //     let mut added = 0;
    //     for i in 0..zgates.len() {
    //         added <<= 1;
    //         let z = prefixed_to_num('z', (zgates.len() - 1 - i) as u8);
    //         if adder.get(&z).unwrap().eval() {
    //             added |= 1;
    //         }
    //     }

    //     println!("{}", if (num1 + num2) & mask != added { panic!() } else { "success" });
    //     println!("{:046b}", (num1 + num2) & mask);
    //     println!("{:046b}", added);
    //     println!("");
        
    // }

    fs::remove_file("test.txt");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("test.txt")
        .unwrap();

    fs::remove_file("valid.txt");

    let mut file2 = OpenOptions::new()
        .append(true)
        .create(true)
        .open("valid.txt")
        .unwrap();

    let mut cache: FxHashMap<u32, Rc<Expr>> = FxHashMap::default();

    for z in zgates.iter() {
        let expr = Rc::new(parse(*z, &input, &mut cache));
        writeln!(file, "{}: {:?}", num_to_str(*z), expr);
        cache.insert(*z, expr);
    }

    let mut wrong = validate(&cache);
    wrong.sort();
    wrong.dedup();

    // let perfect = build_perfect_adder(zgates.len() as u8, 0, 0);

    // let mut wrong: FxHashSet<u32> = FxHashSet::with_capacity_and_hasher(8, BuildHasherDefault::default());

    // for z in zgates.iter() {
    //     let tree = Rc::new(parse(*z, &input, &mut cache));

    //     let this_wrong: Vec<u32> = compare_v2(&tree, perfect.get(z).unwrap(), 7, &mut wrong).iter().map(|(_, wrong)| *wrong).collect();

    //     wrong.extend(
    //         &this_wrong
    //     );

    //     writeln!(file, "{:?}", tree);
    //     writeln!(file, "{:?}", perfect.get(z).unwrap());
    //     writeln!(file, "{:?}", this_wrong);
    //     writeln!(file, "");

    //     // writeln!(file, "{:#?}", tree);
    //     // writeln!(file2, "{:#?}", perfect.get(z).unwrap());
    //     // writeln!(file, "");
    //     // writeln!(file2, "");
    // }

    println!("{:?}", wrong);

    wrong.iter().copied().map(&num_to_str).collect::<Vec<String>>().join(",")
}
