use std::collections::HashMap;

enum Operator {
    Equal(i32, i32),
    NotEqual(i32, i32),
    Greaterthen(i32, i32),
    Smallerthen(i32, i32),
    GreaterthenOrEqualto(i32, i32),
    SmallerthenOrEqualto(i32, i32),
}

impl Operator {
    fn new(other: &str, operator: &str, cmp: i32, map: &HashMap<&str, i32>) -> Operator {
        let n = match map.get(other) {
            Some(item) => *item,
            None => 0,
        };

        match &*operator {
            "==" => Operator::Equal(n, cmp),
            "!=" => Operator::NotEqual(n, cmp),
            ">" => Operator::Greaterthen(n, cmp),
            "<" => Operator::Smallerthen(n, cmp),
            "<=" => Operator::SmallerthenOrEqualto(n, cmp),
            ">=" => Operator::GreaterthenOrEqualto(n, cmp),
            _ => panic!("I don't know this operator!"),
        }
    }
    
    fn cmp(&self) -> bool {
        match self {
            &Operator::Equal(a, b) => a == b,
            &Operator::Greaterthen(a, b) => a > b,
            &Operator::Smallerthen(a, b) => a < b,
            &Operator::NotEqual(a, b) => a != b,
            &Operator::GreaterthenOrEqualto(a, b) => a >= b,
            &Operator::SmallerthenOrEqualto(a, b) => a <= b,

        }
    }
}

pub struct Statement<'a> {
    name: &'a str,
    operation: &'a str,
    value: i32,
    other: &'a str,
    operator: Operator,
}

impl <'a>Statement<'a> {

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn new(line: Vec<&'a str>, map: &HashMap<&str, i32>) -> Statement<'a> {
        let mut it = line.into_iter();

        //the name of the register,
        let name = it.next().expect("Could not fetch the name");

        //increment of decrement
        let operation = it.next().expect("Could not fetch the operation");

        //with how many must be incremented or decremented
        let value = it.next().unwrap().parse::<i32>().expect("could not parse");

        //the item to check
        let other = it.next().expect("Could not get the register");

        //greater then, smaller then, equal to, eqal or greater then..
        let operator = it.next().expect("could not get the operator");

        //the value it's compared with
        let val = it.next().unwrap().parse::<i32>().expect("could not parse");

        Statement {
            name: name,
            operation: operation,
            value: value,
            other: other,
            operator: Operator::new(&other, operator, val, map),
        }
    }

    pub fn eval(&self, map: &mut HashMap<&'a str, i32>) {
        if self.operator.cmp() {
            match self.operation {
                "inc" => *map.entry(self.name).or_insert(0) += self.value,
                "dec" => *map.entry(self.name).or_insert(0) -= self.value,
                _ => panic!("Something when horribly terribly wrong.")
            }
        }
    }
}