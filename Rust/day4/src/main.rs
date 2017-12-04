const PUZZLE: &'static str = include_str!("Input.txt");

use std::collections::HashSet;

/// Iterate over the lines,
/// in each lines iterate over the words.
/// Put each word into the HashSet,
/// if the number of elements in the HashSet equals the lenght of the line,
/// its a valid line.
fn part1(input: &str) -> i32 {
    let mut valids = 0;
    let mut set = HashSet::with_capacity(11);
    
    for line in input.lines() {
        let mut count = 0;
        
        for word in line.split_whitespace() {
            set.insert(word);
            count += 1;
        }

        if set.iter().count() == count {
            valids += 1;
        }
        set.drain();
    }
    valids
}

/// Iterate over the lines,
/// in each line iterate over the words,
/// and store the characters of the words in a vector.
/// Then sort the vector, and put in the HashSet.
/// If the number of elements in the HashSet equals the lenght of the line,
/// it's a valid line
fn part2(input: &str) -> i64 {
    let mut valids = 0;
    let mut set = HashSet::with_capacity(11);
    
    for line in input.lines() {
        let mut count = 0;
        
        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();
            set.insert(chars);
            count += 1;
        }

        if set.iter().count() == count {
            valids += 1;
        }
        set.drain();
    }
    valids
}

fn combined(input: &str) -> (i32, i32) {
    let mut set1 = HashSet::with_capacity(11);
    let mut set2 = HashSet::with_capacity(11);

    let (mut valids1, mut valids2) = (0, 0);

    for line in input.lines() {
        let mut count = 0;
        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();

            set1.insert(word);
            set2.insert(chars);
            count += 1;
        }

        if set1.iter().count() == count {
            valids1 += 1;
        }

        if set2.iter().count() == count {
            valids2 += 1
        }
        set1.drain();
        set2.drain();
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