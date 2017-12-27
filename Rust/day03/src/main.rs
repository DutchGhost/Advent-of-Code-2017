#![feature(conservative_impl_trait)]
#![feature(generator_trait, generators)]

mod genitter;

mod spiral;
use spiral::Spiral;

mod tmp;
use tmp::Spiral as blah;

fn main() {
    let input = 361527;
    let mut spiral = Spiral::new();
    
    println!("{}", spiral.part1(input));
    spiral.reset();
    println!("{}", spiral.part2(input));

    let t = blah::new();

    t.take(10).for_each(|p| println!("{:?}", p));
}
