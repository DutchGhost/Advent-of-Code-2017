const PUZZLE: &'static str = include_str!("Input.txt");

use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<&str>> {
    input.lines().map(|line| line.split(" ").collect()).collect()
}

//returns either a value that needs top copied, or the value of a register
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
    let mut muls = 0;
    while ip < instructions.len() as i64 {
        let ins = &instructions[ip as usize];
        match ins[0].as_ref() {
            "set" => *registers.entry(ins[1]).or_insert(0) = read(&ins[2], &mut registers),
            "sub" => *registers.entry(ins[1]).or_insert(0) -= read(&ins[2], &mut registers),
            "mul" => {
                *registers.entry(ins[1]).or_insert(0) *= read(&ins[2], &mut registers);
                muls += 1;
            },
            "jnz" => if read(&ins[1], &mut registers) != 0 {ip += read(&ins[2], &mut registers)-1;},
            _ => panic!(),
        }
        ip += 1;
    }
    println!("{:?}", registers);
    println!("{}", muls);
}