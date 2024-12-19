use std::{fmt::{Debug, Display}, marker::PhantomData, time::Instant};

mod y2015;
mod y2024;

pub struct Solution {
    name: String,
    solution_1: Box<dyn Debug>,
    time_1: u128,
    solution_2: Box<dyn Debug>,
    time_2: u128,
}

impl Solution {

    pub fn evaluated<S, T>(name: String, first: &dyn Fn() -> S, second: &dyn Fn() -> T) -> Solution where S: Debug + 'static, T: Debug + 'static {
        let (first, time_first) = time(first);
        let (second, time_second) = time(second);

        Solution { name, solution_1: Box::new(first), time_1: time_first, solution_2: Box::new(second), time_2: time_second }
    }

}

pub fn time<T>(function: &dyn Fn() -> T) -> (T, u128) {
    let start = Instant::now();
    let value = function();
    let elapsed = start.elapsed().as_micros();
    return (value, elapsed);
}

fn main() {
    // y2015::solve_all();
    y2024::solve_all();
}
