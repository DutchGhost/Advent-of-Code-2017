use std::collections::HashMap;

const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(input: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let parent_childs = line
                .split_whitespace()
                .filter(|word| !(word == &"->" || word.contains(')')))
                .map(|word| word.replace(",", ""))
                .collect::<Vec<_>>();
        
        let (head, tail) = parent_childs.split_first().unwrap();

        for item in tail {
            map.insert(item.clone(), head.clone());
        }
    }
    map
}

fn main() {
    let set = parse(PUZZLE);
    for (k, v) in set.iter() {
        match set.get(v) {
            Some(_) => continue,
            None => {println!("{}", v); break}
        }
    }
}