use crate::year;

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

pub fn solve_all() -> String {
    year("2024", vec![
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
    ])    
}