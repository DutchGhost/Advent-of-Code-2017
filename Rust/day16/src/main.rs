#![feature(slice_rotate)]
const PUZZLE: &'static str = include_str!("Input.txt");

fn main() {
    let mut programms = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"];
    for ins in PUZZLE.split(",") {
        if ins.starts_with("s") {
            let mut n = programms.len() - ins[1..].parse::<usize>().unwrap();
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
    for letter in programms.iter() {
        print!("{}", letter);
    }
}
