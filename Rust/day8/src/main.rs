#![feature(slice_patterns)]

mod statement;
use statement::*;

const PUZZLE: &'static str = include_str!("Input.txt");

fn main() {
    let mut map = Registers::new();
    for line in PUZZLE.lines() {
        let statement = Statement::from_string_and_map(line, &map);
        match statement {
            Ok(s) => s.eval(&mut map),
            Err(e) => println!("{}", e.discription()),
        }
    }

    let (part1, part2) = map.max();
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
