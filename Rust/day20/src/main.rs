const PUZZLE: &'static str = include_str!("Input.txt");

mod particle;

mod prelude {
    pub use std::str::FromStr;
    pub use particle::GPU;
}

use prelude::*;

fn main() {
    let mut gpu0 = GPU::from_str(PUZZLE).unwrap();
    let mut gpu1 = GPU::from_str(PUZZLE).unwrap();
    for _ in 0..1000 {
        gpu0.update();
        gpu1.collisionupdate();
    }
    println!("part 1: {}", gpu0.closest());
    println!("part 2: {}", gpu1.countparticles());
}
