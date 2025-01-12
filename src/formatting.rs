use std::{fmt::Display, fs, ops::BitAnd, time::Instant};
use tabled::{builder::Builder, settings::{object::{Cell, Columns, Object, Row, Rows, Segment}, themes::Colorization, Alignment, Border, Color, Style}};

#[macro_export]
macro_rules! solutions {
    ($year: expr, $day: expr) => {
        use $crate::formatting::time;
        use $crate::formatting::Solution;

        pub fn solutions() -> Solution {
            let filename = format!("inputs/{}/day{}.txt", $year, $day);
            let (input, input_time) = time(|| get_input(&filename));
        
            Solution::evaluated(
                format!("Day {}", $day), 
                || solve_first(&input),
                || solve_second(&input),
                input_time
            )
        }
    }
}

pub struct Solution {
    name: String,
    input_time: u128,
    solution_1: String,
    time_1: u128,
    solution_2: String,
    time_2: u128,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TestResult {
    Success,
    Failure,
    Unknown
}

impl BitAnd for TestResult {
    type Output = TestResult;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self == TestResult::Unknown || rhs == TestResult::Unknown {
            TestResult::Unknown
        } else if self == TestResult::Failure || rhs == TestResult::Failure {
            TestResult::Failure
        } else {
            TestResult::Success
        }
    }
}

impl Solution {

    pub fn evaluated<S: Display, T: Display, F, G>(name: String, first: F, second: G, input_time: u128) -> Solution where F: FnOnce() -> S, G: FnOnce() -> T {
        let (first, time_first) = time(first);
        let (second, time_second) = time(second);

        Solution { name, input_time, solution_1: first.to_string(), time_1: time_first, solution_2: second.to_string(), time_2: time_second }
    }

    pub fn test(&self, year: &str) -> (TestResult, TestResult) {
        let filename = self.name.to_lowercase().replace(" ", "");
        let Ok(file) = fs::read_to_string(format!("./expect/{year}/{filename}.txt")) else {
            return (TestResult::Unknown, TestResult::Unknown);
        };

        let (first, second) = file.split_once("\n").unwrap_or_else(|| panic!("./expect/{year}/{filename}.txt: Please keep your expected inputs in the format <solution1><newline><solution2>"));

        (
            if self.solution_1 == first { TestResult::Success } else { TestResult::Failure }, 
            if self.solution_2 == second { TestResult::Success } else { TestResult::Failure }
        )
    }

}

pub fn time<T, F>(function: F) -> (T, u128) where F: FnOnce() -> T {
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

#[inline(always)]
pub fn format_solution(solution: &str, redact: bool) -> &str {
    if redact { "######" } else { solution }
}

pub fn format_test(passed: TestResult) -> String {
    if passed == TestResult::Success {
        "✔".to_string()
    } else if passed == TestResult::Failure {
        "✘".to_string()
    } else {
        "?".to_string()
    }
}

pub fn time_color(time: u128) -> Color {
    if time < 1000 {
        Color::FG_BRIGHT_GREEN
    } else if time < 10000 {
        Color::FG_BRIGHT_YELLOW
    } else {
        Color::FG_BRIGHT_RED
    }
}

pub fn year(name: &str, solutions: Vec<Solution>, redact: bool) -> String {
    let total_time: u128 = solutions.iter().map(|s| s.time_1 + s.time_2 + s.input_time).sum::<u128>();

    let mut builder = Builder::default();

    builder.push_record(["", "", &format!("Year {name}")]);

    let mut passed_all = TestResult::Success;
    let mut failed: Vec<Cell> = vec![];
    let mut row_colors: Vec<(Row, Color)> = vec![];

    let mut i = 1;

    for solution in &solutions {
        let (passed_1, passed_2) = solution.test(name);
        let passed = passed_1 & passed_2;
        passed_all = passed_all & passed;

        if passed_1 != TestResult::Success {
            failed.push(Cell::new(i + 1, 3));
        }

        if passed_2 != TestResult::Success {
            failed.push(Cell::new(i + 2, 3));
        }

        if passed != TestResult::Success {
            failed.push(Cell::new(i + 3, 3));
        }

        builder.push_record([&solution.name, "I", "", "", &format_time(solution.input_time), &format_percentage(solution.input_time, total_time)]);
        builder.push_record(["", "#1", format_solution(&solution.solution_1, redact), &format_test(passed_1) , &format_time(solution.time_1), &format_percentage(solution.time_1, total_time)]);
        builder.push_record(["", "#2", format_solution(&solution.solution_2, redact), &format_test(passed_2), &format_time(solution.time_2), &format_percentage(solution.time_2, total_time)]);
        builder.push_record(["", "", "", &format_test(passed), &format_time(solution.time_1 + solution.time_2 + solution.input_time), &format_percentage(solution.time_1 + solution.time_2 + solution.input_time, total_time)]);
        builder.push_record([""]);

        row_colors.push((Rows::single(i), time_color(solution.input_time)));
        row_colors.push((Rows::single(i + 1), time_color(solution.time_1)));
        row_colors.push((Rows::single(i + 2), time_color(solution.time_2)));
        row_colors.push((Rows::single(i + 3), time_color(solution.time_1 + solution.time_2 + solution.input_time)));

        i += 5;
    }

    builder.push_record(["Total", "", "", &format_test(passed_all), &format_time(total_time), &format_percentage(total_time, total_time)]);
    row_colors.push((Rows::single(i), time_color(total_time)));

    if passed_all != TestResult::Success {
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