const PUZZLE: &'static str = include_str!("Input.txt");

fn to_num(s: &str) -> Vec<u32> {
    s.split('\t')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn evenly(vec: Vec<u32>) -> u32 {
    for first in vec.iter() {
        for second in vec.iter().filter(|item| item != &first) {
            if first % second == 0 {
                return first / second;
            }
        }
    }
    unreachable!()
}
fn main() {
    println!(
        "day 1.1: {}",
        PUZZLE
            .lines()
            .map(|line| to_num(line))
            .map(|nums| {
                nums.iter().max().unwrap() - nums.iter().min().unwrap()
            })
            .sum::<u32>()
    );
    println!("day 1.2: {}", PUZZLE.lines().map(|line| to_num(line)).map(|line| evenly(line)).sum::<u32>());
}
