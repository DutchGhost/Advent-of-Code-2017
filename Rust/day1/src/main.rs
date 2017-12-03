const PUZZLE: &'static str = include_str!("PUZZLE.txt");
use std::time::Instant;

fn main() {
    println!("day 1.1: {}", summenize(PUZZLE, 1));
    println!("day 1.2: {}", summenize(PUZZLE, PUZZLE.len() >> 1));
   
    println!("fast: {}", optimized(PUZZLE, PUZZLE.len() >> 1));
}

/// take an &str, loop over the chars,
/// and zip with an infinite version of itself that skips for `skip`.
fn summenize(input: &str, skip: usize) -> u32 {
    input
        .chars()
        .zip(input.chars().cycle().skip(skip))
        .filter_map(|(first, second)| if first == second { first.to_digit(10) } else { None })
        .sum()
}

/// We devide the str in half, check for each element.
/// at the end we bitshift by 1 to the left (multiply by 2),
/// every item we found in the first half, will also be the same fore the second half
fn optimized(input: &str, skip: usize) -> u32 {
    input[..skip]
        .chars()
        .zip(input[skip..].chars())
        .filter_map(|(first, second)| if first == second { first.to_digit(10)} else { None })
        .sum::<u32>() << 1
}