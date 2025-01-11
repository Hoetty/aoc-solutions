use std::{cmp::Ordering, fs, mem::swap, rc::Rc};

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::solutions;

solutions!{2024, 24}

#[derive(Debug, Clone, Copy)]
enum Equation {
    Value(bool),
    And(u32, u32),
    Or(u32, u32),
    Xor(u32, u32),
}

/// Converts a gate string like z43 to its u32 representation
const fn string_to_gate(gate: &str) -> u32 {
    let bytes = gate.as_bytes();

    ((bytes[0] as u32) << 16) | 
        ((bytes[1] as u32) << 8) | 
        (bytes[2] as u32)
}

fn get_input(file: &str) -> FxHashMap<u32, Equation> {
    let file = fs::read_to_string(file).expect("No file there");
    
    let mut connections = FxHashMap::default();
    let (initial_values, equations) = file.split_once("\n\n").unwrap();

    // Parses the initial values of the x and y gates
    connections.extend(initial_values.lines().map(|s| {
        let (gate, value) = s.split_once(": ").unwrap();
        (string_to_gate(gate), Equation::Value(value.starts_with('1')))
    }));

    // Parses the connections from x and y up to z
    connections.extend(equations.lines().map(|e| {
        // Splits x AND y -> z into x AND y, z
        let (operation, output) = e.split_once(" -> ").unwrap();
        let gate = string_to_gate(output);

        // Splits x AND y into x, AND, y
        let operands: Vec<&str> = operation.split_whitespace().collect();
        let left = string_to_gate(operands[0]);
        let right = string_to_gate(operands[2]);

        let equation = match operands[1] {
            "AND" => Equation::And(left, right),
            "OR" => Equation::Or(left, right),
            "XOR" => Equation::Xor(left, right),
            other => panic!("Unknown equation {other}",)
        };

        (gate, equation)
    }));

    connections
}

/// A Helper macro to get the already calculated output of a gate, or compute it if neccessary 
macro_rules! cached_or_evaluate {
    ($gate: ident, $table: ident, $cache: ident) => {
        match $cache.get($gate) {
            None => {
                let value = evaluate($table.get($gate).unwrap(), $table, $cache);
                $cache.insert(*$gate, value);
                value
            },
            Some(b) => *b
        }
    };
}

/// Evaluates an equation using the given equation table and cache
/// For uncached values, this function is recursive until an Equation::Value is hit
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

/// There are 46 zgates from z00 to z45
/// Precompute them for part 1 and 2
const ZGATES: [u32; 46] = {
    let mut zgates = [0; 46];

    let mut num = 0;
    while num < 46 {
        zgates[num as usize] = ((b'z' as u32) << 16) | ((num / 10 + b'0' as u32) << 8) | (num % 10 + b'0' as u32);
        
        num += 1;
    }

    zgates
};

/// ### Device Output
/// 
/// Evaluates all equations and produces a final number where bit at i is the value of zi
fn solve_first(input: &FxHashMap<u32, Equation> ) -> u64 {
    let mut output = 0;
    let mut cache: FxHashMap<u32, bool> = FxHashMap::with_capacity_and_hasher(300, FxBuildHasher);

    for (i, z_output_bit) in ZGATES.iter().enumerate() {
        output |= (evaluate(input.get(z_output_bit).unwrap(), input, &mut cache) as u64) << i;
    }

    output
}

