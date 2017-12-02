const PUZZLE: &'static str = include_str!("Input.txt");

fn to_num(s: &str) -> Vec<u32> {
    s.split('\t')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    println!(
        "{}",
        PUZZLE
            .lines()
            .map(|line| to_num(line))
            .map(|nums| {
                nums.iter().max().unwrap() - nums.iter().min().unwrap()
            })
            .sum::<u32>()
    );
}
