const PUZZLE: &'static str = include_str!("Input.txt");

mod component;
use component::*;

use std::collections::{HashMap, HashSet};

use std::str::FromStr;

fn parse(input: &str) -> Vec<Port> {
    input.lines().map(|line| Port::from_str(line).unwrap()).collect()
}


fn main() {
    let mut components = parse(PUZZLE);
    
   }
