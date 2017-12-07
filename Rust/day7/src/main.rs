use std::collections::HashMap;

const PUZZLE: &'static str = include_str!("Input.txt");

fn values(input: &str) -> HashMap<String, i64> {
    let mut values = HashMap::new();

    for line in input.lines() {
        let name_value = line.split_whitespace().take(2).collect::<Vec<_>>();

        let value = name_value
            .iter()
            .nth(1)
            .unwrap()
            .chars()
            .filter(|c| !(c == &'(' || c == &')'))
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        let name = name_value
            .iter()
            .nth(0)
            .unwrap()
            .to_string();

        values.insert(name, value);
    }
    values
}

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
    //childs pointing to a parent. K is the child, V the parent.
    let mut set = parse(PUZZLE);
    let values = values(PUZZLE);

    let mut part2: HashMap<String, Vec<i64>> = HashMap::new();

    for (k, v) in set.iter() {
        if !set.contains_key(v) {
            println!("part 1: {}", v);
            break;
        }
    }

    //child : parent
    //get the parent, or insert an emptry vec
    //then add the value of the child it.
    //but maybe. the child IS also a parent.
    for (k, v) in set.iter_mut() {
        if is_child(k, &set) {
            part2.entry(v.clone()).or_insert(Vec::new()).push(values.get(k).unwrap().clone())
        }
        //after we've had a child, we dont really need it anymore????
    }
}

//if nothing points to check, check IS a child.
fn is_child(check: &String, map: &HashMap<String, String>) -> bool {
    for (child, parent) in map.iter() {
        if parent == check {
            return false
        }
    }
    return true
}