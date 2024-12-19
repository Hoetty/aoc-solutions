use crate::year;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

#[allow(dead_code)]
pub fn solve_all() -> String {
    year("2015", vec![
        day1::solutions(),
        day2::solutions(),
        day3::solutions(),
        day4::solutions(),
        day5::solutions(),
        day6::solutions(),
    ])    
}