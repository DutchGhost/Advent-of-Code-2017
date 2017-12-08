#![feature(slice_patterns)]
use std::collections::HashMap;

mod statement;
use statement::Statement;

const PUZZLE: &'static str = include_str!("Input.txt");


fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect()
}

fn main() {
    let parsed = parse(PUZZLE);
    let mut map = HashMap::new();

    let mut part2 = 0;

    for line in parsed {
        let statement = Statement::new(line, &map);
        statement.eval(&mut map);

        if let Some(val) = map.get(statement.name()) {
            part2 = std::cmp::max(part2, *val);
        }
    }

    let part1 = map.iter().max_by_key(|&(_, n)| n).map(|(_, n)| n).unwrap();
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
