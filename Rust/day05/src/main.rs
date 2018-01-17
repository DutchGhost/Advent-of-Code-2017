extern crate libaoc;
const PUZZLE: &'static str = include_str!("Input.txt");

use libaoc::convert::TryConvert;

const INPUT_SIZE: usize = 1097;

fn run<F>(mut jumps: [i64; INPUT_SIZE], updater: F) -> i64
where
    F: Fn(i64) -> i64
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

#[inline(always)]
fn one(_: i64) -> i64 { 1 }

#[inline(always)]
fn TWO_UGLY_VERSION(n: i64) -> i64 { -(( n >= 3) as i64) | 1 }
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
fn two(n: i64) -> i64 {if n >= 3 { -1 } else { 1 }}
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
    let mut arr: [i64; INPUT_SIZE] = [0; INPUT_SIZE];
    PUZZLE.lines().try_convert_into_slice(&mut arr);

    println!("day 5.1: {}", run(arr, one));
    println!("day 5.2: {}", run(arr, two));
}