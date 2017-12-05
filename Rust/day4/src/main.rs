const PUZZLE: &'static str = include_str!("Input.txt");

use std::collections::HashSet;

/// Iterate over the lines,
/// in each lines iterate over the words.
/// Collect the characters in a vector,
/// apply transform on this vector. this eiter sorts it, or does nothing.
/// then put the vector in the hashset.
/// if the number of elements in the HashSet equals the lenght of the line,
/// its a valid line.
fn solve<F>(input: &str, mut transformer: F) -> i64
where
    F: FnMut(&mut [char])
{
    let mut valids = 0;
    let mut set = HashSet::with_capacity(11);
    
    for line in input.lines() {
        let mut count = 0;
        
        for mut word in line.split_whitespace() {

            let mut chars = word.chars().collect::<Vec<_>>();
            
            transformer(&mut chars);
            set.insert(chars);
            
            count += 1;
        }

        if set.drain().count() == count {
            valids += 1;
        }
    }

    valids
}

fn noop<T: ?Sized>(_: &mut T) {}
fn main() {
    println!("part 1: {}", solve(PUZZLE, noop));
    println!("part 2: {}", solve(PUZZLE, |word| word.sort()))
}