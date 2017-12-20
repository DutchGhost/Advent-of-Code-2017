#![feature(conservative_impl_trait)]
const PUZZLE: &'static str = include_str!("Input.txt");

mod particle;
use particle::GPU;

use std::str::FromStr;
fn main() {
    let mut gpu0 = GPU::from_str(PUZZLE).unwrap();
    let mut gpu1 = GPU::from_str(PUZZLE).unwrap();

    loop {
        gpu0.update();
        gpu1.collisionupdate();
        println!("{}\t{}", gpu0.closest(), gpu1.countparticles());
    }
}
