#![feature(conservative_impl_trait)]
fn to_num<I: Iterator<Item=char> + Clone>(iter: I) -> impl Iterator<Item=u32> + Clone {
    iter.map(to_digit)
}

fn to_digit(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

const PUZZLE: &'static str = include_str!("PUZZLE.txt");
fn main() {

    println!("day 1.1: {}", summenize(to_num(PUZZLE.chars()), 1));
    println!("day 1.2: {}", summenize(to_num(PUZZLE.chars()), PUZZLE.len() / 2));
}
///take an Iterator, zip with an endless version of itself that skips for `skip`.
fn summenize<'a, I>(iter: I, skip: usize) -> u32
where
    I: Iterator<Item =u32> + Clone,
{
    iter.clone()
        .zip(iter.cycle().skip(skip))
        .filter(|&(first, second)| first == second)
        .map(|(first, _)| first)
        .sum()
}
