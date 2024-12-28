use std::fs;

use crate::solutions;

solutions!{2024, 2}

type List = Vec<Vec<i8>>;

/// Parses a file line by line
/// Each line ("report") contains multiple numbers ("levels") 
fn get_input(file: &str) -> List {
    let content = fs::read_to_string(file).expect("Input file is missing");
    content.lines()
        .map(|report| report.split_whitespace()
            .map(|level| level.parse().expect("Level must be numeric")).collect::<Vec<i8>>()
        ).collect::<List>()
}

/// Checks if all difference are decreasing, indicated by them being negative
#[inline(always)]
fn all_decreasing(diffs: &[i8]) -> bool {
    diffs.iter().all(|diff| diff.is_negative())
}

/// Checks if all differences are increasing, indicated by them being positive
#[inline(always)]
fn all_increasing(diffs: &[i8]) -> bool {
    diffs.iter().all(|diff| diff.is_positive())
}

/// Checks if all differences are gradual, meaning their absolute value is at most 3
#[inline(always)]
fn all_gradual(diffs: &[i8]) -> bool {
    diffs.iter().all(|diff| diff.abs() <= 3)
}

/// ### Number of safe reports
/// 
/// Identify all reports, whose levels are either all gradually decreasing or all gradually increasing
///    and that change at least 1 and at most 3
fn solve_first(input: &List) -> usize {
    input.iter()
        .map(|report| report.windows(2).map(|levels| levels[0] - levels[1]).collect::<Vec<i8>>())
        .filter(|diffs| all_decreasing(diffs) || all_increasing(diffs))
        .filter(|diffs| all_gradual(diffs))
        .count()
}

/// ### Number of safe reports, with Problem Dampener
/// 
/// The same rules apply, as for part 1, but if a single level can be removed to make the report safe,
///    it is considered safe too
fn solve_second(input: &List) -> usize {
    let count_all = input.len();

    // Collect all bad reports
    let bad: Vec<&Vec<i8>> = input.iter()
        .map(|report| (report, report.windows(2).map(|levels| levels[0] - levels[1]).collect::<Vec<i8>>()))
        .filter(|(_, diffs)| !all_decreasing(diffs) && !all_increasing(diffs) || !all_gradual(diffs))
        .map(|(report, _)| report)
        .collect();

    let mut bad_count = bad.len();

    // Foreach bad report, test if removing a single level results in it becoming good
    // If this check succeeds, decrement the badcounter and move on
    for bad_report in bad {
        let number_of_levels = bad_report.len();
        for i in 0..number_of_levels {

            let diffs: Vec<i8> = bad_report
                .iter()
                .enumerate()
                .filter_map(|(test_i, diff)| if test_i != i { Some(diff) } else { None })
                .collect::<Vec<_>>()
                .windows(2)
                .map(|levels| levels[0] - levels[1])
                .collect();

            if (all_decreasing(&diffs) || all_increasing(&diffs)) && all_gradual(&diffs) {
                bad_count -= 1;
                break;
            }
    
        }
    }

    count_all - bad_count
}
