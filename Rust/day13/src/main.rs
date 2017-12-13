const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(": ").map(|item| {
                item.parse::<i64>().expect("Failed to parse")
            });
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn solve(firewall: &[(i64, i64)]) -> i64 {
    firewall
        .iter()
        .filter(|&&(depth, range)| depth % ((range - 1) * 2) == 0)
        .map(|&(depth, range)| depth * range)
        .sum()
}

fn solve2(firewall: &[(i64, i64)]) -> i64 {
    (0..)
        .find(|wait| {
            firewall.iter().all(|&(depth, range)| {
                (depth + wait) % ((range - 1) * 2) != 0
            })
        })
        .unwrap()
}

fn main() {
    let mut parsed = parse(PUZZLE);
    println!("{}", solve(&parsed));
    println!("{}", solve2(&parsed));
}
