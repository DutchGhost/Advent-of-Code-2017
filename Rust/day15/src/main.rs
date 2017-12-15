extern crate rayon;
use rayon::prelude::*;

const VALUE_A: usize = 16807;
const VALUE_B: usize = 48271;

const INPUT_A: usize = 512;
const INPUT_B: usize = 191;

const DEVIDER: usize = 2147483647;

struct Generator {
    value: usize,
    multiplier: usize,
    can_devide: usize
}

impl Generator {
    fn new(value: usize, multiplier: usize, can_devide: usize) -> Generator {
        Generator {
            value: value,
            multiplier: multiplier,
            can_devide: can_devide,
        }
    }
    fn next_with_devide(&mut self) -> usize {
        while let Some(nxt) = self.next() {
            if nxt % self.can_devide == 0 {
                return nxt
            }
        }
        0
    }
}
//maybe swap the remain division.
impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value *= self.multiplier;
        self.value %= DEVIDER;
        Some(self.value)
    }
}

fn part1() -> usize {
    let mut sum = 0;
    let mut generator_a = Generator::new(INPUT_A, VALUE_A, 0);
    let mut generator_b = Generator::new(INPUT_B, VALUE_B, 0);

    generator_a.zip(generator_b).take(40_000_000).filter(|&(next_a, next_b)| (next_a ^ next_b) % 65536 == 0).count()
}

fn part2() -> usize {
    let mut sum = 0;
    let mut generator_a = Generator::new(INPUT_A, VALUE_A, 4);
    let mut generator_b = Generator::new(INPUT_B, VALUE_B, 8);

    for _ in 0..5_000_000 {
        let next_a = generator_a.next_with_devide();
        let next_b = generator_b.next_with_devide();
        
        if (next_a ^ next_b) % 65536 == 0 {
            sum += 1;
        }
    }
    sum
}
fn main() {
   //let (part1, part2) = rayon::join(||part1(), || part2());
   let part1 = part1();
   let part2 = part2();
   println!("part 1: {}", part1);
   println!("part 2: {}", part2);
}