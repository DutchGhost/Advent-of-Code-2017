#![feature(slice_rotate)]
const PUZZLE: &'static str = include_str!("Input.txt");
const PROGRAMMS: [&str; 16] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"];

fn run(programms: &mut [&str], input: &str) {
    for ins in input.split(",") {
        if ins.starts_with("s") {
            let n = programms.len() - ins[1..].parse::<usize>().unwrap();
            programms.rotate(n);
        }
        else if ins.starts_with("x") {
            let mut toswap = ins[1..].split("/").map(|pos| pos.parse::<usize>().unwrap());
            programms.swap(toswap.next().unwrap(), toswap.next().unwrap());
        }
        else {
            let toswap = ins[1..].split("/").collect::<Vec<_>>();
            
            let pos1 = programms.iter().position(|programm| programm == &toswap[0]).unwrap();
            let pos2 = programms.iter().position(|programm| programm == &toswap[1]).unwrap();

            programms.swap(pos1, pos2);
        }
    }
}

//runs the dance untill the initial state. (at the start it's the initial state, but n equals 0.)
//returns after how many dances it repeats itself, and the programms.
fn get_cycle<'a>() -> (usize, [&'a str; 16]) {
    let mut programms = PROGRAMMS;
    
    let mut n = 0;
    while programms != PROGRAMMS || n == 0 {
        run(&mut programms, PUZZLE);
        n += 1;
    }
    (n, programms)
}

fn stringify(programm: [&str; 16]) -> String {
    programm.into_iter().map(|s| s.to_string()).collect::<String>()
}
fn main() {
    let mut programms = PROGRAMMS;
    run(&mut programms, PUZZLE);
    println!("part 1: {}", stringify(programms));
    
    let (cycle, mut programms) = get_cycle();
    for _ in 0..(1_000_000_000 % cycle) {
        run(&mut programms, PUZZLE);
    }
    println!("part 2: {}", stringify(programms));
    println!("{:?}", programms);
}
