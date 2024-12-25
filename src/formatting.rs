use std::{fmt::Display, fs, time::Instant};
use tabled::{builder::Builder, settings::{object::{Cell, Columns, Object, Row, Rows, Segment}, themes::Colorization, Alignment, Border, Color, Style}};

#[macro_export]
macro_rules! solutions {
    ($year: expr, $day: expr) => {
        pub fn solutions() -> Solution {
            let filename = format!("inputs/{}/day{}.txt", $year, $day);
            let input = get_input(&filename);
        
            Solution::evaluated(
                format!("Day {}", $day), 
                &|| solve_first(input.clone()),
                &|| solve_second(input.clone())
            )
        }
    }
}

pub struct Solution {
    name: String,
    solution_1: Box<dyn Display>,
    time_1: u128,
    solution_2: Box<dyn Display>,
    time_2: u128,
}

impl Solution {

    pub fn evaluated<S, T>(name: String, first: &dyn Fn() -> S, second: &dyn Fn() -> T) -> Solution where S: Display + 'static, T: Display + 'static {
        let (first, time_first) = time(first);
        let (second, time_second) = time(second);

        Solution { name, solution_1: Box::new(first), time_1: time_first, solution_2: Box::new(second), time_2: time_second }
    }

    pub fn test(&self, year: &str) -> (bool, bool) {
        let filename = self.name.to_lowercase().replace(" ", "");
        let file = fs::read_to_string(format!("./expect/{year}/{filename}.txt")).unwrap();
        let (first, second) = file.split_once("\n").unwrap();

        (self.solution_1.to_string() == first, self.solution_2.to_string() == second)
    }

}

pub fn time<T>(function: &dyn Fn() -> T) -> (T, u128) {
    let start = Instant::now();
    let value = function();
    let elapsed = start.elapsed().as_micros();
    (value, elapsed)
}

pub fn format_time(time: u128) -> String {
    format!("{:.3}ms", time as f64 / 1000.0)
}

pub fn format_percentage(time: u128, total: u128) -> String {
    format!("{:.2}%", time as f64 / total as f64 * 100.0)
}

pub fn format_solution(solution: &dyn Display) -> String {
    format!("{}", solution)
}

pub fn format_test(passed: bool) -> String {
    if passed {
        "✔".to_string()
    } else {
        "✘".to_string()
    }
}

pub fn time_color(time: u128) -> Color {
    if time <= 1000 {
        Color::FG_BRIGHT_GREEN
    } else if time <= 10000 {
        Color::FG_BRIGHT_YELLOW
    } else {
        Color::FG_BRIGHT_RED
    }
}

pub fn year(name: &str, solutions: Vec<Solution>) -> String {
    let total_time: u128 = solutions.iter().map(|s| s.time_1 + s.time_2).sum::<u128>();

    let mut builder = Builder::default();

    builder.push_record(["", "", &format!("Year {name}")]);

    let mut passed_all = true;
    let mut failed: Vec<Cell> = vec![];
    let mut row_colors: Vec<(Row, Color)> = vec![];

    let mut i = 1;

    for solution in &solutions {
        let (passed_1, passed_2) = solution.test(name);
        let passed = passed_1 && passed_2;
        passed_all = passed_all && passed;

        if !passed_1 {
            failed.push(Cell::new(i, 3));
        }

        if !passed_2 {
            failed.push(Cell::new(i + 1, 3));
        }

        if !passed {
            failed.push(Cell::new(i + 2, 3));
        }

        builder.push_record([&solution.name, "#1", &format_solution(&solution.solution_1), &format_test(passed_1) , &format_time(solution.time_1), &format_percentage(solution.time_1, total_time)]);
        builder.push_record(["", "#2", &format_solution(&solution.solution_2), &format_test(passed_2), &format_time(solution.time_2), &format_percentage(solution.time_2, total_time)]);
        builder.push_record(["", "", "", &format_test(passed), &format_time(solution.time_1 + solution.time_2), &format_percentage(solution.time_1 + solution.time_2, total_time)]);
        builder.push_record([""]);

        row_colors.push((Rows::single(i), time_color(solution.time_1)));
        row_colors.push((Rows::single(i + 1), time_color(solution.time_2)));
        row_colors.push((Rows::single(i + 2), time_color(solution.time_1 + solution.time_2)));

        i += 4;
    }

    builder.push_record(["Total", "", "", &format_test(passed_all), &format_time(total_time), &format_percentage(total_time, total_time)]);
    row_colors.push((Rows::single(i), time_color(total_time)));

    if !passed_all {
        failed.push(Cell::new(i, 3));
    }

    let mut table = builder.build();
    table
        .with(Style::re_structured_text())
        .modify(Segment::all(), Alignment::right())
        .modify(Columns::single(2), Alignment::center())
        .modify(Rows::last(), Border::inherit(Style::re_structured_text()).top('='));

    for colorization in row_colors {
        table.with(Colorization::exact([colorization.1], colorization.0.intersect(Columns::new(4..=5))));
    }
        
    table.with(Colorization::exact([Color::FG_BRIGHT_GREEN], Columns::single(3)));

    for cell in failed {
        table.with(Colorization::exact([Color::FG_BRIGHT_RED], cell));
    }

    table.to_string()
}