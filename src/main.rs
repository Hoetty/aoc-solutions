use std::time::Instant;

mod y2015;
mod y2024;

pub fn time(name: &str ,function: &dyn Fn()) {
    let start = Instant::now();
    function();
    println!("{} in {}ms\n", name, start.elapsed().as_micros() as f64 / 1000.0);
}

fn main() {
    // time("2015 Total", &y2015::solve_all);
    time("2024 Total", &y2024::solve_all);
}
