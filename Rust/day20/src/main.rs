#![feature(conservative_impl_trait)]
const PUZZLE: &'static str = include_str!("Input.txt");
use std::str::FromStr;

mod particle;
use particle::*;

fn main() {
    let mut GPU = GPU::from_str(PUZZLE).unwrap();

    for i in 0..1_000_000 {
        GPU.collisionupdate();
        println!("{}", GPU.countparticles());
    }
    //println!("{}", GPU.leftover());
}
