use std::collections::HashMap;

const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|bank| bank.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

///start at idx + 1. Loop 'till 15, then chain with something endless.
/// then take as many items as needed (the number of items we need is value!)
/// and add one to each element of the slice.
#[inline]
fn redistribute(memory: &mut [i32], idx: usize, value: i32) {
    let len = memory.len();
    memory[idx] = 0;

    (idx + 1..len)
        .chain(0..)
        .take(value as usize)
        .for_each(|n| memory[n] += 1);
}

/// max_by_key returns the last max value... using .rev() escapes from this,
/// since the 'last' max is actually the first max in a tie this way!
/// (in a list with [1, [3], 3, 2] it will return the item marked in parentheses
/// because for the iterator it's the last max value) 
fn cycle(memory: &[i32]) -> (usize, i32) {
    memory
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, bank)| bank)
        .map(|(idx, bank)| (idx, *bank))
        .unwrap()
}

fn solve(input: &str) -> (i32, i32) {
    let mut memory = parse(input);
    let mut cache: HashMap<Vec<i32>, i32> = HashMap::new();

    let mut n = 0;
    while !cache.contains_key(&memory) {
        cache.insert(memory.clone(), n);
        
        let (idx, value) = cycle(&memory);
        redistribute(&mut memory, idx, value);
        
        n += 1;
    }
    (n, n - cache.get(&memory).unwrap())
}

fn main() {
    let (part1, part2) = solve(PUZZLE);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}