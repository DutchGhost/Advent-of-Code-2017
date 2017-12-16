#![feature(slice_patterns)]

mod statement;
use statement::*;

const PUZZLE: &'static str = include_str!("Input.txt");


fn parse<'a>(input: &'a str) -> Vec<Vec<&'a str>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect()
}

fn main() {
    let parsed = parse(PUZZLE);
    let mut map = Registers::new();

    // for line in parsed {
    //     let statement = Statement::new(line, &map);
    //     statement.eval(&mut map);
    // }

    for line in PUZZLE.lines() {
        let v = line.split_whitespace().collect::<Vec<_>>();
        let statement: Result<Statement<fn(i32) -> i32>, StatementError> = Statement::<fn(i32) -> i32>::new(v, &map);
        match statement {
            Ok(s) => s.eval(&mut map),
            Err(e) => println!("{}", e.discription()),
        }
    }

    let (part1, part2) = map.max();
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
