const PUZZLE: &'static str = include_str!("Input.txt");
mod programm;
use programm::Programm;

use std::collections::HashMap;
use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Vec<&str>> {
    input.lines().map(|line| line.split(" ").collect()).collect()
}

//returns either a value that needs to copied, or the value of a register
fn read<'b, 'a: 'b>(s: &'a str, map: &mut HashMap<&'b str, i64>) -> i64 {
    match s.parse::<i64>() {
        Ok(n) => n,
        Err(_) => *map.entry(s).or_insert(0),
    }
}

fn main() {
    let mut registers = HashMap::new();
    let instructions = parse(PUZZLE);
    let mut ip: i64 = 0;
    let mut snd: (&str, Option<i64>) = ("init", None);
    let mut rcv = 0;


    'outer: loop {
        let ins = &instructions[ip as usize];
        match ins[0].as_ref() {
            "set" => *registers.entry(ins[1]).or_insert(0) = read(&ins[2], &mut registers),
            "sub" => *registers.entry(ins[1]).or_insert(0) -= read(&ins[2], &mut registers),
            "add" => *registers.entry(ins[1]).or_insert(0) += read(&ins[2], &mut registers),
            "mod" => *registers.entry(ins[1]).or_insert(0) %= read(ins[2], &mut registers),
            "mul" => *registers.entry(ins[1]).or_insert(0) *= read(&ins[2], &mut registers),
            "jgz" => if read(&ins[1], &mut registers) > 0 {ip += read(&ins[2], &mut registers)-1;},

            "snd" => snd = (ins[1], registers.get(ins[1]).cloned()),
            "rcv" => if ins[1] == snd.0{
                if let Some(n) = snd.1 {
                    if n > 0 {
                        println!("part 1: {}", snd.1.unwrap());
                        break 'outer;
                    }
                }
            }
            _ => panic!(),
        }
        ip += 1;
    }

    let mut p0 = Programm::new(0, instructions.clone());
    let mut p1 = Programm::new(1, instructions.clone());

    let mut deque_send_p0 = VecDeque::new();
    let mut deque_send_p1 = VecDeque::new();
    loop {
        let retp0 = p0.run(&mut deque_send_p1);
        if let Some(val) = retp0 {
            deque_send_p0.push_back(val);
        }
        let retp1 = p1.run(&mut deque_send_p0);

        if let Some(val) = retp1 {
            deque_send_p1.push_back(val);
        }
        if p0.is_waiting() && p1.is_waiting() {
            println!("part 2: {}", p1.sended);
            break;
        }
    }
}
