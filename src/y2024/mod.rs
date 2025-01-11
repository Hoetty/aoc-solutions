use crate::{args::Args, formatting::year};

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
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn solve_all(args: &Args) -> String {
    let days = if args.day.is_empty() {
        vec![
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
            day19::solutions(),
            day20::solutions(),
            day21::solutions(),
            day22::solutions(),
            day23::solutions(),
            day24::solutions(),
            day25::solutions(),
        ]
    } else {
        args.day.iter().map(|day| match day {
            1 => day1::solutions(),
            2 => day2::solutions(),
            3 => day3::solutions(),
            4 => day4::solutions(),
            5 => day5::solutions(),
            6 => day6::solutions(),
            7 => day7::solutions(),
            8 => day8::solutions(),
            9 => day9::solutions(),
            10 => day10::solutions(),
            11 => day11::solutions(),
            12 => day12::solutions(),
            13 => day13::solutions(),
            14 => day14::solutions(),
            15 => day15::solutions(),
            16 => day16::solutions(),
            17 => day17::solutions(),
            18 => day18::solutions(),
            19 => day19::solutions(),
            20 => day20::solutions(),
            21 => day21::solutions(),
            22 => day22::solutions(),
            23 => day23::solutions(),
            24 => day24::solutions(),
            25 => day25::solutions(),
            _ => panic!("Unknown day {day} for year 2024"),
        }).collect()
    };

    year("2024", days, args.redact)
}