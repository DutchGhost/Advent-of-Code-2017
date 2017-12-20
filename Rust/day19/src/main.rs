const PUZZLE: &'static str = include_str!("Input.txt");
mod node;
use node::{Node, Walker};

fn parse(input: &str) -> Vec<Vec<Node>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| Node::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn main() {
    let nodes = parse(PUZZLE);
    let mut n = 0;
    let mut walker = Walker::new(nodes);

    while !walker.atvoidnode() {
        walker.walk();
        n += 1;
    }
    println!("part 1: {}", walker.getstr());
    println!("part 2: {}", n);
}
