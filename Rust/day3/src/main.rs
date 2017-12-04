#![feature(conservative_impl_trait)]
#![feature(generator_trait, generators)]

mod genitter;

mod spiral;
use spiral::{Spiral, SpecialSpiral};

fn main() {
    let input = 361527;

    let mut spiralizer = Spiral::new();
    let mut specialspiralizer = SpecialSpiral::new();
    println!("{}", spiralizer.part1(input));
    println!("{}", specialspiralizer.part2(input));
}
