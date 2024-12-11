use std::time::Instant;

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

pub fn time(name: &str ,function: &dyn Fn()) {
    let start = Instant::now();
    function();
    println!("{}: {}ms", name, start.elapsed().as_micros() as f64 / 1000.0);
}

fn solve_all() {
    time("Day 1", &day1::solutions);
    time("Day 2", &day2::solutions);
    time("Day 3", &day3::solutions);
    time("Day 4", &day4::solutions);
    time("Day 5", &day5::solutions);
    time("Day 6", &day6::solutions);
    time("Day 7", &day7::solutions);
    time("Day 8", &day8::solutions);
    time("Day 9", &day9::solutions);
    time("Day 10", &day10::solutions);
    time("Day 11", &day11::solutions);
}

fn main() {
    time("Total", &solve_all);
}
