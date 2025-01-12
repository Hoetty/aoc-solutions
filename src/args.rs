use clap::Parser;

use crate::{y2015, y2024};

/// Solve all implemented AOC Puzzles \
/// Keep your input files under "./inputs/{year}/day{day}.txt" \
/// Keep your answer file (answers seperated by a newline) under "./expect/{year}/day{day}.txt"
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The years to run
    #[arg(short, long)]
    pub year: Vec<u16>,

    /// The days to run (requires year)
    #[arg(short, long, requires("year"))]
    pub day: Vec<u8>,

    /// Redact solutions from output
    #[arg(short, long)]
    pub redact: bool,

    /// Output to ./output.txt in addition to the terminal
    #[arg(short, long)]
    pub output: bool
}

impl Args {
    pub fn years(&self) -> Vec<String> {
        if self.year.is_empty() {
            vec![y2015::solve_all(self), y2024::solve_all(self)]
        } else {
            self.year.iter().map(|year| match year {
                2015 => y2015::solve_all(self),
                2024 => y2024::solve_all(self),
                _ => panic!("Unknown year {year}")
            }).collect()
        }
    }
}