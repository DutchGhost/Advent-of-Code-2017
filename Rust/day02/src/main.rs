extern crate libaoc;

use libaoc::{ToNum, MinMax};

const PUZZLE: &'static str = include_str!("Input.txt");

/// gets the biggest out of the nums.
/// returns None if the bigger one can not be equally devided by the smaller one.
#[inline]
fn is_divisible(a: &u32, b: &u32) -> Option<u32> {
    let (num1, num2) = (a, b).maxmin();
    if num1 % num2 == 0 {
        Some(num1 / num2)
    } else {
        None
    }
}
fn evenly(vec: &[u32]) -> Option<u32> {
    // The inner Iterator can result in a None, if nothing so far is divisible.
    vec.iter()
        .enumerate()
        .filter_map(|(idx, num1)| {
            vec[..idx]
                .iter()
                .filter_map(|num2| is_divisible(num1, num2))
                .next()
        })
        .next()
}

fn difference(nums: &[u32]) -> u32 {
    nums.iter()
        .fold((0u32, std::u32::MAX), |(max, min), item| {
            (std::cmp::max(max, *item), std::cmp::min(min, *item))
        })
        .sub()
}

fn solve() -> (u32, u32) {
    let parsed = PUZZLE.lines().filter_map(|line| line.split_whitespace().to_num().ok()).collect::<Vec<_>>();
    let part1 = parsed.iter().map(|nums| difference(nums)).sum::<u32>();
    let part2 = parsed.iter().filter_map(|nums| evenly(nums)).sum::<u32>();

    (part1, part2)
}
fn main() {
    let (part1, part2) = solve();

    println!("day 2.1: {}", part1);
    println!("day 2.2: {}", part2);
}

trait Sub<T>
where
    T: std::ops::Sub<T>,
{
    fn sub(self) -> <T as std::ops::Sub>::Output;
}

impl<T> Sub<T> for (T, T)
where
    T: std::ops::Sub<T>,
{
    fn sub(self) -> <T as std::ops::Sub>::Output {
        self.0 - self.1
    }
}
