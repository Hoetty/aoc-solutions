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

pub fn solve_all() {
    time("2024 Day 1", &day1::solutions);
    time("2024 Day 2", &day2::solutions);
    time("2024 Day 3", &day3::solutions);
    time("2024 Day 4", &day4::solutions);
    time("2024 Day 5", &day5::solutions);
    time("2024 Day 6", &day6::solutions);
    time("2024 Day 7", &day7::solutions);
    time("2024 Day 8", &day8::solutions);
    time("2024 Day 9", &day9::solutions);
    time("2024 Day 10", &day10::solutions);
    time("2024 Day 11", &day11::solutions);
    time("2024 Day 12", &day12::solutions);
    time("2024 Day 13", &day13::solutions);
    time("2024 Day 14", &day14::solutions);
    time("2024 Day 15", &day15::solutions);
    time("2024 Day 16", &day16::solutions);
    time("2024 Day 17", &day17::solutions);
}