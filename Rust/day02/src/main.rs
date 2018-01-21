#[macro_use]
extern crate libaoc;

use libaoc::convert::TryConvert;
use libaoc::MinMax;

const PUZZLE: &'static str = include_str!("Input.txt");

const INPUT_LEN: usize = 16;

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
    let mut arr: [[u32; INPUT_LEN]; INPUT_LEN] = [[0; INPUT_LEN]; INPUT_LEN];

    let parsed = PUZZLE.lines();

    for (row, line) in arr.iter_mut().zip(parsed) {
        line.split_whitespace().try_convert_into_slice(row).unwrap();
    }

    let part1 = arr.iter().map(|nums| difference(nums)).sum::<u32>();
    let part2 = arr.iter().filter_map(|nums| evenly(nums)).sum::<u32>();

    (part1, part2)
}

fn solve_with_collected_array() -> (u32, u32) {
    let arr = arraycollect!(
        PUZZLE
            .lines()
            .map(|line|
                arraycollect!(
                    line.split_whitespace()
                        .try_convert_iter()
                        .map(|item| item.unwrap()) => [u32; 16]).unwrap()
        ) => [[u32; 16]; 16]).unwrap();

    let part1 = arr.iter().map(|nums| difference(nums)).sum::<u32>();
    let part2 = arr.iter().filter_map(|nums| evenly(nums)).sum::<u32>();

    (part1, part2)
}
fn main() {
    use std::time::Instant;

    let b = Instant::now();
    for _ in 0..1_000_000 {
        let (_, _) = solve_with_collected_array();
    }
    println!("{:?}", b.elapsed());



    let f = Instant::now();
    for _ in 0..1_000_000 {
        let (_, _) = solve();
    }
    println!("{:?}", f.elapsed());

    let (p1, p2) = solve();
    println!("day 2.1: {}", p1);
    println!("day 2.2: {}", p2);
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
    #[inline]
    fn sub(self) -> <T as std::ops::Sub>::Output {
        self.0 - self.1
    }
}
