const PUZZLE: &'static str = include_str!("Input.txt");

use std::collections::HashMap;

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


    'outer: while rcv == 0 {
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
                        println!("{:?}", snd.1);
                        break 'outer;
                    }
                }
            }
            _ => panic!(),
        }
        ip += 1;
    }
    println!("{:?}", registers);
}
