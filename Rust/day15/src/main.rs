const VALUE_A: i64 = 16807;
const VALUE_B: i64 = 48271;

const INPUT_A: i64 = 512;
const INPUT_B: i64 = 191;

const DIVIDE_A: i64 = 4;
const DIVIDE_B: i64 = 8;

const DIVIDER: i64 = 2147483647;

struct Generator {
    value: i64,
    multiplier: i64,
}

impl Generator {
    fn new(value: i64, multiplier: i64) -> Generator {
        Generator {
            value: value,
            multiplier: multiplier,
        }
    }
}

impl Iterator for Generator {
    type Item = i64;

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

//maybe add 'item / (item >> 2) == DIVIDE_A', and 'item / (item >> 3) == DEVIDE_B'
//however, this does not seem to work.
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
   let part1 = part1();
   let part2 = part2();
   println!("part 1: {}", part1);
   println!("part 2: {}", part2);
}