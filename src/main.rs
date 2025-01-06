use std::fs;
use args::Args;
use clap::Parser;
use strip_ansi_escapes::strip;

mod y2015;
mod y2024;

pub mod args;
pub mod formatting;
pub mod util;

fn main() {

    let args = Args::parse();

    let years = args.years();

    let output = years.join("\n\n");

    fs::remove_file("output.txt").unwrap_or(());

    println!("{output}");
    fs::write("output.txt", strip(output)).unwrap_or(());

}
