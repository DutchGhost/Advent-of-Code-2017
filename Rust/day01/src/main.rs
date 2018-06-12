#![feature(test)]
extern crate test;

use std::time::{Duration, Instant};
const PUZZLE: &'static str = include_str!("Input.txt");
const BPUZZLE: &'static [u8; 2190] = include_bytes!("Input.txt");
const SUB: i8 = 48;

fn main() {
    let (ans, time) =
        measure_command(|| optimized_andpercent_unrolled(BPUZZLE, BPUZZLE.len() >> 1));
    println!("andpercent_unrolled: {}, {:?}", ans, time);

    // let (ans1, time1) = measure_command(|| summmenize_andpercent(BPUZZLE, 1));
    // println!("summenize_andpercent: {} {:?}", ans1, time1);
    //
    // let (ans2, time2) = measure_command(|| optimized_andpercent(BPUZZLE, PUZZLE.len() >> 1));
    // println!("optimized_andpercent: {} {:?}", ans2, time2);
    //
    // let (ans3, time3) = measure_command(|| bytes_summenize(BPUZZLE, 1));
    // println!("part 1.1: {} {:?}", ans3, time3);
    //
    // let (ans4, time4) = measure_command(|| bytes_optimized(BPUZZLE, BPUZZLE.len() >> 1));
    // println!("part 1.2: {} {:?}", ans4, time4);

    let (ans5, time5) = measure_command(|| c_like(BPUZZLE));
    println!("part 1.2: {:?} {:?}", ans5, time5);
    let (ans5, time5) = measure_command(|| testing(BPUZZLE));
    println!("part 1.2: {:?} {:?}", ans5, time5);
    let (ans5, time5) = measure_command(|| more_rust(BPUZZLE));
    println!("part 1.2: {:?} {:?}", ans5, time5);
}

#[inline]
fn measure_command<T, F: Fn() -> T>(f: F) -> (F::Output, Duration) {
    let start = Instant::now();

    let mut v = Vec::new();
    for _ in 0..50_000_000 {
        let mut i = f();
        v.push(i);
    }
    let stop = start.elapsed();

    (f(), stop)
}

///JUST PART 1!
#[inline]
fn summmenize_andpercent(input: &[u8; 2190], skip: usize) -> u32 {
    let mut totall: u32 = 0;

    for (c1, c2) in input.iter().zip(input.iter().skip(skip)) {
        totall += ((*c1 as i8 - 48) & -((c1 == c2) as i8)) as u32
    }
    //assert!(input.len() > 2189);
    totall += ((input[0] - 48) as i32 & -((input[0] == input[2189]) as i32)) as u32;
    totall
}

///JUST PART 2!
#[inline]
fn optimized_andpercent(input: &[u8; 2190], half: usize) -> u32 {
    let (head, tail) = input.split_at(half);

    head.iter()
        .zip(tail.iter())
        .map(|(c1, c2)| ((*c1 as i8 - 48) & -((c1 == c2) as i8)) as u32)
        .sum::<u32>() << 1
}

//JUST PART 1!
/// take an &str, loop over the chars,
/// and zip with an infinite version of itself that skips for `skip`.
#[inline]
fn summenize(input: &str, skip: usize) -> u32 {
    input
        .chars()
        .zip(input.chars().cycle().skip(skip))
        .filter_map(|(first, second)| {
            if first == second {
                first.to_digit(10)
            } else {
                None
            }
        })
        .sum()
}

///JUST PART 2!
/// We devide the str in half, check for each element.
/// at the end we bitshift by 1 to the left (multiply by 2),
/// every item we found in the first half, will also be the same fore the second half
#[inline]
fn optimized(input: &str, half: usize) -> u32 {
    let (head, tail) = input.split_at(half);
    head.chars()
        .zip(tail.chars())
        .filter_map(|(first, second)| {
            if first == second {
                first.to_digit(10)
            } else {
                None
            }
        })
        .sum::<u32>() << 1
}

//JUST PART 1
#[inline]
fn bytes_summenize(input: &[u8; 2190], skip: usize) -> u32 {
    input
        .iter()
        .zip(input.iter().cycle().skip(skip))
        .filter_map(|(first, second)| {
            if first == second {
                (*first as char).to_digit(10)
            } else {
                None
            }
        })
        .sum::<u32>()
}

//JUST PART 2
#[inline]
fn bytes_optimized(input: &[u8; 2190], half: usize) -> u32 {
    let (head, tail) = input.split_at(half);
    head.iter()
        .zip(tail.iter())
        .filter_map(|(first, second)| {
            if first == second {
                (*first as char).to_digit(10)
            } else {
                None
            }
        })
        .sum::<u32>() << 1
}

