extern crate itertools;

use itertools::Itertools;

const PUZZLE: &'static str = include_str!("PUZZLE.txt");

fn main() {
    println!("day 1.1: {}", summenize(PUZZLE, 1));
    println!("day 1.2: {}", summenize(PUZZLE, PUZZLE.len() / 2));
}

/// take an &str, loop over the chars,
/// and zip with an infinite version of itself that skips for `skip`.
fn summenize(input: &str, skip: usize) -> u32 {
    input
        .chars()
        .zip(input.chars().cycle().dropping(skip))
        .filter_map(|(first, second)| if first == second { first.to_digit(10) } else { None })
        .sum()
}
