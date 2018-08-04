#[macro_use]
extern crate libaoc;

extern crate byte_num;

use byte_num::convert::FromAscii;

use libaoc::convert::TryConvert;
use libaoc::MinMax;

const PUZZLE: &'static str = include_str!("Input.txt");
const BPUZZLE: &'static [u8] = include_bytes!("Input.txt");

const INPUT_LEN: usize = 16;

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

fn solve_with_collected_array() -> (u32, u32) {
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
    let part2 = arr.iter().filter_map(|nums| evenly(nums)).sum::<u32>();

    (part1, part2)
}

// #[inline(always)]
// unsafe fn _mm_smax_epi16(vec: __m128i) -> i32 {
//     let mut vmax = vec;
//     // let mut l = _mm_unpacklo_epi16(vec, _mm_setzero_si128());
//     // let mut h = _mm_unpackhi_epi16(vec, _mm_setzero_si128());
//     //
//     // let arr: [u16; 8] = std::mem::transmute(l);
//     // let arr2: [u16; 8] = std::mem::transmute(h);
//     // println!("{:?} {:?}", arr, arr2);
//     //
//     // l = _mm_minpos_epu16(l);
//     // h = _mm_minpos_epu16(h);
//     //
//     //  let arr: [u16; 8] = std::mem::transmute(l);
//     // // let arr2: [u16; 8] = std::mem::transmute(h);
//     //  println!("{:?} {:?}", arr, ());
//     // _mm_extract_epi16(_mm_min_epu16(l, h), 0)

//     // vmax = _mm_max_epi8(vmax, _mm_alignr_epi8(vmax, vmax, 1));
//     // vmax = _mm_max_epi8(vmax, _mm_alignr_epi8(vmax, vmax, 4));
//     // vmax = _mm_max_epi8(vmax, _mm_alignr_epi8(vmax, vmax, 8));
//     // vmax = _mm_max_epi8(vmax, _mm_alignr_epi8(vmax, vmax, 16));
//     //
//     // let arr: [u16; 8] = std::mem::transmute(vmax);
//     // println!("{:?}", arr);
//     // _mm_extract_epi16(vmax, 0)

//     let max1 = _mm_shufflehi_epi16(vec, _MM_SHUFFLE(0,0,3,2) as i32);
//     let max2 = _mm_max_epi32(vec ,max1);
//     let max3 = _mm_shufflelo_epi16(max2, _MM_SHUFFLE(0,0,0,1) as i32);
//     let max4 = _mm_max_epi16(max2,max3);

//     let arr: [u16; 8] = std::mem::transmute(max4);

//     println!("{:?}", arr);

//     _mm_cvtsi128_si32(max4)
// }
// fn solve_simd(bytes: &[u8]) -> (u32, u32) {

//     let mut buffer: [u16; 16] = [0; 16];

//     for line in bytes.split(|b| b == &b'\n') {
//         for (num, buff) in line.split(|b| b == &b'\t').zip(buffer.iter_mut()) {

//             // filter trailing '\r'
//             let parsed = if num.last().filter(|b| b == &&b'\r').is_some() {
//                 u16::atoi(&num[..num.len() - 1])
//             } else {
//                 u16::atoi(num)
//             };
//             *buff = parsed.unwrap();
//         }

//         unsafe {
//             let base_ptr = buffer.as_ptr();

//             let first = _mm_loadu_si128(base_ptr as *const __m128i);
//             let second = _mm_loadu_si128(base_ptr.offset(8) as *const __m128i);

//             let max_vec = _mm_max_epi16(first, second);
//             let max = _mm_smax_epi16(max_vec);

//             //let arr: [u16; 8] = std::mem::transmute(max);
//             println!("{:?}", max);
//         }
//     }
//     (0, 0)
// }

const SIZE: usize = 16;
pub type COLUMN = [u32; SIZE];

#[derive(Debug)]
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

use std::mem;
use std::ptr;

