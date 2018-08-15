extern crate aoc2017;

const PUZZLE01: &'static str = include_str!("Inputs/day01.txt");   
const PUZZLE02: &'static str = include_str!("Inputs/day02.txt");
const PUZZLE04: &'static str = include_str!("Inputs/day04.txt");

use aoc2017::*;

fn main() {
    println!("{:?}", day01::solve(PUZZLE01));
    println!("{:?}", day02::solve(PUZZLE02));
    println!("{:?}", day03::solve(361527));
    println!("{:?}", day04::solve(PUZZLE04));
}
