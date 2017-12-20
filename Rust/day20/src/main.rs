const PUZZLE: &'static str = include_str!("Input.txt");
use std::str::FromStr;
mod particle;
use particle::*;

fn main() {
    let mut GPU = GPU::from_str(PUZZLE).unwrap();

    for i in 0..50_000_000 {
        GPU.update();
        if i % 100_000 == 0 {
            println!("{}", i);
        }
    }
    println!("{}", GPU.closest());
}
