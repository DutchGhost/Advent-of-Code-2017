use std::collections::hash_map::*;

const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|bank| bank.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn redistribute(memory: &mut [i32], mut idx: usize, mut value: i32) {
    memory[idx] = 0;

    idx += 1;
    let lenght = memory.len();

    while value > 0 {

        idx = idx % lenght;
        memory[idx] += 1;
        
        value -= 1;
        idx += 1;
    }
    
}

fn solve(input: &str) -> (i32, i32) {
    let mut memory = parse(input);
    let mut cache: HashMap<Vec<i32>, i32> = HashMap::new();

    let mut n = 0;
    loop {
        let (idx, value) = memory
            .iter()
            .enumerate()
            .map(|(idx, bank)| (idx, *bank))
            .max_by_key(|&(idx, bank)| (bank, -(idx as i64))).unwrap();

        redistribute(&mut memory, idx, value);
        n += 1;

        match cache.entry(memory.clone()) {
            Entry::Occupied(map) => {
                return (n, n - map.get());
            },
            Entry::Vacant(memory) => memory.insert(n),
        };
    }
}

fn main() {
    let (part1, part2) = solve(PUZZLE);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}