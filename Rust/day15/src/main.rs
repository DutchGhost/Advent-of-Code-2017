const VALUE_A: usize = 16807;
const VALUE_B: usize = 48271;

const INPUT_A: usize = 512;
const INPUT_B: usize = 191;

const DIVIDE_A: usize = 4;
const DIVIDE_B: usize = 8;

const DIVIDER: usize = 2147483647;

struct Generator {
    value: usize,
    multiplier: usize,
}

impl Generator {
    fn new(value: usize, multiplier: usize) -> Generator {
        Generator {
            value: value,
            multiplier: multiplier,
        }
    }
}
//maybe swap the remain division.
impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value *= self.multiplier;
        self.value %= DIVIDER;
        Some(self.value)
    }
}

fn part1() -> usize {
    let generator_a = Generator::new(INPUT_A, VALUE_A);
    let generator_b = Generator::new(INPUT_B, VALUE_B);

    generator_a.zip(generator_b).take(40_000_000).filter(|&(next_a, next_b)| (next_a ^ next_b) % 65536 == 0).count()
}

fn part2() -> usize {
    let generator_a = Generator::new(INPUT_A, VALUE_A);
    let generator_b = Generator::new(INPUT_B, VALUE_B);

    generator_a
        .filter(|item| item % DIVIDE_A == 0)
        .zip(generator_b.filter(|item| item % DIVIDE_B == 0))
        .take(5_000_000)
        .filter(|&(next_a, next_b)| (next_a ^ next_b) % 65536 == 0).count()
}
fn main() {
   //let (part1, part2) = rayon::join(||part1(), || part2());
   let part1 = part1();
   let part2 = part2();
   println!("part 1: {}", part1);
   println!("part 2: {}", part2);
}