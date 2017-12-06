const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|bank| bank.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn redistribute(memory: &mut [i32], idx: i64, mut value: i32) {
    let mut index = idx + 1;
    
    let lenght = memory.len();
    memory[idx as usize] = 0;

    while value > 0 {

        index = index % lenght as i64;
        memory[index as usize] += 1;
        
        value -= 1;
        index += 1;
    }
    
}
fn main() {
    let mut memory = parse(PUZZLE);
    let mut cache: Vec<Vec<i32>> = Vec::new();

    let mut n = 0;

    loop {
        //println!("{:?}", memory);
        let (idx, value) = memory
            .iter()
            .enumerate()
            .map(move |(idx, bank)| (idx.clone() as i64, bank.clone()))
            .max_by_key(|&(idx, bank)| (bank, -idx)).unwrap();

        redistribute(&mut memory, idx, value);

        if cache.iter().filter(|cached| *cached == &memory).count() < 1 {
            cache.push(memory.clone());
            n += 1
        }
        else {
            n += 1;
            break;
        }
    }
    println!("{}", n);
}

/*
    PART 2:
            if cache.iter().filter(|&&(_, ref cached)| cached == &memory).count() < 1 {
            cache.push((n, memory.clone()));
            n += 1
        }
        else {
            println!("{:?}", cache.iter().filter(|&&(_, ref cached)| cached == &memory).map(|&(num, _)| n - num).next());
            break;
        }
*/