//JUST PART 2
#[inline]
fn optimized_andpercent_unrolled(input: &[u8; 2190], _: usize) -> u32 {
    let mut totall = 0;
    unsafe {
        let head = input.get_unchecked(0..1095);
        let tail = input.get_unchecked(1095..);

        head.chunks(15).zip(tail.chunks(15)).for_each(|(lhs, rhs)| {
            totall += (((*lhs.get_unchecked(0) as i8 - SUB)
                & -((*lhs.get_unchecked(0) == *rhs.get_unchecked(0)) as i8))
                + ((*lhs.get_unchecked(1) as i8 - SUB)
                    & -((*lhs.get_unchecked(1) == *rhs.get_unchecked(1)) as i8))
                + ((*lhs.get_unchecked(2) as i8 - SUB)
                    & -((*lhs.get_unchecked(2) == *rhs.get_unchecked(2)) as i8))
                + ((*lhs.get_unchecked(3) as i8 - SUB)
                    & -((*lhs.get_unchecked(3) == *rhs.get_unchecked(3)) as i8))
                + ((*lhs.get_unchecked(4) as i8 - SUB)
                    & -((*lhs.get_unchecked(4) == *rhs.get_unchecked(4)) as i8))
                + ((*lhs.get_unchecked(5) as i8 - SUB)
                    & -((*lhs.get_unchecked(5) == *rhs.get_unchecked(5)) as i8))
                + ((*lhs.get_unchecked(6) as i8 - SUB)
                    & -((*lhs.get_unchecked(6) == *rhs.get_unchecked(6)) as i8))
                + ((*lhs.get_unchecked(7) as i8 - SUB)
                    & -((*lhs.get_unchecked(7) == *rhs.get_unchecked(7)) as i8))
                + ((*lhs.get_unchecked(8) as i8 - SUB)
                    & -((*lhs.get_unchecked(8) == *rhs.get_unchecked(8)) as i8))
                + ((*lhs.get_unchecked(9) as i8 - SUB)
                    & -((*lhs.get_unchecked(9) == *rhs.get_unchecked(9)) as i8))
                + ((*lhs.get_unchecked(10) as i8 - SUB)
                    & -((*lhs.get_unchecked(10) == *rhs.get_unchecked(10)) as i8))
                + ((*lhs.get_unchecked(11) as i8 - SUB)
                    & -((*lhs.get_unchecked(11) == *rhs.get_unchecked(11)) as i8))
                + ((*lhs.get_unchecked(12) as i8 - SUB)
                    & -((*lhs.get_unchecked(12) == *rhs.get_unchecked(12)) as i8))
                + ((*lhs.get_unchecked(13) as i8 - SUB)
                    & -((*lhs.get_unchecked(13) == *rhs.get_unchecked(13)) as i8))
                + ((*lhs.get_unchecked(14) as i8 - SUB)
                    & -((*lhs.get_unchecked(14) == *rhs.get_unchecked(14)) as i8)))
                as u32;
        });
    }
    totall << 1
}

//PART 1 AND 2 IN ONE!
fn testing(input: &[u8; 2190]) -> (u32, u32) {
    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut iter = input.windows(2);

    for (idx, chunk) in (&mut iter).enumerate().take(1095) {
        unsafe {
            part_1 += ((*chunk.get_unchecked(0) as i8 - SUB)
                & -((chunk.get_unchecked(0) == chunk.get_unchecked(1)) as i8))
                as u32;
            part_2 += ((*chunk.get_unchecked(0) as i8 - SUB)
                & -((chunk.get_unchecked(0) == input.get_unchecked(idx + 1095)) as i8))
                as u32;
        }
    }

    for chunk in iter {
        unsafe {
            part_1 += ((*chunk.get_unchecked(0) as i8 - SUB)
                & -((chunk.get_unchecked(0) == chunk.get_unchecked(1)) as i8))
                as u32;
        }
    }
    unsafe {
        part_1 += ((*input.get_unchecked(0) as i8 - SUB)
            & -((input.get_unchecked(0) == input.get_unchecked(2189)) as i8))
            as u32;
    }

    (part_1, part_2 << 1)
}

//PART 1 AND 2 IN ONE!
fn more_rust(input: &[u8; 2190]) -> (u32, u32) {
    let mut part_1 = 0;
    let mut part_2 = 0;

    unsafe {
        let head = input.get_unchecked(..);
        let tail = input.get_unchecked(1095..);
        let mut n: i64 = 0;
        let mut part_1_iter = head.windows(2);

        for (half, chunk) in tail.iter().zip(&mut part_1_iter) {
            part_1 += ((*chunk.get_unchecked(0) as i8 - SUB)
                & -((chunk.get_unchecked(0) == chunk.get_unchecked(1)) as i8))
                as u32;
            part_2 += ((*chunk.get_unchecked(0) as i8 - SUB)
                & -((chunk.get_unchecked(0) == half) as i8)) as u32;
            n += 1;
        }
        for chunk in part_1_iter {
            part_1 += ((*chunk.get_unchecked(0) as i8 - SUB)
                & -((chunk.get_unchecked(0) == chunk.get_unchecked(1)) as i8))
                as u32;
        }
    }
    unsafe {
        part_1 += ((*input.get_unchecked(0) as i8 - SUB)
            & -((input.get_unchecked(0) == input.get_unchecked(2189)) as i8))
            as u32;
    }
    (part_1, part_2 << 1)
}

