const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().expect("this should not happen!"))
        .collect::<Vec<_>>()
}


fn run<F>(mut jumps: Vec<i64>, updater: F) -> i64
where
    F: Fn(i64) -> i64
{
    let mut n = 0;
    let mut pc: i64 = 0;

    while let Some(idx) = jumps.get_mut(pc as usize) {
        pc += *idx;
        *idx += updater(*idx);
        n += 1;
    }
    n
}

fn main() {
    let data = parse(PUZZLE);
    println!("day 5.1: {}", run(data.clone(), |_| 1));
    println!("day 5.2: {}", run(data, |item| if item >= 3 { -1 } else { 1 }));
}
