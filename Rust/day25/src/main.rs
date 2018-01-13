const PUZZLE: &'static str = include_str!("Input.txt");
use std::str::FromStr;
mod state;
use state::*;
use state::Block;

fn main() {
    let mut cpu = CPU::new(PUZZLE.split("\n,").map(|block| Block::from_str(block).unwrap()).collect());
    for _ in 0..12794428 {
        cpu.run();
    }
    println!("{:?}", cpu.count_one());
}