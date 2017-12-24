const PUZZLE: &'static str = include_str!("Input.txt");

fn parse<S: AsRef<str>>(input: S) -> Vec<i64> {
    input
        .as_ref()
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

    while let Some(offset) = jumps.get_mut(pc as usize) {
        pc += *offset;
        *offset += updater(*offset);
        n += 1;
    }
    n
}

#[inline]
fn one(_: i64) -> i64 { 1 }

#[inline]
fn two(n: i64) -> i64 {if n >= 3 { - 1 } else { 1 } }

fn main() {
    let data = parse(PUZZLE);
    println!("day 5.1: {}", run(data.clone(), one));
    println!("day 5.2: {}", run(data, two));
}
