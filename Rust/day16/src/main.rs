#![feature(slice_rotate)]

const PUZZLE: &'static str = include_str!("Input.txt");
const PROGRAMMS: [&str; 16] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"];

enum Move<'a> {
    Spin(usize),
    Exchange(usize, usize),
    Partner(&'a str, &'a str),
}

impl<'a> Move<'a> {
    fn new(s: &str) -> Move {
        if s.starts_with("s") {
            Move::Spin(s[1..].parse().unwrap())
        }
        else if s.starts_with("x") {
            let mut toswap = s[1..].split("/").map(|pos| pos.parse::<usize>().unwrap());
            Move::Exchange(toswap.next().unwrap(), toswap.next().unwrap())
        }
        else {
            let mut partners = s[1..].split("/");
            Move::Partner(partners.next().unwrap(), partners.next().unwrap())
        }
    }
}

struct Instructions<'a> {
    instructions: Vec<Move<'a>>,
}

impl <'a>Instructions<'a> {
    fn new(s: &'a str) -> Instructions<'a> {
        Instructions {
            instructions: s.split(",").map(|item| Move::new(item)).collect::<Vec<_>>(),
        }
    }
    fn iter<'s>(&'s self) -> std::slice::Iter<'s, Move<'s>> {
        self.instructions.iter()
    }
}

fn run(programms: &mut [&str], instructions: &Instructions) {
    let len = programms.len();

    for instruction in instructions.iter() {
        match instruction {
            &Move::Spin(n) => programms.rotate(len - n),
            &Move::Exchange(n1, n2) => programms.swap(n1, n2),
            &Move::Partner(d1, d2) => {
                let n1 = programms.iter().position(|item| item == &d1).unwrap();
                let n2 = programms.iter().position(|item| item == &d2).unwrap();
                programms.swap(n1, n2);
            }
        }
    }
}

//runs the dance untill the initial state. (at the start it's the initial state, but n equals 0.)
//returns after how many dances it repeats itself, and the programms.
fn get_cycle<'a>(programms: &mut [&str], instructions: &Instructions) -> usize {
    
    let mut n = 0;
    while programms != PROGRAMMS || n == 0 {
        run(programms, instructions);
        n += 1;
    }
    n
}

fn stringify(programm: [&str; 16]) -> String {
    programm.into_iter().map(move |s| s.to_string()).collect::<String>()
}
fn main() {
    let instructions = Instructions::new(PUZZLE);

    let mut programms = PROGRAMMS;
    run(&mut programms, &instructions);
    println!("part 1: {}", stringify(programms));
    
    let mut programms = PROGRAMMS;
    let cycle = get_cycle(&mut programms, &instructions);
    for _ in 0..(1_000_000_000 % cycle) {
        run(&mut programms, &instructions);
    }
    println!("part 2: {}", stringify(programms));
}
