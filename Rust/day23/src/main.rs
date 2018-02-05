const PUZZLE: &'static str = include_str!("Input.txt");

use std::collections::HashMap;

fn parse(input: &str) -> [[&str; 3]; 32] {
    let mut arr: [[&str; 3]; 32] = [[""; 3]; 32];

    for (row, line) in arr.iter_mut().zip(input.lines()) {
        for (thing, word) in row.iter_mut().zip(line.split(" ")) {
            *thing = word;
        }
    }

    arr
}

//returns either a value that needs to copied, or the value of a register
fn read<'b, 'a: 'b>(s: &'a str, map: &mut [i64; 8]) -> i64 {
    match s.parse::<i64>() {
        Ok(n) => n,
        Err(_) => (*map).to_idx(s).clone(),
    }
}

trait ToIdx {
    fn to_idx(&mut self, s: &str) -> &mut i64;
}

impl ToIdx for [i64; 8] {
    fn to_idx(&mut self, s: &str) -> &mut i64 {
        match s {
            "a" => &mut self[0],
            "b" => &mut self[1],
            "c" => &mut self[2],
            "d" => &mut self[3],
            "e" => &mut self[4],
            "f" => &mut self[5],
            "g" => &mut self[6],
            "h" => &mut self[7],
            _ => panic!()
        }
    }
}
fn main() {
    let mut registers: [i64; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    let instructions: [[&str; 3]; 32] = parse(PUZZLE);
    let mut ip: i64 = 0;
    let mut muls = 0;
    let mut millions = 0;
    while ip < instructions.len() as i64 {
        let ins = &instructions[ip as usize];
        match ins[0].as_ref() {
            "set" => *registers.to_idx(ins[1]) = read(&ins[2], &mut registers),
            "sub" => *registers.to_idx(ins[1]) -= read(&ins[2], &mut registers),
            "mul" => {
                *registers.to_idx(ins[1]) *= read(&ins[2], &mut registers);
                muls += 1;
            },
            "jnz" => if read(&ins[1], &mut registers) != 0 {ip += read(&ins[2], &mut registers)-1;},
            _ => panic!(),
        }
        ip += 1;
    }
    println!("{}", muls);
    println!("{}", main_2());
}


fn main_2() -> usize {
    // let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (1, 0, 0, 0, 0, 0 , 0, 0);
    //
    // b = 105700;
    // c = 122700;   // 8
    //
    // '_1: loop {
    //     f = 1;         // 9
    //     d = 2;         // 10
    //
    //
    //     '_2: loop {
    //         e = 2;         // 11
    //
    //         '_3: loop {
    //             if (d * e) - b == 0 {    // 15
    //                 f = 0;
    //             }
    //
    //             e += 1;      // 17
    //             if e - b == 0 { break '_3; }
    //         }
    //
    //         d += 1;       // 21
    //         if d - b == 0 { break '_2; }
    //     }
    //
    //     if f == 0 {  //25
    //         h += 1;
    //     }   //26
    //
    //     if b - c == 0 { break '_1; }
    //     b += 17;
    // }
    StepBy::new(105700, 122700, 17).filter(|n| (2..*n).any(|d| n % d == 0)).count()
}

struct StepBy {
    start: usize,
    end: usize,
    steps: usize,
}

impl StepBy {
    fn new(start: usize, end: usize, steps: usize) -> StepBy {
        StepBy {start, end, steps}
    }
}
impl Iterator for StepBy {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.start;
        if self.start <= self.end {
            self.start += self.steps;
            return Some(ret)
        }
        None
    }
}
