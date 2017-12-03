#![feature(conservative_impl_trait)]
#![feature(generator_trait, generators)]

mod genitter;

mod spiral;
use spiral::Spiral;

fn main() {
    let input = 361527;
    let mut spiralizer = Spiral::new();
    println!("{}", spiralizer.part1(input));
}
