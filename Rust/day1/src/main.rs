const PUZZLE: &'static str = include_str!("PUZZLE.txt");

fn main() {
    println!("day 1.1: {}", summenize(PUZZLE.chars(), 1));
    println!("day 1.2: {}", summenize(PUZZLE.chars(), PUZZLE.len() / 2));
}

///take an Iterator, zip with an endless version of itself that skips for `skip`.
fn summenize<'a, I>(iter: I, skip: usize) -> u32
where
    I: Iterator<Item = char> + Clone,
{
    iter.clone()
        .zip(iter.cycle().skip(skip))
        .filter(|&(first, second)| first == second)
        .map(|(first, _)| first.to_digit(10).unwrap())
        .sum()
}
