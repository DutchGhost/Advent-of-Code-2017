use std::collections::HashMap;

enum Operator {
    Equal(i32, i32),
    NotEqual(i32, i32),
    Greaterthan(i32, i32),
    Smallerthan(i32, i32),
    GreaterthanOrEqualto(i32, i32),
    SmallerthanOrEqualto(i32, i32),
}

impl Operator {
    fn new<'a>(cmpregister: &'a str, operator: &'a str, cmp: i32, registers: &HashMap<&'a str, i32>) -> Operator {
        let n = match registers.get(cmpregister) {
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

pub struct Statement<'a> {
    register: &'a str,
    instruction: &'a str,
    value: i32,
    operator: Operator,
}

impl<'a, 'b, 'm> Statement<'a>
where
    'a: 'b,
    'a: 'm,
{
    pub fn register(&self) -> &'a str {
        self.register
    }

    pub fn new(line: Vec<&'a str>, registers: &'b HashMap<&'a str, i32>) -> Statement<'a> {
        match line.as_slice() {
            &[register, instruction, value, cmpregister, operator, val] => {
                Statement {
                    register: register,
                    instruction: instruction,
                    value: value.parse::<i32>().expect(
                        "Failed to parse value to increment or decrement",
                    ),
                    operator: Operator::new(
                        &cmpregister,
                        operator,
                        val.parse::<i32>().expect(
                            "Failed to parse number to compare with",
                        ),
                        registers,
                    ),
                }
            }
            _ => panic!(),
        }
    }

    pub fn eval(&self, registers: &'m mut HashMap<&'a str, i32>) {
        if self.operator.cmp() {
            match self.instruction {
                "inc" => *registers.entry(self.register).or_insert(0) += self.value,
                "dec" => *registers.entry(self.register).or_insert(0) -= self.value,
                _ => panic!("Something when horribly terribly wrong."),
            }
        }
    }
}
