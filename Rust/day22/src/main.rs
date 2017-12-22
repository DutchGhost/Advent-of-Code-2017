const PUZZLE: &'static str = include_str!("Input.txt");

mod walker;
mod walker2;
mod tmp;
use walker::*;
use walker2::*;

fn main() {
    let nodes1 = PUZZLE.lines().map(|line| line.chars().map(|c| Node::from(c)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let nodes2 = PUZZLE.lines().map(|line| line.chars().map(|c| Node2::from(c)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let walker = Walker::new(nodes1);
    let walker2 = Walker2::new(nodes2);

    println!("{}", walker.take(10_000).sum::<i64>());
    println!("{}", walker2.take(10_000_000).sum::<i64>());

}
