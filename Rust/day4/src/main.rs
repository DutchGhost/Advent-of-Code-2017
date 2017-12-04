const PUZZLE: &'static str = include_str!("Input.txt");

use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let mut valids = 0;
    let mut map = HashMap::new();
    
    /// Iterate over the lines
    for line in input.lines() {
        let mut count = 0;
        
        // Iterate over each word, and put it in a hashmap
        for word in line.split_whitespace() {
            *map.entry(word).or_insert(0) += 1;
            count += 1;
        }

        // If the count of the keys equals to the count of the words in the line, it's a valid line.
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
    
    // Iterate over the lines
    for line in input.lines() {
        let mut count = 0;
        
        // Iterate over each word, sort the word, and put in in a hashmap
        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();
            *map.entry(chars).or_insert(1) += 1;
            count += 1;
        }
        
        // If the count of the keys eqals the count of the (sorted) words in the line, it's a valid line.
        if map.keys().count() == count {
            valids += 1;
        }
        map.drain();
    }
    valids
}

fn combined(input: &str) -> (i32, i32) {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    let (mut valids1, mut valids2) = (0, 0);


    for line in input.lines() {
        let mut count = 0;
        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();

            *map1.entry(word).or_insert(0) += 1;
            *map2.entry(chars).or_insert(0) += 1;
            count += 1;
        }

        if map1.keys().count() == count {
            valids1 += 1;
        }

        if map2.keys().count() == count {
            valids2 += 1
        }
        map1.drain();
        map2.drain();
    }

    (valids1, valids2)
}
fn main() {
    println!("{}", part1(PUZZLE));
    println!("{}", part2(PUZZLE));

    let (part1, part2) = combined(PUZZLE);
    println!("{}", part1);
    println!("{}", part2);
}