//PART 1 AND 2 IN ONE!
fn c_like(input: &[u8; 2190]) -> (u32, u32) {
    let mut part_1: u32 = 0;
    let mut part_2: u32 = 0;

    let mut prv = 0;
    let mut nxt = 1;
    let mut half = 2190 >> 1;

    while half != 2190 {
        unsafe {
            part_1 += ((*input.get_unchecked(prv) as i8 - SUB)
                & -((input.get_unchecked(prv) == input.get_unchecked(nxt)) as i8))
                as u32;
            part_1 += ((*input.get_unchecked(prv + 1) as i8 - SUB)
                & -((input.get_unchecked(prv + 1) == input.get_unchecked(nxt + 1)) as i8))
                as u32;
            part_1 += ((*input.get_unchecked(prv + 2) as i8 - SUB)
                & -((input.get_unchecked(prv + 2) == input.get_unchecked(nxt + 2)) as i8))
                as u32;
            part_1 += ((*input.get_unchecked(prv + 3) as i8 - SUB)
                & -((input.get_unchecked(prv + 3) == input.get_unchecked(nxt + 3)) as i8))
                as u32;
            part_1 += ((*input.get_unchecked(prv + 4) as i8 - SUB)
                & -((input.get_unchecked(prv + 4) == input.get_unchecked(nxt + 4)) as i8))
                as u32;

            part_2 += ((*input.get_unchecked(prv) as i8 - SUB)
                & -((input.get_unchecked(prv) == input.get_unchecked(half)) as i8))
                as u32;
            part_2 += ((*input.get_unchecked(prv + 1) as i8 - SUB)
                & -((input.get_unchecked(prv + 1) == input.get_unchecked(half + 1)) as i8))
                as u32;
            part_2 += ((*input.get_unchecked(prv + 2) as i8 - SUB)
                & -((input.get_unchecked(prv + 2) == input.get_unchecked(half + 2)) as i8))
                as u32;
            part_2 += ((*input.get_unchecked(prv + 3) as i8 - SUB)
                & -((input.get_unchecked(prv + 3) == input.get_unchecked(half + 3)) as i8))
                as u32;
            part_2 += ((*input.get_unchecked(prv + 4) as i8 - SUB)
                & -((input.get_unchecked(prv + 4) == input.get_unchecked(half + 4)) as i8))
                as u32;
        }

        half += 5;
        prv += 5;
        nxt += 5;
    }

    while nxt != 2190 {
        unsafe {
            part_1 += ((*input.get_unchecked(prv) as i8 - SUB)
                & -((input.get_unchecked(prv) == input.get_unchecked(nxt)) as i8))
                as u32;
            part_1 += ((*input.get_unchecked(prv + 1) as i8 - SUB)
                & -((input.get_unchecked(prv + 1) == input.get_unchecked(nxt + 1)) as i8))
                as u32;
        }
        nxt += 2;
        prv += 2;
    }
    unsafe {
        part_1 += ((*input.get_unchecked(0) as i8 - SUB)
            & -((input.get_unchecked(0) == input.get_unchecked(2189)) as i8))
            as u32;
    }

    (part_1, part_2 << 1)
}
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

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

    #[bench]
    fn bytes_summenize_part1_andpercent(b: &mut Bencher) {
        b.iter(|| summmenize_andpercent(BPUZZLE, 1));
    }

    #[bench]
    fn bytes_optimized_part2_andpercent(b: &mut Bencher) {
        b.iter(|| optimized_andpercent(BPUZZLE, PUZZLE.len() >> 1));
    }

    #[bench]
    fn unrolled_optimized_part2_andpercent(b: &mut Bencher) {
        b.iter(|| optimized_andpercent_unrolled(BPUZZLE, BPUZZLE.len() >> 1));
    }

    #[bench]
    fn bench_more_rust(b: &mut Bencher) {
        b.iter(|| more_rust(BPUZZLE));
    }

    #[bench]
    fn bench_c_like(b: &mut Bencher) {
        b.iter(|| c_like(BPUZZLE));
    }

    #[bench]
    fn bench_testing(b: &mut Bencher) {
        b.iter(|| testing(BPUZZLE));
    }
}
