use std::fs;

use tabled::{builder::Builder, settings::{object::{Columns, Rows, Segment}, Alignment, Panel, Style}, Table};

use crate::time;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;

pub fn solve_all() {

    let solutions = vec![
        day1::solutions(),
        day2::solutions(),
        day3::solutions(),
        day4::solutions(),
        day5::solutions(),
        day6::solutions(),
        day7::solutions(),
        day8::solutions(),
        day9::solutions(),
        day10::solutions(),
        day11::solutions(),
        day12::solutions(),
        day13::solutions(),
        day14::solutions(),
        day15::solutions(),
        day16::solutions(),
        day17::solutions(),
        day18::solutions(),
    ];

    let total_time: f64 = solutions.iter().map(|s| s.time_1 + s.time_2).sum::<u128>() as f64 / 1000.0;

    let mut builder = Builder::default();

    for solution in solutions {
        builder.push_record([&solution.name, "#1", &format!("{:?}", solution.solution_1), &format!("{}ms", solution.time_1 as f64 / 1000.0), &format!("{:.2}%", solution.time_1 as f64 / 10.0 / total_time)]);
        builder.push_record(["", "#2", &format!("{:?}", solution.solution_2), &format!("{}ms", solution.time_2 as f64 / 1000.0), &format!("{:.2}%", solution.time_2 as f64 / 10.0 / total_time)]);
        builder.push_record(["", "T", "", &format!("{}ms", (solution.time_1 + solution.time_2) as f64 / 1000.0), &format!("{:.2}%", (solution.time_1 + solution.time_2) as f64 / 10.0 / total_time)]);
        builder.push_record([""]);
    }

    let table = builder.build()
            .with(Panel::header("Year 2024"))
            .with(Panel::footer(format!("Total Time: {total_time}ms")))
            .with(Style::sharp())
            .modify(Segment::all(), Alignment::right())
            .modify(Rows::first(), Alignment::center())
            .modify(Rows::last(), Alignment::center())
            .modify(Columns::single(2), Alignment::center())
            .to_string();

    println!("{table}");
    fs::write("output.txt", format!("{table}")).unwrap();
    
}