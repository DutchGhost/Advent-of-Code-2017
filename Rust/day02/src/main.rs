#[macro_use]
extern crate libaoc;

extern crate byte_num;

use byte_num::convert::FromAscii;

use libaoc::convert::TryConvert;
use libaoc::MinMax;

const PUZZLE: &'static str = include_str!("Input.txt");
const BPUZZLE: &'static [u8] = include_bytes!("Input.txt");

const INPUT_LEN: usize = 16;

use std::mem;
use std::ptr;

use std::arch::x86_64::*;

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

fn solve_with_collected_array() -> u32 {
    let arr = arraycollect!(
        PUZZLE
            .lines()
            .map(|line|
                arraycollect!(
                    line.split_whitespace()
                        .try_convert_iter()
                        .map(|item| item.unwrap()) => [u32; 16]).unwrap()
        ) => [[u32; 16]; 16])
        .unwrap();

    let part1 = arr.iter().map(|nums| difference(nums)).sum::<u32>();
    //let part2 = arr.iter().filter_map(|nums| evenly(nums)).sum::<u32>();

    part1
}

const SIZE: usize = 16;
pub type COLUMN = [u32; SIZE];

#[derive(Debug)]
#[repr(align(16))]
struct Layout {
    c1: COLUMN,
    c2: COLUMN,
    c3: COLUMN,
    c4: COLUMN,
    c5: COLUMN,
    c6: COLUMN,
    c7: COLUMN,
    c8: COLUMN,
    c9: COLUMN,
    c10: COLUMN,
    c11: COLUMN,
    c12: COLUMN,
    c13: COLUMN,
    c14: COLUMN,
    c15: COLUMN,
    c16: COLUMN,
}

impl Layout {
    fn load(bytes: &[u8]) -> Self {
        let mut me: Self = unsafe { mem::uninitialized() };
        for (idx, row) in bytes.split(|b| b == &b'\n').enumerate() {
            let mut iter = row.split(|b| b == &b'\t').map(|b| {
                if b.last().filter(|b| b == &&b'\r').is_some() {
                    u32::atoi(&b[..b.len() - 1]).unwrap()
                } else {
                    u32::atoi(b).unwrap()
                }
            });

            unsafe {
                ptr::write(me.c1.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c2.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c3.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c4.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c5.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c6.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c7.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c8.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c9.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c10.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c11.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c12.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c13.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c14.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c15.get_unchecked_mut(idx), iter.next().unwrap());
                ptr::write(me.c16.get_unchecked_mut(idx), iter.next().unwrap());
            }
        }
        me
    }
}

#[inline(always)]
unsafe fn apply<F: Fn(__m128i, __m128i) -> __m128i>(
    ptr: *const u32,
    other: __m128i,
    f: F,
) -> __m128i {
    let load = _mm_load_si128(ptr as *const __m128i);
    f(load, other)
}

#[inline(always)]
unsafe fn blocks<T>(slice: &[T], offset: isize) -> *const T {
    slice.as_ptr().offset(offset)
}

#[inline(always)]
unsafe fn compute<F: Fn(__m128i, __m128i) -> __m128i + Copy>(
    layout: &Layout,
    offset: isize,
    f: F,
) -> __m128i {
    let block_c1 = blocks(&layout.c1, offset);
    let block_c2 = blocks(&layout.c2, offset);
    let block_c3 = blocks(&layout.c3, offset);
    let block_c4 = blocks(&layout.c4, offset);
    let block_c5 = blocks(&layout.c5, offset);
    let block_c6 = blocks(&layout.c6, offset);
    let block_c7 = blocks(&layout.c7, offset);
    let block_c8 = blocks(&layout.c8, offset);
    let block_c9 = blocks(&layout.c9, offset);
    let block_c10 = blocks(&layout.c10, offset);
    let block_c11 = blocks(&layout.c11, offset);
    let block_c12 = blocks(&layout.c12, offset);
    let block_c13 = blocks(&layout.c13, offset);
    let block_c14 = blocks(&layout.c14, offset);
    let block_c15 = blocks(&layout.c15, offset);
    let block_c16 = blocks(&layout.c16, offset);

    let simd_c1 = _mm_load_si128(block_c1 as *const __m128i);
    let simd_c2 = _mm_load_si128(block_c2 as *const __m128i);

    let mut result = f(simd_c1, simd_c2);

    result = apply(block_c3, result, f);
    result = apply(block_c4, result, f);
    result = apply(block_c5, result, f);
    result = apply(block_c6, result, f);
    result = apply(block_c7, result, f);
    result = apply(block_c8, result, f);
    result = apply(block_c9, result, f);
    result = apply(block_c10, result, f);
    result = apply(block_c11, result, f);
    result = apply(block_c12, result, f);
    result = apply(block_c13, result, f);
    result = apply(block_c14, result, f);
    result = apply(block_c15, result, f);
    result = apply(block_c16, result, f);

    result
}

#[inline(always)]
unsafe fn _mm_sum_epi32(mut v: __m128i) -> i32 {
    v = _mm_add_epi32(v, _mm_srli_si128(v, 8));
    v = _mm_add_epi32(v, _mm_srli_si128(v, 4));

    _mm_cvtsi128_si32(v)
}

fn solve_simd() -> i32 {
    let max = |a, b| unsafe { _mm_max_epi32(a, b) };
    let min = |a, b| unsafe { _mm_min_epi32(a, b) };

    let layout = Layout::load(BPUZZLE);

    //println!("{}", mem::align_of_val(&layout.c1));

    unsafe {
        let max_block_1 = compute(&layout, 0, max);
        let max_block_2 = compute(&layout, 4, max);
        let max_block_3 = compute(&layout, 8, max);
        let max_block_4 = compute(&layout, 12, max);

        let min_block_1 = compute(&layout, 0, min);
        let min_block_2 = compute(&layout, 4, min);
        let min_block_3 = compute(&layout, 8, min);
        let min_block_4 = compute(&layout, 12, min);

        let diff_block_1 = _mm_sub_epi32(max_block_1, min_block_1);
        let diff_block_2 = _mm_sub_epi32(max_block_2, min_block_2);
        let diff_block_3 = _mm_sub_epi32(max_block_3, min_block_3);
        let diff_block_4 = _mm_sub_epi32(max_block_4, min_block_4);

        let sum_block_1 = _mm_sum_epi32(diff_block_1);
        let sum_block_2 = _mm_sum_epi32(diff_block_2);
        let sum_block_3 = _mm_sum_epi32(diff_block_3);
        let sum_block_4 = _mm_sum_epi32(diff_block_4);

        sum_block_1 + sum_block_2 + sum_block_3 + sum_block_4
    }
}

fn main() {
    use std::time::Instant;
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let p1 = solve_with_collected_array();
    }
    //println!("day 2.1: {}", p1);
    println!("{:?}", start.elapsed());

    let start = Instant::now();
    for _ in 0..1_000_000 {
        let s = solve_simd();
    }
    //println!("day 2.1: {}", s);
    let s = solve_simd();
    println!("day 2.1: {}", s);
 
    println!("{:?}", start.elapsed());
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
