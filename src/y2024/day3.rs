use std::fs;

use crate::solutions;

solutions!{2024, 3}

fn get_input(file: &str) -> String {
    fs::read_to_string(file).expect("No file there")
}

/// Describes how many characters where matched of the string
/// This is needed, as when no character is matched, the caller may need to
///    advance the pointer manually
#[derive(PartialEq, Eq, Clone, Copy)]
enum MatchStrResult {
    None,
    Some,
    All
}

/// A helper struct to store state and provide convenient lookups during parsing
struct Parser {
    pointer: usize,
    characters: Vec<char>
}


impl Parser {

    /// Tells us if the last character has been consumed
    /// Continueing parsing, when this function returns true will lead to a panic
    fn is_at_end(&self) -> bool {
        self.pointer >= self.characters.len()
    }

    /// Gets the current character without incrementing the internal pointer
    fn peek(&self) -> char {
        self.characters[self.pointer]
    }

    /// Gets the current character and then increments the internal pointer
    fn consume(&mut self) -> char {
        let char = self.peek();
        self.pointer += 1;
        char
    }

    /// Consumes the current character if it is equal to the provided character
    /// Returns if the characters where equal and one was consumed
    fn match_char(&mut self, predicate: char) -> bool {
        if self.peek() == predicate {
            self.consume();
            true
        } else {
            false
        }
    }

    /// Consumes characters as long as they match the corresponding characters from the predicate string
    /// Returns if no characters, some or all have been matched
    /// In case no characters were returned, it may be necessary to manually consume the next character
    fn match_str(&mut self, predicate: &str) -> MatchStrResult {
        let mut result = MatchStrResult::None;

        for predicate_char in predicate.chars() {
            if self.is_at_end() || !self.match_char(predicate_char) {
                return result;
            }

            result = MatchStrResult::Some;
        }

        MatchStrResult::All 
    }

    /// Consumes digit characters and collects the resulting number
    /// The caller needs to check, that the first number is a digit if they wan't a meaningfull result,
    ///    otherwise 0 is returned
    fn parse_number(&mut self) -> u32 {
        let mut number = 0;
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            number = number * 10 + self.consume().to_digit(10).unwrap();
        }

        number
    }

    /// Parses the input and sums all the products of the contained 'mul' instructions
    fn parse_muls(&mut self) -> u32 {
        let mut sum = 0;

        while !self.is_at_end() {
            match self.match_str("mul(") {
                MatchStrResult::None => {
                    self.consume();
                    continue;
                },
                MatchStrResult::Some => continue,
                MatchStrResult::All => { },
            }


            if !self.peek().is_ascii_digit() {
                continue;
            }

            let left = self.parse_number();

            if !self.match_char(',') {
                continue;
            }

            if !self.peek().is_ascii_digit() {
                continue;
            }

            let right = self.parse_number();

            if !self.match_char(')') {
                continue;
            }

            sum += left * right;
        }
        
        sum
    }

    /// Similar to parse_muls
    /// However, when a 'dont'()' is encountered, no muls will be parsed, until a do() is parsed
    fn parse_muls_aware(&mut self) -> u32 {
        let mut sum = 0;
        let mut enabled = true;

        while !self.is_at_end() {
            if enabled {
                if self.match_str("don't()") == MatchStrResult::All {
                    enabled = false;
                    continue;
                }

                match self.match_str("mul(") {
                    MatchStrResult::None => {
                        self.consume();
                        continue;
                    },
                    MatchStrResult::Some => continue,
                    MatchStrResult::All => { },
                }
    
    
                if !self.peek().is_ascii_digit() {
                    continue;
                }
    
                let left = self.parse_number();
    
                if !self.match_char(',') {
                    continue;
                }
    
                if !self.peek().is_ascii_digit() {
                    continue;
                }
    
                let right = self.parse_number();
    
                if !self.match_char(')') {
                    continue;
                }
    
                sum += left * right;
            } else {
                match self.match_str("do()") {
                    MatchStrResult::None => {
                        self.consume();
                    },
                    MatchStrResult::Some => { },
                    MatchStrResult::All => { 
                        enabled = true;
                    },
                }
            }
        }
        
        sum
    }

    /// Constructs a new parser from the input string with the pointer set to zero
    fn new(input: &str) -> Self {
        Parser {
            characters: input.chars().collect(),
            pointer: 0
        }
    }
    
}

/// ### Sum of Products
/// 
/// Scans the input string for all mul(123,123) instructions and sums the resulting products
fn solve_first(input: &str) -> u32 {
    Parser::new(input)
        .parse_muls()
}


/// ### Sum of Products with Awareness
/// 
/// Scans the input string for all mul(123,123) instructions and sums the resulting products,
///    but stops recognising muls after parsing a dont'() instructions until a do() instruction is parsed
fn solve_second(input: &str) -> u32 {
    Parser::new(input)
        .parse_muls_aware()
}
