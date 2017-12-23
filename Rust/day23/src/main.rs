const PUZZLE: &'static str = include_str!("Input.txt");

use std::collections::BTreeMap;

//returns either a value that needs top copied, or the value of a register
fn read(s: &str, map: &BTreeMap<String, i64>) -> i64 {
    match s.parse::<i64>() {
        Ok(n) => n,
        Err(_) => *map.get(s).unwrap(),
    }
}

fn main() {
    let mut registers = vec![
            ("a".to_string(), 0i64),
            ("b".to_string(), 0i64),
            ("c".to_string(), 0i64),
            ("d".to_string(), 0i64),
            ("e".to_string(), 0i64),
            ("f".to_string(), 0i64),
            ("g".to_string(), 0i64),
            ("h".to_string(), 0i64)]
        .into_iter()
        .collect::<BTreeMap<String, i64>>();

    let instructions: Vec<Vec<String>> = PUZZLE.lines()
        .map(|s| s.split(" ").map(|ss| ss.to_owned()).collect())
        .collect();
    let mut ip: i64 = 0;
    let mut muls = 0;
    while ip < instructions.len() as i64 {
        let ins = &instructions[ip as usize];
        match ins[0].as_ref() {
            "set" => *registers.get_mut(&ins[1]).unwrap() = read(&ins[2], &registers),
            "sub" => *registers.get_mut(&ins[1]).unwrap() -= read(&ins[2], &registers),
            "mul" => {*registers.get_mut(&ins[1]).unwrap() *= read(&ins[2], &registers); muls += 1;},
            "jnz" => if read(&ins[1], &registers) != 0 {ip += read(&ins[2], &registers)-1;},
            _ => panic!(),
        }
        ip += 1;
    }
    println!("{:?}", registers);
    println!("{}", muls);
}