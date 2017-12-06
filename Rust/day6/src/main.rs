const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|bank| bank.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn redistribute(memory: &mut [i32], mut idx: i64, mut value: i32) {
    
    memory[idx as usize] = 0;
    
    idx += 1;
    let lenght = memory.len();

    while value > 0 {

        idx = idx % lenght as i64;
        memory[idx as usize] += 1;
        
        value -= 1;
        idx += 1;
    }
    
}
fn main() {
    let mut memory = parse(PUZZLE);
    let mut cache: Vec<(i64, Vec<i32>)> = Vec::new();

    let mut part1 = 0;

    loop {
        let (idx, value) = memory
            .iter()
            .enumerate()
            .map(move |(idx, bank)| (idx.clone() as i64, bank.clone()))
            .max_by_key(|&(idx, bank)| (bank, -idx)).unwrap();

        redistribute(&mut memory, idx, value);
        part1 += 1;

        if cache.iter().filter(|&&(_, ref cached)| *cached == memory).count() == 1 {
            println!("part 2: {}", cache.iter().filter(|&&(_, ref cached)| *cached == memory).map(|&(n, _)| part1 - n).next().unwrap());
            break;
        }

        cache.push((part1, memory.clone()));
    }
    println!("part 1: {}", part1);
}