const PUZZLE: &'static str = include_str!("Input.txt");
mod node;
use node::Node;
use node::Coordinates;

fn parse(input: &str) -> Vec<Vec<Node>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| Node::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn main() {
    let nodes = parse(PUZZLE);
    let mut n = 0;
    let mut coordinates = Coordinates::new(&nodes);
    while nodes[coordinates.y as usize][coordinates.x as usize] != Node::Void {
        coordinates.walk();
        coordinates.checknode(&nodes);
        n += 1;
    }
    println!("part 1: {}", coordinates.message);
    println!("part 2: {}", n);
}
