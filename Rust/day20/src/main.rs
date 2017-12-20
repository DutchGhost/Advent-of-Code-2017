const PUZZLE: &'static str = include_str!("Input.txt");
use std::str::FromStr;
mod particle;
use particle::*;

fn main() {
    for line in PUZZLE.lines() {
        println!("{:?}", Particle::from_str(line));
    }
}
