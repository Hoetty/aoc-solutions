use std::fs;

use crate::solutions;

solutions!{2024, 5}

type Rules = Vec<u128>;
type Updates = Vec<Vec<usize>>;

/// Collect the ordering rules into a vec of u128,
///    where the index into the vec is the left page, and all set bits indicate right pages
/// 
/// The updates are parsed as a vec of vecs of usizes
fn get_input(file: &str) -> (Rules, Updates) {
    let file = fs::read_to_string(file).expect("No file there");
    let (rule_string, updates_string) = file.split_once("\n\n").unwrap();

    let collected_rules = rule_string.lines().map(|line| {
        let (left, right) = line.split_once("|").unwrap();

        (left.parse().unwrap(), right.parse().unwrap())
    }).collect::<Vec<(usize, usize)>>();

    let mut rules: Rules = vec![0;100];

    for (left_page, right_page) in collected_rules {
        // Set the bit corresponding to the right page in the left pages number
        rules[left_page] |= 1 << right_page;
    }

    (
        rules,
        updates_string.lines()
            .map(|line| line.split_terminator(",")
                .map(|page_number| page_number.parse().unwrap())
                .collect()
            ).collect()
    )
}

/// ### Correctly Ordered Pages
/// 
/// Sum the middle number of all updates whose pages are ordered according to the given ruleset of page orderings
fn solve_first(input: &(Rules, Updates)) -> usize {
    let (rules, updates) = input;

    let mut sum = 0;

    // For each update, check for each page all preceeding pages, if they arent violating any rules
    'outer: for update in updates {
        for i in 0..update.len() {
            let right_page = update[i];
            for left_page in update.iter().take(i) {
                // Check if the page currently on the right has a rule, that states 
                //    it should be on the left of the currently left page
                // If so, then continue with the next update, as this one is not ordered correctly
                if rules[right_page] & (1 << left_page) != 0 {
                    continue 'outer;
                }
            }
        }

        // Add the middle number to the sum
        let middle_page_number = update[update.len() / 2];
        sum += middle_page_number;
    }

    sum
}

/// ### Corrected Page Orderings
/// 
/// Sum the middle numbers of all updates whose pages where corrected to respect the ruleset of page orderings
fn solve_second(input: &(Rules, Updates)) -> usize {
    let (rules, updates) = input;
    let updates = updates.clone();

    let mut sum = 0;

    // Similar to part 1, but swap any violators to reestablish order
    for mut update in updates {
        let mut needed_correction = false;

        for i in 0..update.len() {
            let right_page = update[i];
            for j in 0..i {
                // If two pages dont fit a rule, then swap them and check again, starting a page further left
                if rules[right_page] & (1 << update[j]) != 0 {
                    update.swap(i, j);
                    needed_correction = true;
                }
            }
        }
        
        // Only if a swap has occured, add the middle page number
        if needed_correction {
            let middle_page_number = update[update.len() / 2];
            sum += middle_page_number;
        }
    }

    sum
}
