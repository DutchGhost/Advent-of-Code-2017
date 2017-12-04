const PUZZLE: &'static str = include_str!("Input.txt");

use std::collections::HashSet;

/// Iterate over the lines,
/// in each lines iterate over the words.
/// Put each word into the HashSet,
/// if the number of elements in the HashSet equals the lenght of the line,
/// its a valid line.
fn part1(input: &str) -> i32 {
    let mut valids = 0;
    let mut map = HashSet::new();
    
    for line in input.lines() {
        let mut count = 0;
        
        for word in line.split_whitespace() {
            map.insert(word);
            count += 1;
        }

        if map.iter().count() == count {
            valids += 1;
        }
        map.drain();
    }
    valids
}

/// Iterate over the lines,
/// in each line iterate over the words,
/// and store the characters of the words in a vector.
/// Then sort the vector, and put in the HashSet.
/// If the number of elements in the HashSet equals the lenght of the line,
/// it's a vlid line
fn part2(input: &str) -> i64 {
    let mut valids = 0;
    let mut map = HashSet::new();
    
    for line in input.lines() {
        let mut count = 0;
        
        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();
            map.insert(chars);
            count += 1;
        }
        
        if map.iter().count() == count {
            valids += 1;
        }
        map.drain();
    }
    valids
}

fn combined(input: &str) -> (i32, i32) {
    let mut map1 = HashSet::new();
    let mut map2 = HashSet::new();

    let (mut valids1, mut valids2) = (0, 0);

    for line in input.lines() {
        let mut count = 0;
        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();

            map1.insert(word);
            map2.insert(chars);
            count += 1;
        }

        if map1.iter().count() == count {
            valids1 += 1;
        }

        if map2.iter().count() == count {
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