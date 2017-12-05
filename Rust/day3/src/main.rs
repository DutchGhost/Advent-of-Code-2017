#![feature(conservative_impl_trait)]
#![feature(generator_trait, generators)]

mod genitter;

mod spiral;
use spiral::Spiral;

fn main() {
    let input = 361527;

    let mut part1 = Spiral::new();
    let mut part2 = Spiral::new();
    println!("{}", part1.part1(input));
    println!("{}", part2.part2(input));
}
