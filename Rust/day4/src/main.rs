extern crate permutohedron;
const PUZZLE: &'static str = include_str!("Input.txt");

use permutohedron::heap_recursive;

use std::collections::HashMap;
use std::collections::HashSet;

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

    let n = PUZZLE.lines().filter(|line| anagram(line)).count();
    println!("{}", n);
}

fn permutations(mut vec: &mut Vec<char>, line: &str) -> Vec<String> {
    let mut v = Vec::new();
    heap_recursive(&mut vec, |permutation| {
        v.push(permutation.iter().collect::<String>());
    });
    v
}

//we have line, we get the words out it. We also store the words in a vector.
//we get the permutations of each word,
//if the permutations of a word are more than 1 times in the vector, its NOT a valid line.
fn anagram(line: &str) -> bool {
    let v = line.split_whitespace().map(|word| word.to_string()).collect::<Vec<_>>();
    for word in line.split_whitespace() {
        let mut tmp = word.chars().collect::<Vec<_>>();
        let permutations = permutations(&mut tmp, line);
            
        //get the occurence of a permutation in the vector.
        for item in line.split_whitespace() {
            let i = permutations.iter().filter(|perm| perm == &item).count() as i64;
            if i > 0 {
                println!("{} is {} times in the vector, but mutated\n{:?}", item, i, v);
                return false
            }
        }
        //if occurence == 2 {println!("{:?}, {}, {:?}", v, word, permutations.iter().find(|elem| elem.iter().collect::<String>() == word));return false;}
    }
    true
}