/// Expr in constrast to Equation stores direct references to its operands
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

    /// Gets the output gate of the expr
    fn gate(&self) -> u32 {
        *match self {
            Expr::Value(gate, _) => gate,
            Expr::And(gate, _, _) => gate,
            Expr::Or(gate, _, _) => gate,
            Expr::Xor(gate, _, _) => gate,
        }
    }

    /// Gets the left and right operands of the expr
    #[inline]
    fn sub_exprs(&self) -> (&Rc<Expr>, &Rc<Expr>) {
        match self {
            Expr::Value(_, _) => panic!("Called sub_exprs on value"),
            Expr::And(_, left, right) => (left, right),
            Expr::Or(_, left, right) => (left, right),
            Expr::Xor(_, left, right) => (left, right),
        }
    }

    /// Gets the left operand of the expr
    fn left(&self) -> &Rc<Expr> {
        self.sub_exprs().0
    }

    /// Gets the right operand of the expr
    fn right(&self) -> &Rc<Expr> {
        self.sub_exprs().1
    }

    /// Provides the enum value of the expr, for example for comparision
    fn enum_value(&self) -> u32 {
        match self {
            Expr::Value(_, _) => Self::VALUE,
            Expr::And(_, _, _) => Self::AND,
            Expr::Or(_, _, _) => Self::OR,
            Expr::Xor(_, _, _) => Self::XOR,
        }
    }

    /// Calculates the deep value from both operands
    fn deep_value(&self) -> u32 {
        match self {
            Expr::Value(gate, _) => *gate,
            Expr::And(_, expr, expr1) => expr.enum_value() + expr1.enum_value(),
            Expr::Or(_, expr, expr1) => expr.enum_value() + expr1.enum_value(),
            Expr::Xor(_, expr, expr1) => expr.enum_value() + expr1.enum_value(),
        }
    }

    /// Sorts the operands
    fn sort(&mut self) {
        match self {
            Expr::Value(_, _) => { },
            Expr::And(_, left, right) => {
                if left > right {
                    swap(left, right);
                }
            },
            Expr::Or(_, left, right) => {
                if left > right {
                    swap(left, right);
                }
            },
            Expr::Xor(_, left, right) => {
                if left > right {
                    swap(left, right);
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
        // Order based on the enum value, or the deep value for equal elements
        match self.enum_value().cmp(&other.enum_value()) {
            Ordering::Equal => self.deep_value().cmp(&other.deep_value()),
            cmp => cmp
        }
    }
}

/// Similar to cached_or_evaluate but for parsing expressions instead
macro_rules! cached_or_parse {
    ($gate: ident, $table: ident, $cache: ident) => {
        match $cache.get($gate) {
            None => {
                let expr = Rc::new(parse(*$gate, $table, $cache));
                $cache.insert(*$gate, Rc::clone(&expr));
                expr
            },
            Some(b) => Rc::clone(b)
        }
    };
}

/// Parses equations into an AST, where each node holds a direct reference to its operands
fn parse(gate: u32, table: &FxHashMap<u32, Equation>, cache: &mut FxHashMap<u32, Rc<Expr>>) -> Expr {
    let mut expr = match table.get(&gate).unwrap() {
        Equation::Value(value) => Expr::Value(gate, *value),
        Equation::And(left, right) => {
            Expr::And(gate, cached_or_parse!(left, table, cache), cached_or_parse!(right, table, cache))
        },
        Equation::Or(left, right) => {
            Expr::Or(gate, cached_or_parse!(left, table, cache), cached_or_parse!(right, table, cache))
        },
        Equation::Xor(left, right) => {
            Expr::Xor(gate, cached_or_parse!(left, table, cache), cached_or_parse!(right, table, cache))
        },
    };

    expr.sort();
    expr
}

const Z45: u32 = 8008757;
const Z01: u32 = 8007729;
const X01: u32 = 7876657;

/// Checks if the specified output gate is a z_gate
#[inline(always)]
const fn is_z_gate(gate: u32) -> bool {
    (gate >> 16) & 0xFF == b'z' as u32
}

/// Tests all gates if they are valid and collects all invalid gates into a vector
fn validate(gates: &FxHashMap<u32, Rc<Expr>>) -> Vec<u32> {
    let mut wrong = vec![];

    // zgates that are not the last one (45) MUST be an XOR
    wrong.extend(
        gates.iter().filter(|(gate, expr)| is_z_gate(**gate) && **gate != Z45 && expr.enum_value() != Expr::XOR).map(|(num, _)| num)
    );

    // XOR Gates must either output into a zgate or accept two values as input
    wrong.extend(
        gates.iter().filter(|(gate, expr)| expr.enum_value() == Expr::XOR && !is_z_gate(**gate) && !(expr.left().enum_value() == Expr::VALUE && expr.right().enum_value() == Expr::VALUE)).map(|(num, _)| num)
    );

    // AND Gates not at the start (z01) only output into an OR gate 
    wrong.extend(
        gates.iter().filter(|(gate, expr)| (expr.enum_value() == Expr::AND || expr.enum_value() == Expr::XOR) && expr.left().enum_value() == Expr::AND && **gate != Z01 && expr.right().left().gate() != X01).map(|(_, expr)| expr.left().gate())
    );

    // XOR Gates can't be the input of an OR expr
    wrong.extend(
        gates.iter().filter(|(_, expr)| expr.enum_value() == Expr::OR && expr.right().enum_value() == Expr::XOR).map(|(_, expr)| expr.right().gate())
    );

    wrong
}

/// Converts a gate number back into a string
fn gate_to_str(gate: u32) -> String {
    [
        (gate >> 16) as u8 as char, 
        ((gate >> 8) & 0xFF) as u8 as char, 
        (gate & 0xFF) as u8 as char
    ].iter().collect()
}

/// ### Swapped Outputs
/// 
/// Finds the incorrect outputs in the full adder described by the equations and returns them as sequence
fn solve_second(input: &FxHashMap<u32, Equation> ) -> String {
    let mut cache: FxHashMap<u32, Rc<Expr>> = FxHashMap::with_capacity_and_hasher(400, FxBuildHasher);

    for z_output_bit in ZGATES.iter() {
        let expr = Rc::new(parse(*z_output_bit, input, &mut cache));
        cache.insert(*z_output_bit, expr);
    }

    let mut wrong = validate(&cache);
    wrong.sort();
    wrong.dedup();

    wrong.iter()
        .copied()
        .map(&gate_to_str)
        .collect::<Vec<String>>()
        .join(",")
}
