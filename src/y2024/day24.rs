use std::{cmp::Ordering, fs, mem::swap, rc::Rc};

use rustc_hash::FxHashMap;

use crate::formatting::Solution;
use crate::solutions;

solutions!{2024, 24}

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

fn get_input(file: &str) -> FxHashMap<u32, Equation> {
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


fn solve_first(input: &FxHashMap<u32, Equation> ) -> u64 {
    let mut zgates: Vec<u32> = input.keys().filter(|gate| (**gate >> 16) as u8 == b'z').copied().collect();
    zgates.sort();

    let mut num = 0;
    let mut cache: FxHashMap<u32, bool> = FxHashMap::default();

    for z in zgates.iter().rev() {
        num = (num << 1) | evaluate(input.get(z).unwrap(), input, &mut cache) as u64;
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
        Some(self.cmp(other))
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.enum_value().cmp(&other.enum_value()) {
            Ordering::Equal => self.deep_value().cmp(&other.deep_value()),
            cmp => cmp
        }
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

fn solve_second(input: &FxHashMap<u32, Equation> ) -> String {
    let mut zgates: Vec<u32> = input.keys().filter(|gate| (**gate >> 16) as u8 == b'z').copied().collect();
    zgates.sort();
    let mut cache: FxHashMap<u32, Rc<Expr>> = FxHashMap::default();

    for z in zgates.iter() {
        let expr = Rc::new(parse(*z, input, &mut cache));
        cache.insert(*z, expr);
    }

    let mut wrong = validate(&cache);
    wrong.sort();
    wrong.dedup();

    wrong.iter().copied().map(&num_to_str).collect::<Vec<String>>().join(",")
}
