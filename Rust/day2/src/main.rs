const PUZZLE: &'static str = include_str!("Input.txt");

fn to_num(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

/// gets the biggest out of the nums.
/// returns None if the bigger one can not be equally devided by the smaller one.
fn remainder(a: &u32, b: &u32) -> Option<u32> {
    if a > b {
        if a % b == 0 {
            return Some(a / b);
        }
    } else {
        if b % a == 0 {
            return Some(b / a);
        }
    }
    None
}
fn evenly(vec: &[u32]) -> Option<Option<u32>> {
    // The inner Iterator can result in a None, if nothing so far is divisible.
    vec.iter()
        .enumerate()
        .map(|(idx, num1)| {
            vec[..idx]
                .iter()
                .filter_map(|num2| remainder(num1, num2))
                .next()
        })
        .filter(|item| item.is_some())
        .next()
}

fn difference(nums: &[u32]) -> u32 {
    nums.iter().max().unwrap() - nums.iter().min().unwrap()
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
