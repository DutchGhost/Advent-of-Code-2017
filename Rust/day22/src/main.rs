const PUZZLE: &'static str = include_str!("Input.txt");

mod walker;
use walker::*;

fn main() {
    let nodes = PUZZLE.lines().map(|line| line.chars().map(|c| Node::from(c)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut walker = Walker::new(nodes);
    
    println!("{}", walker.take(10_000).sum::<i64>());

}
