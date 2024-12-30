use std::fs;
use strip_ansi_escapes::strip;

mod y2015;
mod y2024;

pub mod formatting;
pub mod util;

fn main() {
    let years = [
        // y2015::solve_all(),
        y2024::solve_all()
    ];

    let output = years.join("\n\n");

    fs::remove_file("output.txt").unwrap_or(());

    println!("{output}");
    fs::write("output.txt", strip(output)).unwrap_or(());

}
