use std::iter::*;
use std::ops::Range;

const PUZZLE: &'static str = "31,2,85,1,80,109,35,63,98,255,0,13,105,254,128,33";
const BYTESPUZZLE: [u8; 49] = *b"31,2,85,1,80,109,35,63,98,255,0,13,105,254,128,33";
const SALT: [u8; 5] = [17, 31, 73, 47, 23];

use std::time::Instant;

fn parse_str(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|word| word.parse().expect("Failed to parse"))
        .collect()
}

fn nums() -> Vec<usize> {
    (0..).take(256).collect()
}

fn parse_bytes(input: [u8; 49]) -> Vec<usize> {
    input
        .into_iter()
        .chain(SALT.iter())
        .map(|b| *b as usize)
        .collect()
}

//wrapper for the 'range' of nums that need to be changed.
enum Wrapper {
    Wrapped(Zip<Chain<Range<usize>, Range<usize>>, Rev<Chain<Range<usize>, Range<usize>>>>),
    Nonwrapped(Zip<Range<usize>, Rev<Range<usize>>>),
}

//'Wrapped' goes from cpos to numslenght, and then from 0 to (len - (numslenght - cpos))
//'Nonwrapped' goes from cpos to cpos + len
fn wrapping(cpos: usize, len: usize, numslenght: usize) -> Wrapper {
    if cpos + len < numslenght {
        Wrapper::Nonwrapped((cpos..cpos + len).zip((cpos..cpos + len).rev()))
    } else {
        
        let already_got = numslenght - cpos;
        let it = (cpos..numslenght).chain(0..(len - already_got));

        Wrapper::Wrapped((cpos..numslenght).chain(0..(len - already_got)).zip(
            (cpos..numslenght).chain(0..(len - already_got)).rev(),
        ))
    }
}
fn solve(rounds: i64, nums: &mut [usize], lenghts: &[usize]) -> usize {
    let mut cpos = 0;
    let mut skipsize = 0;
    let mut numslenght = nums.len();

    for _ in 0..rounds {
        for len in lenghts.iter() {

            let it = wrapping(cpos, *len, numslenght);

            match it {
                Wrapper::Wrapped(iter) => iter.take(len / 2).for_each(|(n1, n2)| nums.swap(n1, n2)),
                Wrapper::Nonwrapped(iter) => iter.take(len / 2).for_each(|(n1, n2)| nums.swap(n1, n2)),
            };

            cpos += (*len + skipsize);
            cpos = cpos % numslenght;
            skipsize += 1;
        }
    }
    nums[0] * nums[1]
}
fn dense(nums: &[usize]) -> String {
    nums.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |n, acc| n ^ acc))
        .map(|chunk| format!("{:02x}", chunk).to_lowercase())
        .collect()
}
fn main() {
    let mut nums_part1 = nums();
    let lenghts_part1 = parse_str(PUZZLE);
    println!("part 1: {}", solve(1, &mut nums_part1, &lenghts_part1));

    let mut nums_part2 = nums();
    let lenghts_part2 = parse_bytes(BYTESPUZZLE);
    solve(64, &mut nums_part2, &lenghts_part2);
    println!("part 2: {}", dense(&nums_part2));
}
