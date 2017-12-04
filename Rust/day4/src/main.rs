const PUZZLE: &'static str = include_str!("Input.txt");

use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let mut valids = 0;
    let mut map = HashMap::new();
    
    for line in input.lines() {
        let mut count = 0;
        
        for word in line.split_whitespace() {
            *map.entry(word).or_insert(0) += 1;
            count += 1;
        }
        
        if map.keys().count() == count {
            valids += 1;
        }
        map.drain();
    }
    valids
}

fn part2(input: &str) -> i64 {
    let mut valids = 0;
    let mut map = HashMap::new();
    
    for line in input.lines() {
        let mut count = 0;
        
        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();
            *map.entry(chars).or_insert(1) += 1;
            count += 1;
        }
        
        if map.keys().count() == count {
            valids += 1;
        }
        map.drain();
    }
    valids
}
fn main() {
    println!("{}", part1(PUZZLE));
    println!("{}", part2(PUZZLE));
}
