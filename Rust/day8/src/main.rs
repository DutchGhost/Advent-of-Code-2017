use std::collections::HashMap;

const PUZZLE: &'static str = include_str!("Input.txt");

#[derive(Debug)]
enum Operator {
    Equal(i64, i64),
    NotEqual(i64, i64),
    Greaterthen(i64, i64),
    Smallerthen(i64, i64),
    GreaterthenOrEqualto(i64, i64),
    SmallerthenOrEqualto(i64, i64),
}

impl Operator {
    fn new(map: &HashMap<String, i64>, operator: String, var: String, n: i64) -> Operator {
        let to_cmp = match map.get(&var) {
            Some(item) => item.clone(),
            None => 0,
        };

        match &*operator {
            "==" => Operator::Equal(to_cmp, n),
            "!=" => Operator::NotEqual(to_cmp, n),
            ">" => Operator::Greaterthen(to_cmp, n),
            "<" => Operator::Smallerthen(to_cmp, n),
            "<=" => Operator::SmallerthenOrEqualto(to_cmp, n),
            ">=" => Operator::GreaterthenOrEqualto(to_cmp, n),
            _ => panic!("I don't know this operator!"),
        }
    }
    
    fn eval(self) -> bool {
        match self {
            Operator::Equal(a, b) => a == b,
            Operator::Greaterthen(a, b) => a > b,
            Operator::Smallerthen(a, b) => a < b,
            Operator::NotEqual(a, b) => a != b,
            Operator::GreaterthenOrEqualto(a, b) => a >= b,
            Operator::SmallerthenOrEqualto(a, b) => a <= b,

        }
    }
}

fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|word| word.to_string()).collect::<Vec<_>>())
        .collect()
}

fn main() {
    let parsed = parse(PUZZLE);
    let mut map = HashMap::new();

    for line in parsed {
        let n = line[6].parse::<i64>().unwrap();
        let operator = Operator::new(&map, line.iter().nth(5).unwrap().to_string(), line.iter().nth(4).unwrap().to_string(), n);

        if operator.eval() {
            match &*(line.iter().nth(1).unwrap().to_string()) {
                "inc" => {
                    *map.entry(line.iter().nth(0).unwrap().to_string()).or_insert(0) += line.iter().nth(2).unwrap().parse::<i64>().unwrap();
                }
                "dec" => {
                    *map.entry(line.iter().nth(0).unwrap().to_string()).or_insert(0) -= line.iter().nth(2).unwrap().parse::<i64>().unwrap();
                }
                _ => panic!("CAN NOT DO THIS"),
            }
        }
    }

    let i = map.iter().max_by_key(|&(_, val)| val);
    println!("{:?}", i);
}
