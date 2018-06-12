extern crate libaoc;
const PUZZLE: &'static str = include_str!("Input.txt");

use libaoc::convert::TryConvert;

const INPUT_SIZE: usize = 1097;

#[inline]
fn run<F>(mut jumps: [i64; INPUT_SIZE], updater: F) -> i64
where
    F: Fn(i64) -> i64,
{
    let mut n = 0;
    let mut pc: i64 = 0;

    while let Some(offset) = jumps.get_mut(pc as usize) {
        pc += *offset;
        *offset += updater(*offset);
        n += 1;
    }
    n
}

#[inline]
fn fast_run_with_get_mut<F>(mut jumps: [i64; INPUT_SIZE], updater: F) -> i64
where
    F: Fn(i64) -> i64,
{
    let mut n = 0;
    let mut pc = 0;

    loop {
        if let Some(offset) = jumps.get_mut(pc as usize) {
            pc += *offset;
            *offset += updater(*offset);
        } else {
            return n;
        }

        if let Some(offset) = jumps.get_mut(pc as usize) {
            pc += *offset;
            *offset += updater(*offset);
        } else {
            return n + 1;
        }

        n += 2;
    }
}

macro_rules! __unroll {
    ($jumps:expr, $updater:expr, $idx:expr, $n:expr, $plus:expr) => ({
        let offset = $jumps.get_unchecked_mut($idx as usize);
        $idx += *offset;
        *offset += $updater(*offset);
        if $idx as usize >= INPUT_SIZE { return $n + $plus }
    });
    ($jumps:expr, $updater:expr, $idx:expr, $n:expr, $plus:expr $(, $tail:expr)*) => (
        __unroll!($jumps, $updater, $idx, $n, $plus);
        __unroll!($jumps, $updater, $idx, $n $(, $tail)*);
    )
}

#[inline]
fn fast_run<F>(mut jumps: [i64; INPUT_SIZE], updater: F) -> i64
where
    F: Fn(i64) -> i64,
{
    let mut n = 0;
    let mut idx = 0;

    unsafe {
        loop {
            __unroll!(jumps, updater, idx, n, 1, 2, 3, 4, 5, 6, 7, 8);
            n += 8;
        }
    }
}

#[inline(always)]
fn one(_: i64) -> i64 {
    1
}

#[inline(always)]
fn TWO_UGLY_VERSION(n: i64) -> i64 {
    -((n >= 3) as i64) | 1
}
/*
    push rbp
    mov rbp, rsp
    cmp rdi, 2
    mov ecx, 1
    mov rax, -1
    cmovle rax, rcx
    pop rbp
    ret
*/

#[inline(always)]
fn two(n: i64) -> i64 {
    if n >= 3 {
        -1
    } else {
        1
    }
}
/*
    push rbp
    mov rbp, rsp
    cmp rdi, 2
    mov ecx, 1
    mov rax, -1
    cmovle rax, rcx
    pop rbp
    ret
*/
fn main() {
    use std::time::Instant;
    let mut arr: [i64; INPUT_SIZE] = [0; INPUT_SIZE];
    let _ = PUZZLE.lines().try_convert_into_slice(&mut arr);

    println!("day 5.1: {}", fast_run(arr, one));
    println!("day 5.2: {}", fast_run(arr, two));

    println!("day 5.1: {}", fast_run_with_get_mut(arr, one));
    println!("day 5.2: {}", fast_run_with_get_mut(arr, two));
}
