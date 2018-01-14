extern crate libaoc;

const PUZZLE: &'static str = include_str!("Input.txt");

use libaoc::convert::Convert;

mod node;
use node::{Node, Walker};

const ROWS: usize = 201;
const COLUMS: usize = 202;

fn parse(input: &str) -> [[Node; COLUMS]; ROWS] {
    let mut arr: [[Node; COLUMS]; ROWS] = [[Node::Void; COLUMS]; ROWS];
 
    for (row, line) in arr.iter_mut().zip(input.lines()) {
        line.chars().convert_into_slice(row);
    }

    arr
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
