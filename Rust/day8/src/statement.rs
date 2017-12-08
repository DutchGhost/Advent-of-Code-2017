use std::collections::HashMap;
use std::cmp::max;

//A register
#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Register<'a> {
    name: &'a str
}

//Holds Registers, with their values
pub struct Registers<'a> {
    max: i32,
    registers: HashMap<Register<'a>, i32>,
}

//An Instruction. Increment(Register, Value) or Decrement(Register, Value).
enum Instruction<'a> {
    inc(Register<'a>, i32),
    dec(Register<'a>, i32),
}

//An operator, based on this a register's value is incremented or decremented
enum Operator {
    Equal(i32, i32),
    NotEqual(i32, i32),
    Greaterthan(i32, i32),
    Smallerthan(i32, i32),
    GreaterthanOrEqualto(i32, i32),
    SmallerthanOrEqualto(i32, i32),
}

//A statement. [instruction] [register] with [value] if [value of a register] [operator] [othervalue]
pub struct Statement<'a> {
    instruction: Instruction<'a>,
    operator: Operator,
}


impl <'a>Register<'a> {
    fn new(name: &'a str) -> Register<'a> {
        Register {
            name: name
        }
    }
}

impl <'a>Registers<'a> {
    pub fn new() -> Registers<'a> {
        Registers {
            max: 0,
            registers: HashMap::new(),
        }
    }

    fn get<'v, 's: 'v>(&'s self, k: &Register<'a>) -> Option<&'v i32> {
        self.registers.get(k)
    }

    fn update(&mut self, operator: &Operator, instruction: Instruction<'a>) {
        if operator.cmp() {
            match instruction {
                Instruction::inc(register, value) => {
                    *self.registers.entry(register.clone()).or_insert(0) += value;
                    self.max = max(self.max, self.registers.get(&register).unwrap().clone());
                },
                Instruction::dec(register, value) => {
                    *self.registers.entry(register.clone()).or_insert(0) -= value;
                    self.max = max(self.max, self.registers.get(&register).unwrap().clone())
                },
            };

        }
    }

    pub fn max(self) -> (i32, i32) {
        (self.registers.values().max().unwrap().clone(), self.max)
    }
}

impl <'a>Instruction<'a> {
    fn new(ins: &'a str, register: Register<'a>, value: &'a str) -> Instruction<'a> {
        match ins {
            "inc" => Instruction::inc(register, value.parse::<i32>().expect("Invalid incremental value")),
            "dec" => Instruction::dec(register, value.parse::<i32>().expect("Invalid decremental value")),
            _ => panic!("unknown instruction"),
        }
    }
}

impl Operator {
    fn new<'a>(cmpregister: Register<'a>, operator: &'a str, cmp: i32, registers: &Registers) -> Operator {
        let n = match registers.get(&cmpregister) {
            Some(item) => *item,
            None => 0,
        };

        match &operator {
            &"==" => Operator::Equal(n, cmp),
            &"!=" => Operator::NotEqual(n, cmp),
            &">" => Operator::Greaterthan(n, cmp),
            &"<" => Operator::Smallerthan(n, cmp),
            &"<=" => Operator::SmallerthanOrEqualto(n, cmp),
            &">=" => Operator::GreaterthanOrEqualto(n, cmp),
            _ => panic!("I don't know this operator!"),
        }
    }

    fn cmp(&self) -> bool {
        match self {
            &Operator::Equal(a, b) => a == b,
            &Operator::Greaterthan(a, b) => a > b,
            &Operator::Smallerthan(a, b) => a < b,
            &Operator::NotEqual(a, b) => a != b,
            &Operator::GreaterthanOrEqualto(a, b) => a >= b,
            &Operator::SmallerthanOrEqualto(a, b) => a <= b,
        }
    }
}

impl<'a, 'b, 'm> Statement <'a>
where
    'a: 'b,
    'a: 'm,
{
    pub fn new(line: Vec<&'a str>, registers: &'b Registers) -> Statement<'a> {
        match line.as_slice() {
            &[register, instruction, value, cmpregister, operator, val] => {
                Statement {
                    instruction: Instruction::new(instruction, Register::new(register), value),
                    operator: Operator::new(
                        Register::new(cmpregister),
                        operator,
                        val.parse::<i32>().expect("Failed to parse the number to compare with."),
                        registers,
                    ),
                }
            },
            _ => panic!("Failed to parse the input."),
        }
    }

    pub fn eval(self, registers: &'m mut Registers<'a>) {
        registers.update(&self.operator, self.instruction);
    }
}

/*
    NOTE:
        an if-statement has an expression, and a instruction.
        IF expression {
            Statement
        }
        Statement if Expression
*/