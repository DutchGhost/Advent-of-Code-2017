const PUZZLE: &'static str = include_str!("Input.txt");

fn to_num(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn sort<'a>(a: &'a u32, b: &'a u32) -> (&'a u32, &'a u32) {
    if a > b { (a, b) } else { (b, a) }
}

/// gets the biggest out of the nums.
/// returns None if the bigger one can not be equally devided by the smaller one.
fn is_divisible(a: &u32, b: &u32) -> Option<u32> {
    let (num1, num2) = sort(a, b);
    if num1 % num2 == 0 {
        Some(num1 / num2)
    }
    else {
        None
    }
}
fn evenly(vec: &[u32]) -> Option<Option<u32>> {
    // The inner Iterator can result in a None, if nothing so far is divisible.
    vec.iter()
        .enumerate()
        .map(|(idx, num1)| {
            vec[..idx]
                .iter()
                .filter_map(|num2| is_divisible(num1, num2))
                .next()
        })
        .filter(|item| item.is_some())
        .next()
}

fn difference(nums: &[u32]) -> u32 {
    nums.iter().fold((0u32, std::u32::MAX), |(max, min), item,| {
        (std::cmp::max(max, *item), std::cmp::min(min, *item))
    }).sub()
}

fn solve() -> (u32, u32) {
    let parsed = PUZZLE.lines().map(|line| to_num(line)).collect::<Vec<_>>();
    let part1 = parsed.iter().map(|nums| difference(nums)).sum::<u32>();
    let part2 = parsed.iter().filter_map(|nums| evenly(nums).unwrap()).sum::<u32>();

    (part1, part2)
}
fn main() {
    let (part1, part2) = solve();

    println!("day 2.1: {}", part1);
    println!("day 2.2: {}", part2);
}

trait Sub<T>
where
    T: std::ops::Sub<T>
{
    fn sub(self) -> <T as std::ops::Sub>::Output;
}

impl <T>Sub<T> for (T, T)
where
    T: std::ops::Sub<T>
{
     fn sub(self) -> <T as std::ops::Sub>::Output {
        self.0 - self.1
    }
}
