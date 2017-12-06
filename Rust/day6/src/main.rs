use std::collections::hash_map::*;

const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|bank| bank.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn redistribute(memory: &mut [i32], idx: usize, value: i32) {
    memory[idx] = 0;

    (0..memory.len())
        .cycle()
        .skip(idx + 1)
        .take(value as usize)
        .for_each(|idx| memory[idx] += 1);
}

fn cycle(memory: &[i32]) -> (usize, i32) {
    memory
        .iter()
        .enumerate()
        .map(|(idx, bank)| (idx, *bank))
        .max_by_key(|&(idx, bank)| (bank, -(idx as i64))).unwrap()
}

fn solve(input: &str) -> (i32, i32) {
    let mut memory = parse(input);
    let mut cache: HashMap<Vec<i32>, i32> = HashMap::new();

    let mut n = 0;
    loop {
        let (idx, value) = cycle(&memory);

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