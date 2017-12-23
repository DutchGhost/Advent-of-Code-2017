use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Programm<'a> {
    ip: i64,
    instructions: Vec<Vec<&'a str>>,
    registers: HashMap<&'a str, i64>,
    pub sended: i64,
    id: i64,
    waiting: bool,
}

fn read<'b, 'a: 'b>(s: &'a str, map: &mut HashMap<&'b str, i64>) -> i64 {
    match s.parse::<i64>() {
        Ok(n) => n,
        Err(_) => *map.entry(s).or_insert(0),
    }
}


impl <'a>Programm<'a> {
    pub fn new(id: i64, instructions: Vec<Vec<&'a str>>) -> Programm<'a> {
        let mut registers = HashMap::new();
        registers.insert("p", id);

        Programm {
            ip: 0,
            instructions: instructions,
            registers: registers,
            sended: 0,
            id: id,
            waiting: false,
        }
    }

    pub fn run(&mut self, rcv: &mut VecDeque<i64>) -> Option<i64> {
        let mut ret = None;
        let ins = &self.instructions[self.ip as usize];
        match ins[0].as_ref() {
            "set" => *self.registers.entry(ins[1]).or_insert(0) = read(&ins[2], &mut self.registers),
            "sub" => *self.registers.entry(ins[1]).or_insert(0) -= read(&ins[2], &mut self.registers),
            "add" => *self.registers.entry(ins[1]).or_insert(0) += read(&ins[2], &mut self.registers),
            "mod" => *self.registers.entry(ins[1]).or_insert(0) %= read(ins[2], &mut self.registers),
            "mul" => *self.registers.entry(ins[1]).or_insert(0) *= read(&ins[2], &mut self.registers),
            "jgz" => if read(&ins[1], &mut self.registers) > 0 {
                self.ip += read(&ins[2], &mut self.registers) - 1;
            },

            "snd" => {
                ret = self.registers.get(ins[1]).cloned();
                self.sended += 1;
            },
            "rcv" => {
                if let Some(r) = rcv.pop_front() {
                    *self.registers.entry(ins[1]).or_insert(0) = r;
                    self.waiting = false;
                }
                else {
                    self.waiting = true;
                }
            }
            _ => panic!(),
        }
        if !self.waiting {
            self.ip += 1;
        }
        return ret;
    }

    pub fn is_waiting(&self) -> bool {
        self.waiting
    }
}