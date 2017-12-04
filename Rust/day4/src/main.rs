extern crate permutohedron;
use permutohedron::heap_recursive;

use std::collections::HashMap;
use std::str;
const PUZZLE: &'static str = include_str!("Input.txt");

fn main() {
    let mut n = 0;
    for line in PUZZLE.lines() {
        let mut map = HashMap::new();
        for word in line.split_whitespace() {
    
            *map.entry(word).or_insert(0) += 1;
        }
        if map.values().all(|value| value == &1) {
            n += 1;
        }
    }
    println!("{}", n);
}
