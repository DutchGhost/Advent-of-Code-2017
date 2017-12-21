#![feature(test)]
extern crate test;
const PUZZLE: &'static str = include_str!("Input.txt");
const BPUZZLE: &'static [u8; 2190] = include_bytes!("Input.txt");

fn main() {
    println!("day 1.1: {}", bytes_summenize(BPUZZLE, 1));
    println!("day 1.2: {}", bytes_optimized(BPUZZLE, PUZZLE.len() >> 1));
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
fn optimized(input: &str, half: usize) -> u32 {
    let (head, tail) = input.split_at(half);
    head
        .chars()
        .zip(tail.chars())
        .filter_map(|(first, second)| if first == second { first.to_digit(10)} else { None })
        .sum::<u32>() << 1
}

fn bytes_summenize(input: &[u8; 2190], skip: usize) -> u32 {
    input
        .iter()
        .zip(input.iter().cycle().skip(skip))
        .filter_map(|(first, second)| if first == second { (*first as char).to_digit(10) } else { None })
        .sum::<u32>()
}

fn bytes_optimized(input: &[u8; 2190], half: usize) -> u32 {
    let (head, tail) = input.split_at(half);
    head
        .iter()
        .zip(tail.iter())
        .filter_map(|(first, second)| if first == second { (*first as char).to_digit(10)} else { None })
        .sum::<u32>() << 1
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;

    #[bench]
    fn bytes_summenize_part1(b: &mut Bencher) {
        b.iter(|| bytes_summenize(BPUZZLE, 1));
    }

    #[bench]
    fn bytes_summenize_part2(b: &mut Bencher) {
        b.iter(|| bytes_summenize(BPUZZLE, BPUZZLE.len() >> 1));
    }

    #[bench]
    fn bytes_fast_part2(b: &mut Bencher) {
        b.iter(|| bytes_optimized(BPUZZLE, BPUZZLE.len() >> 1));
    }

    #[bench]
    fn str_summenize_part1(b: &mut Bencher) {
        b.iter(|| summenize(PUZZLE, 1));
    }

    #[bench]
    fn str_summenize_part2(b: &mut Bencher) {
        b.iter(|| summenize(PUZZLE, PUZZLE.len() >> 1));
    }

    #[bench]
    fn str_fast_part2(b: &mut Bencher) {
        b.iter(|| optimized(PUZZLE, PUZZLE.len() >> 1));
    }
}
