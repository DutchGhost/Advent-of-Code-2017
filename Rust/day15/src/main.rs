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
        loop {
            if let Some(nxt) = self.next() {
                if nxt % self.can_devide == 0 {
                    return nxt;
                }
            }
        }
    }
}
//maybe swap the remain division.
impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.multiplier) % DEVIDER;
        Some(self.value)
    }
}

fn compare_bin_string(string1: String, string2: String) -> bool {
    string1.chars().rev().zip(string2.chars().rev()).take(16).all(|(c1, c2)| c1 == c2)
}

fn part1() -> usize {
    let mut sum = 0;
    let mut generator_a = Generator::new(INPUT_A, VALUE_A, 0);
    let mut generator_b = Generator::new(INPUT_B, VALUE_B, 0);

    for _ in 0..40_000_000 {
        let next_a = generator_a.next().unwrap();
        let next_b = generator_b.next().unwrap();

        let stringa = format!("{:032b}", next_a);
        let stringb = format!("{:032b}", next_b);

        if compare_bin_string(stringa, stringb) {
            sum += 1;
        }
    }
    sum
}

fn part2() -> usize {
    let mut sum = 0;
    let mut generator_a = Generator::new(INPUT_A, VALUE_A, 4);
    let mut generator_b = Generator::new(INPUT_B, VALUE_B, 8);

    for _ in 0..5_000_000 {
        let (next_a, next_b) = rayon::join(||generator_a.next_with_devide(),|| generator_b.next_with_devide());
        
        let stringa = format!("{:032b}", next_a);
        let stringb = format!("{:032b}", next_b);

        if compare_bin_string(stringa, stringb) {
            sum += 1;
        }
    }
    sum
}
fn main() {
   let (part1, part2) = rayon::join(||part1(), || part2());
   println!("part 1: {}", part1);
   println!("part 2: {}", part2);
}