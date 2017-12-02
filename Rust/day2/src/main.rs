const PUZZLE: &'static str = include_str!("Input.txt");

fn to_num(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn evenly(vec: &[u32]) -> u32 {
    for first in vec.iter() {
        for second in vec.iter().rev().filter(|item| item != &first) {
            if first % second == 0 {
                return first / second;
            }
        }
    }
    unreachable!()
}

fn difference(nums: &[u32]) -> u32 {
    nums.iter().max().unwrap() - nums.iter().min().unwrap()
}

fn solve() -> (u32, u32) {
    let parsed = PUZZLE.lines().map(|line| to_num(line)).collect::<Vec<_>>();
    
    let part1 = parsed
                .iter()
                .map(|nums| difference(nums))
                .sum::<u32>();
    
    let part2 = parsed
                .iter()
                .map(|nums| evenly(nums))
                .sum::<u32>();

    (part1, part2)
}
fn main() {
    let (part1, part2) = solve();

    println!("day 2.1: {}", part1);
    println!("day 2.2: {}", part2);
}
