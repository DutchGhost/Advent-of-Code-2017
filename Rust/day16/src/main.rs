#![feature(slice_rotate)]

const PUZZLE: &'static str = include_str!("Input.txt");
const PROGRAMMS: [u8; 16] = [
    b'a',
    b'b',
    b'c',
    b'd',
    b'e',
    b'f',
    b'g',
    b'h',
    b'i',
    b'j',
    b'k',
    b'l',
    b'm',
    b'n',
    b'o',
    b'p',
];

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl<'a> From<&'a str> for Move {
    fn from(s: &str) -> Move {
        if s.starts_with("s") {
            Move::Spin(s[1..].parse::<usize>().unwrap())
        } else if s.starts_with("x") {
            let mut toswap = s[1..].split("/").map(|pos| pos.parse().unwrap());
            Move::Exchange(toswap.next().unwrap(), toswap.next().unwrap())
        } else {
            let mut partners = s[1..].bytes().filter(|c| *c != b'/');
            Move::Partner(partners.next().unwrap(), partners.next().unwrap())
        }
    }
}

struct Instructions(Vec<Move>);

impl Instructions {
    fn new<'a>(s: &'a str) -> Instructions {
        Instructions(s.split(",").map(|line| Move::from(line)).collect())
    }
    fn iter<'s>(&'s self) -> std::slice::Iter<'s, Move> {
        self.0.iter()
    }
}

fn run(programms: &mut [u8], instructions: &Instructions) {
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
fn get_cycle<'a>(programms: &mut [u8], instructions: &Instructions) -> usize {

    let mut n = 0;
    while programms != PROGRAMMS || n == 0 {
        run(programms, instructions);
        n += 1;
    }
    n
}

fn stringify(programm: [u8; 16]) -> String {
    programm.iter().map(|b| *b as char).collect::<String>()
}
fn main() {
    let instructions = Instructions::new(PUZZLE);
    {
        let mut programms = PROGRAMMS;
        run(&mut programms, &instructions);
        println!("part 1: {}", stringify(programms));
    }
    {
        let mut programms = PROGRAMMS;
        let cycle = get_cycle(&mut programms, &instructions);
        for _ in 0..(1_000_000_000 % cycle) {
            run(&mut programms, &instructions);
        }
        println!("part 2: {}", stringify(programms));
    }
}
