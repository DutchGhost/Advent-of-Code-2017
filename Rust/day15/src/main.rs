const MASK: i64 = 0b1111_1111_1111_1111;
const DIVIDER: i64 = 2147483647;

const VALUE_A: i64 = 16807;
const VALUE_B: i64 = 48271;

const INPUT_A: i64 = 512;
const INPUT_B: i64 = 191;

const DIVIDE_A: i64 = 4;
const DIVIDE_B: i64 = 8;

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

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.value *= self.multiplier;
        self.value %= DIVIDER;
        Some(self.value)
    }
}

#[inline]
fn mask<'r>(&(n1, n2): &'r (i64, i64)) -> bool {
    n1 & MASK == n2 & MASK
}

#[inline]
fn criteria(item: i64, to_devide: i64) -> bool {
    item % to_devide == 0
}

fn part1() -> usize {
    let generator_a = Generator::new(INPUT_A, VALUE_A);
    let generator_b = Generator::new(INPUT_B, VALUE_B);

    generator_a.zip(generator_b).take(40_000_000).filter(mask).count()
}

fn part2() -> usize {
    let generator_a = Generator::new(INPUT_A, VALUE_A);
    let generator_b = Generator::new(INPUT_B, VALUE_B);

    generator_a
        .filter(|&n| criteria(n, DIVIDE_A))
        .zip(generator_b.filter(|&n| criteria(n, DIVIDE_B)))
        .take(5_000_000)
        .filter(mask)
        .count()
}
fn main() {
   let part1 = part1();
   let part2 = part2();

   println!("part 1: {}", part1);
   println!("part 2: {}", part2);
}