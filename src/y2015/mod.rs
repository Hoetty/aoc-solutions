use crate::{args::Args, formatting::year};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

#[allow(dead_code)]
pub fn solve_all(args: &Args) -> String {
    let days = if args.day.is_empty() {
        vec![
            day1::solutions(),
            day2::solutions(),
            day3::solutions(),
            day4::solutions(),
            day5::solutions(),
            day6::solutions(),
        ]
    } else {
        args.day.iter().map(|day| match day {
            1 => day1::solutions(),
            2 => day2::solutions(),
            3 => day3::solutions(),
            4 => day4::solutions(),
            5 => day5::solutions(),
            6 => day6::solutions(),
            _ => panic!("Unknown day {day} for year 2015"),
        }).collect()
    };

    year("2015", days, args.redact)
}