use crate::time;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub fn solve_all() {
    time("Day 1", &day1::solutions);
    time("Day 2", &day2::solutions);
    time("Day 3", &day3::solutions);
    // time("Day 4", &day4::solutions);
    time("Day 5", &day5::solutions);
    // time("Day 6", &day6::solutions);
}