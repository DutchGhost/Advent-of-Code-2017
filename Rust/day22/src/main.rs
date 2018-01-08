extern crate libaoc;

const PUZZLE: &'static str = include_str!("Input.txt");

mod walker;
use walker::*;

fn main() {
    let walker = Walker::new(PUZZLE, Part::Part1);
    let walker2 = Walker::new(PUZZLE, Part::Part2);
    
    println!("part 2: {}", walker.take(10000).sum::<i32>());
    println!("part 1: {}", walker2.take(10_000_000).sum::<i32>());
}
