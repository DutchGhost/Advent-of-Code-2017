use std::collections::HashMap;
use std::cmp::max;

pub trait from_str_and_hashmap<'r>
{
    type Err;
    fn from_s<'a: 'r>(s: &'a str, map: &Registers<'r>) -> Result<Statement<'r>, Self::Err>;
}

//A register
#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Register<'r> {
    name: &'r str
}

//Holds Registers, with their values
pub struct Registers<'r> {
    max: i32,
    registers: HashMap<Register<'r>, i32>,
}

//An Instruction. Increment(Register, Value) or Decrement(Register, Value).
enum Instruction<'r>
{
    Operation(&'static Fn(i32) -> i32, Register<'r>, i32)
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
pub struct Statement<'r>
{
    instruction: Instruction<'r>,
    operator: Operator,
}


impl <'r>Register<'r> {
    fn new(name: &'r str) -> Register<'r> {
        Register { name: name }
    }
}

impl <'i, 'r>Registers<'r> {
    pub fn new() -> Registers<'r> {
        Registers {
            max: 0,
            registers: HashMap::new(),
        }
    }

    fn get<'v, 's: 'v>(&'s self, k: &Register<'r>) -> Option<&'v i32> {
        self.registers.get(k)
    }

    fn update(&mut self, operator: &Operator, instruction: Instruction<'r>)
    {
        if operator.cmp() {
            match instruction {
                Instruction::Operation(operation, register, value) => {
                    *self.registers.entry(register.clone()).or_insert(0) += operation(value);
                    self.max = max(self.max, self.registers.get(&register).unwrap().clone());
                }
            }
        }
    }

    pub fn max(self) -> (i32, i32) {
        (self.registers.values().max().unwrap().clone(), self.max)
    }
}

impl <'a, 'r, 'i>Instruction<'r>
{
    fn new(ins: &'a str, register: Register<'r>, value: &'a str) -> Instruction<'r> {
        match ins {
            "inc" => Instruction::Operation(&Self::inc, register, value.parse::<i32>().expect("Invalid incremental value")),
            "dec" => Instruction::Operation(&Self::dec, register, value.parse::<i32>().expect("Invalid decremental value")),
            _ => panic!("unknown instruction"),
        }
    }
    fn inc(value: i32) -> i32 { value }
    fn dec(value: i32) -> i32 { -value }
}
impl Operator {
    fn new<'r, 'a, 'rs>(cmpregister: Register<'r>, operator: &'a str, cmp: i32, registers: &Registers<'r>) -> Operator {
        let n = *registers.get(&cmpregister).unwrap_or(&0i32);

        match &operator {
            &"==" => Operator::Equal(n, cmp),
            &"!=" => Operator::NotEqual(n, cmp),
            &"<"  => Operator::Smallerthan(n, cmp),
            &">"  => Operator::Greaterthan(n, cmp),
            &"<=" => Operator::SmallerthanOrEqualto(n, cmp),
            &">=" => Operator::GreaterthanOrEqualto(n, cmp),
            _ => panic!("I don't know this operator!"),
        }
    }

    fn cmp(&self) -> bool {
        match self {
            &Operator::Equal(a, b) => a == b,
            &Operator::NotEqual(a, b) => a != b,
            &Operator::Smallerthan(a, b) => a < b,
            &Operator::Greaterthan(a, b) => a > b,
            &Operator::SmallerthanOrEqualto(a, b) => a <= b,
            &Operator::GreaterthanOrEqualto(a, b) => a >= b,
        }
    }
}

impl<'r, 'b, 'm, 'a> Statement <'r>
where
    'a: 'b,
    'a: 'm,
{
    pub fn new(line: Vec<&'r str>, registers: &'b Registers) -> Result<Statement<'r>, StatementError> {
        match line.as_slice() {
            &[register, instruction, value, cmpregister, operator, otherval] => {
                Ok(Statement {
                    instruction: Instruction::new(instruction, Register::new(register), value),
                    operator: Operator::new(
                        Register::new(cmpregister),
                        operator,
                        otherval.parse::<i32>().expect("Failed to parse the number to compare with."),
                        registers,
                    ),
                })
            },
            _ => Err(StatementError::new("Could not match on the slice.")),
        }
    }

    pub fn eval(self, registers: &'m mut Registers<'r>) {
        registers.update(&self.operator, self.instruction);
    }
}

pub struct StatementError {
    discription: String,
}

impl StatementError {
    pub fn new<'a>(s: &'a str) -> StatementError {
        StatementError {
            discription: String::from(s),
        }
    }
    pub fn discription(self) -> String {
        self.discription
    }
}
impl <'r>from_str_and_hashmap<'r> for Statement<'r>
{
    type Err = StatementError;

    fn from_s<'a: 'r>(s: &'a str, map: &Registers<'r>) -> Result<Statement<'r>, StatementError> {
        let v = s.split_whitespace().collect::<Vec<_>>();
        Statement::new(v, map)
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