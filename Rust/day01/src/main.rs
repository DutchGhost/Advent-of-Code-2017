#![feature(test)]
extern crate test;

use std::time::{Duration, Instant};
const BPUZZLE: &'static [u8] = include_bytes!("Input.txt");
const SUB: i8 = 48;

use std::arch::x86_64::{
    __m128i, _mm_and_si128, _mm_cmpeq_epi8, _mm_extract_epi16, _mm_loadu_si128, _mm_sad_epu8,
    _mm_set1_epi8, _mm_setzero_si128, _mm_sub_epi8,
};

fn main() {
    let (ans6, time6) = measure_command(|| solve_simd(BPUZZLE));
    println!("simd: {:?} {:?}", ans6, time6);

    //println!("{:?}", optimized_andpercent(BPUZZLE, BPUZZLE.len() >> 1));
    let (ans7, time7) = measure_command(|| {
        (
            optimized_andpercent(BPUZZLE, BPUZZLE.len() >> 1),
            summmenize_andpercent(BPUZZLE, 1),
        )
    });
    println!("normal: {:?} {:?}", ans7, time7);
}

#[inline]
fn measure_command<T, F: Fn() -> T>(f: F) -> (F::Output, Duration) {
    let start = Instant::now();

    //let mut v = Vec::new();
    for _ in 0..5_000_000 {
        let mut i = f();
        // v.push(i);
    }
    let stop = start.elapsed();

    (f(), stop)
}

///JUST PART 1!
#[inline]
fn summmenize_andpercent(input: &[u8], skip: usize) -> u32 {
    let mut totall: u32 = 0;

    for (c1, c2) in input.iter().zip(input.iter().skip(skip)) {
        totall += ((*c1 as i8 - 48) & -((c1 == c2) as i8)) as u32
    }
    //assert!(input.len() > 2189);
    totall += ((input[0] - 48) as i32 & -((input[0] == input[input.len() - 1]) as i32)) as u32;
    totall
}

///JUST PART 2!
#[inline]
fn optimized_andpercent(input: &[u8], half: usize) -> u32 {
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

#[inline(always)]
unsafe fn _mm_sum_epi8(vec: __m128i) -> i32 {
    let tmp = _mm_sad_epu8(vec, _mm_setzero_si128());
    _mm_extract_epi16(tmp, 0) + _mm_extract_epi16(tmp, 4)
}

fn solve_simd(bytes: &[u8]) -> (u32, u32) {
    const SIMD_SIZE: usize = 128 / 8;

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    // parts 1 is looking 1 ahead
    let mut prv = 0;
    let mut nxt = 1;

    // need to check so we do not overrun the slice
    let len = bytes.len();
    let cap = len - SIMD_SIZE;

    // for part 2, start in the middle
    let mut middle = len >> 1;

    let const_ascii_m128_epi8 = unsafe { _mm_set1_epi8(48) };
    let base_ptr = bytes.as_ptr();

    unsafe {
        // first loop. Finishes half of part 1, and almost part 2
        while middle <= cap {
            let first = _mm_loadu_si128(base_ptr.offset(prv as isize) as *const __m128i);
            let second = _mm_loadu_si128(base_ptr.offset(nxt as isize) as *const __m128i);
            let half = _mm_loadu_si128(base_ptr.offset(middle as isize) as *const __m128i);

            let compared_part1 = _mm_cmpeq_epi8(first, second);
            let compared_part2 = _mm_cmpeq_epi8(first, half);

            // convert ascii to nums
            let nums = _mm_sub_epi8(first, const_ascii_m128_epi8);

            // the comparison gave a __m128i with 0's and 255's. 255 is true, 0 false.
            // any u8 AND 255 is itself again, any u8 AND 0 is 0, so the nums that where not equal are set to 0.
            let valids_part1 = _mm_and_si128(compared_part1, nums);
            let valids_part2 = _mm_and_si128(compared_part2, nums);

            part1 += _mm_sum_epi8(valids_part1) as u32;
            part2 += _mm_sum_epi8(valids_part2) as u32;

            prv += SIMD_SIZE;
            nxt += SIMD_SIZE;
            middle += SIMD_SIZE;
        }
        //println!("{}", part1);
        let fixup = len - middle;

        // Fixup loop for part 2, also continues part 1
        for offset in 0..fixup {
            let prv_unchecked = *bytes.get_unchecked(prv + offset) as i8;
            let nxt_unchecked =
                -((prv_unchecked == *bytes.get_unchecked(nxt + offset) as i8) as i8);
            let middle_unchecked =
                -((prv_unchecked == *bytes.get_unchecked(middle + offset) as i8) as i8);

            part1 += (prv_unchecked - SUB & nxt_unchecked) as u32;
            part2 += (prv_unchecked - SUB & middle_unchecked) as u32;
        }

        prv += fixup + 1;
        nxt += fixup + 1;

        // almost finish part 1
        while nxt <= cap {
            let first = _mm_loadu_si128(base_ptr.offset(prv as isize) as *const __m128i);
            let second = _mm_loadu_si128(base_ptr.offset(nxt as isize) as *const __m128i);

            let compared = _mm_cmpeq_epi8(first, second);

            let nums = _mm_sub_epi8(first, const_ascii_m128_epi8);

            let valids = _mm_and_si128(compared, nums);

            part1 += _mm_sum_epi8(valids) as u32;

            prv += SIMD_SIZE;
            nxt += SIMD_SIZE;
        }

        let fixup = len - nxt;

        // fixup loop part 1
        for offset in 0..fixup {
            let prv_unchecked = *bytes.get_unchecked(prv + offset) as i8;
            let nxt_unchecked =
                -((prv_unchecked == *bytes.get_unchecked(nxt + offset) as i8) as i8);

            part1 += (prv_unchecked - SUB & nxt_unchecked) as u32;
        }

        // check begin and end for part 1
        let begin_unchecked = *bytes.get_unchecked(0) as i8;
        let end_unchecked = -((begin_unchecked == *bytes.get_unchecked(len - 1) as i8) as i8);

        part1 += (begin_unchecked - SUB & end_unchecked) as u32;
    }

    (part1, part2 << 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

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
    fn bench_simd_solve(b: &mut Bencher) {
        b.iter(|| solve_simd(BPUZZLE));
    }

    #[test]
    fn is_ok() {
        assert_eq!(solve_simd(BPUZZLE), (1144, 1194));
    }
}
