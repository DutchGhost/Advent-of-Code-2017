#![feature(slice_rotate)]
const PUZZLE: &'static str = include_str!("Input.txt");
const PROGRAMMS: [&str; 16] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"];

fn run(programms: &mut [&str], ins: &str) {
    if ins.starts_with("s") {
        let n = programms.len() - ins[1..].parse::<usize>().unwrap();
        programms.rotate(n);
    }
    else if ins.starts_with("x") {
        let toswap = ins[1..].split("/").map(|pos| pos.parse::<usize>().unwrap()).collect::<Vec<_>>();
        programms.swap(toswap[0], toswap[1]);
    }
    else {
        let toswap = ins[1..].split("/").collect::<Vec<_>>();

        let pos1 = programms.iter().position(|programm| programm == &toswap[0]).unwrap();
        let pos2 = programms.iter().position(|programm| programm == &toswap[1]).unwrap();

        programms.swap(pos1, pos2);
    }
}

//make this better!
fn get_cycle() -> usize {
    let mut programms = PROGRAMMS;
    let mut cache = Vec::new();
    for i in 0.. {
        for ins in PUZZLE.split(",") {
            run(&mut programms, ins);
        }
        if cache.contains(&programms) {
            return i;
        }
        else {
            cache.push(programms.clone());
        }
    }
    unreachable!()
}

fn stringify(programm: [&str; 16]) -> String {
    programm.into_iter().map(|s| s.to_string()).collect::<String>()
}
fn main() {
    let mut programms = PROGRAMMS;
    for ins in PUZZLE.split(",") {
        run(&mut programms, ins);
    }
    println!("part 1: {}", stringify(programms));
    
    let mut programms = PROGRAMMS;
    for _ in 0..(1000000000 % get_cycle()) {
        for ins in PUZZLE.split(",") {
            run(&mut programms, ins);
        }
    }
    println!("part 2: {}", stringify(programms));
}
