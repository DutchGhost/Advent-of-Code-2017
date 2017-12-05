const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().expect("this should not happend"))
        .collect::<Vec<_>>()
}


fn part1(mut jumps: Vec<i64>) -> i64 {
    let mut n = 0;
    let mut prev: i64 = 0;
    let mut pc: i64 = 0;
    let mut lenght = jumps.len();

    while pc < lenght as i64 {
        pc += jumps[pc as usize];
        jumps[prev as usize] += 1;
        prev = pc;
        n += 1;
    }

    n
}

fn part2(mut jumps: Vec<i64>) -> i64 {
    let mut n = 0;
    let mut prev = 0;
    let mut pc: i64 = 0;
    let mut lenght = jumps.len();

    while pc < lenght as i64 {
        pc += jumps[pc as usize];

        if jumps[prev as usize] >= 3 {
            jumps[prev as usize] -= 1;
        }
        else {
            jumps[prev as usize] +=1;
        }
        prev = pc;
        n += 1;
    }
    n
}
fn main() {
    let mut jumps = parse(PUZZLE);
    println!("{}", part1(jumps.clone()));
    println!("{}", part2(jumps.clone()));
}