impl Layout {
    fn load(bytes: &[u8]) -> Self {
        let mut me: Self = unsafe { mem::uninitialized() };
        for (idx, row) in bytes.split(|b| b == &b'\n').enumerate() {
            let mut iter = row
                .split(|b| b == &b'\t')
                .map(|b| {
                    if b.last().filter(|b| b == &&b'\r').is_some() {
                        u32::atoi(&b[..b.len() - 1]).unwrap()
                    }
                    else {
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
unsafe fn apply(ptr: *const u32, other: __m128i, f: impl Fn(__m128i, __m128i) -> __m128i) -> __m128i {
    let load = _mm_loadu_si128(ptr as *const __m128i);
    f(load, other)
}

#[inline(always)]
unsafe fn blocks<T>(slice: &[T], blocksize: isize) -> *const T {
    slice.as_ptr().offset(blocksize)
}

#[inline(always)]
unsafe fn compute(layout: &Layout, blocksize: isize, f: impl Fn(__m128i, __m128i) -> __m128i) -> __m128i {
    let block_c1 = blocks(&layout.c1, blocksize);
    let block_c2 = blocks(&layout.c2, blocksize);
    let block_c3 = blocks(&layout.c3, blocksize);
    let block_c4 = blocks(&layout.c4, blocksize);
    let block_c5 = blocks(&layout.c5, blocksize);
    let block_c6 = blocks(&layout.c6, blocksize);
    let block_c7 = blocks(&layout.c7, blocksize);
    let block_c8 = blocks(&layout.c8, blocksize);
    let block_c9 = blocks(&layout.c9, blocksize);
    let block_c10 = blocks(&layout.c10, blocksize);
    let block_c11 = blocks(&layout.c11, blocksize);
    let block_c12 = blocks(&layout.c12, blocksize);
    let block_c13 = blocks(&layout.c13, blocksize);
    let block_c14 = blocks(&layout.c14, blocksize);
    let block_c15 = blocks(&layout.c15, blocksize);
    let block_c16 = blocks(&layout.c16, blocksize);

    
    let simd_c1 = _mm_loadu_si128(block_c1 as *const __m128i);
    let simd_c2 = _mm_loadu_si128(block_c2 as *const __m128i);

    let mut max = f(simd_c1, simd_c2);

    max = apply(block_c3, max, f);
    max = apply(block_c4, max, f);
    max = apply(block_c5, max, f);
    max = apply(block_c6, max, f);
    max = apply(block_c7, max, f);
    max = apply(block_c8, max, f);
    max = apply(block_c9, max, f);
    max = apply(block_c10, max, f);
    max = apply(block_c11, max, f);
    max = apply(block_c12, max, f);
    max = apply(block_c13, max, f);
    max = apply(block_c14, max, f);
    max = apply(block_c15, max, f);
    max = apply(block_c16, max, f);

    let arr: [u32; 4] = mem::transmute(max);
    println!("max = {:?}", arr);
    println!("");

    max
}
fn main() {
    use std::time::Instant;
    let (p1, p2) = solve_with_collected_array();
    println!("day 2.1: {}", p1);
    println!("day 2.2: {}", p2);

    let layout = Layout::load(BPUZZLE);
    
    unsafe {
        let max_block_1 = compute(&layout, 0, _mm_max_epi32);
        let max_block_2 = compute(&layout, 4, _mm_max_epi32);
        let max_block_3 = compute(&layout, 8, _mm_max_epi32);
        let max_block_4 = compute(&layout, 12, _mm_max_epi32);

        let arr1: [u32; 4] = mem::transmute(max_block_1);
        let arr2: [u32; 4] = mem::transmute(max_block_2);
        let arr3: [u32; 4] = mem::transmute(max_block_3);
        let arr4: [u32; 4] = mem::transmute(max_block_4);

        println!("{:?}", arr1);
        println!("{:?}", arr2);
        println!("{:?}", arr3);
        println!("{:?}", arr4);

    }
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
