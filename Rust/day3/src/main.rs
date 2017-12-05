#![feature(conservative_impl_trait)]
#![feature(generator_trait, generators)]

mod genitter;

mod spiral;
use spiral::Spiral;

fn main() {
    let input = 361527;
    let mut spiral = Spiral::new();
    println!("{}", spiral.part1(input));

    spiral.reset();
    
    println!("{}", spiral.part2(input));
}
