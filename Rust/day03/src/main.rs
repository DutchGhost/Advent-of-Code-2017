#![feature(generator_trait, generators)]

extern crate libaoc;
mod genitter;

mod spiral;
use spiral::Spiral;

fn isqrt(n: usize) -> usize {
    let mut root = 0;

    while root * root < n {
        root += 2;
    }
    root
}

fn __fst_solve(mut input: usize) -> usize {
    input -= 1;
    let ring = (isqrt(input) + 1) >> 1;
    return (ring as i32 + (input as i32 % (ring as i32 * 2) - ring as i32).abs()) as usize;
}

fn main() {
    let input = 361527;
    let mut spiral = Spiral::new();

    println!("{}", spiral.part1(input));
    spiral.reset();
    println!("{}", spiral.part2(input));

    println!("{}", __fst_solve(input as usize));
}
