#![feature(generators, generator_trait, conservative_impl_trait)]

mod funcs;
mod genitter;

pub mod day01 {
    /// Takes any &str, and a skip.
    /// Returns the sum of all items where the current character equals a character `skip` indexes ahead.
    /// 
    /// #Example
    /// ```
    /// use aoc2017::*;
    /// 
    /// let s = "12344321";
    /// assert_eq!(day01::summenize(s, 1), 5);
    /// ```
    pub fn summenize(input: &str, skip: usize) -> u32 {
        input
            .chars()
            .zip(input.chars().cycle().skip(skip))
            .filter_map(|(first, second)| if first == second { first.to_digit(10) } else { None })
            .sum()
    }

    /// Does the same as `summenize`. Only difference being,
    /// that this only iteratares over half the string,
    /// and then multiplies it's answer by 2.
    /// see [summenize()](fn.summenize.html).
    pub fn optimized(input: &str, half: usize) -> u32 {
        let (head, tail) = input.split_at(half);
        head
            .chars()
            .zip(tail.chars())
            .filter_map(|(first, second)| if first == second { first.to_digit(10)} else { None })
            .sum::<u32>() << 1
    }

    /// Same as `summenize`, but now for bytes.
    #[allow(dead_code)]
    pub fn bytes_summenize(input: &[u8; 2190], skip: usize) -> u32 {
        input
            .iter()
            .zip(input.iter().cycle().skip(skip))
            .filter_map(|(first, second)| if first == second { (*first as char).to_digit(10) } else { None })
            .sum::<u32>()
    }

    ///same as `optimized`, but now for bytes.
    #[allow(dead_code)]
    pub fn bytes_optimized(input: &[u8; 2190], half: usize) -> u32 {
        let (head, tail) = input.split_at(half);
        head
            .iter()
            .zip(tail.iter())
            .filter_map(|(first, second)| if first == second { (*first as char).to_digit(10)} else { None })
            .sum::<u32>() << 1
    }

    pub fn solve(puzzle: &'static str) -> (u32, u32) {
        (summenize(puzzle, 1), optimized(puzzle, puzzle.len() >> 1))
    }
}

pub mod day02 {
    use funcs::{to_vec_of_nums, Sub};

    /// Sorts 2 &'a T's, where T implements Ord.
    #[inline]
    fn sort<'a, T: Ord>(a: &'a T, b: &'a T) -> (&'a T, &'a T) {
        if a > b { (a, b) } else { (b, a) }
    }

    /// Returns the division of the 2 numbers, if they CAN be devided.
    /// Otherwise, returns None.
    #[inline]
    fn is_divisible(a: &u32, b: &u32) -> Option<u32> {
        let (num1, num2) = sort(a, b);
        if num1 % num2 == 0 {
            Some(num1 / num2)
        } else {
            None
        }
    }

    /// An Iterator to return the divisor of the only 2 items that can be divided.
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
    
    /// Iterates over a slice, return the difference between the biggest and lowest number in the slice.
    fn difference(nums: &[u32]) -> u32 {
        nums.iter()
            .fold((0u32, ::std::u32::MAX), |(max, min), item| {
                (::std::cmp::max(max, *item), ::std::cmp::min(min, *item))
            })
            .sub()
    }

    pub fn solve(puzzle: &str) -> (u32, u32) {
        let parsed = puzzle.lines().map(|line| to_vec_of_nums(line)).collect::<Vec<_>>();
        let part1 = parsed.iter().map(|nums| difference(nums)).sum::<u32>();
        let part2 = parsed.iter().filter_map(|nums| evenly(nums)).sum::<u32>();

        (part1, part2)
    }
}

pub mod day03 {
    use funcs::{Spiral};

    pub fn solve(puzzle: i64) -> (i64, i64) {
        let mut spiral = Spiral::new();
        let part1 = spiral.part1(puzzle);
        spiral.reset();
        let part2 = spiral.part2(puzzle);

        (part1, part2)
    }
}

pub mod day04 {
    use funcs::{noop};
    use std::collections::HashSet;

    fn valids<S, F>(input: S, mut transformer: F) -> i64
    where
        S: AsRef<str>,
        F: FnMut(&mut [char])
    {
        let mut valids = 0;
        let mut set = HashSet::with_capacity(11);
        
        for line in input.as_ref().lines() {
            let mut count = 0;
            
            for mut word in line.split_whitespace() {

                let mut chars = word.chars().collect::<Vec<_>>();
                
                transformer(&mut chars);
                set.insert(chars);
                
                count += 1;
            }

            if set.drain().count() == count {
                valids += 1;
            }
        }

        valids
    }

    pub fn solve(puzzle: &str) -> (i64, i64) {
        let part2 = |chars: &mut [char]| { chars.sort() };

        (valids(puzzle, noop), valids(puzzle, part2))
    }
}