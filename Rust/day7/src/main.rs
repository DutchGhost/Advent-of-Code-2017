use std::collections::HashMap;

const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter(|word| !(word == &"->" || word.contains(&"(")))
                .map(|word| word.chars().filter(|c| c != &',').collect::<String>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn key_value(vec: &[String]) -> (String, Option<Vec<String>>) {
    let (head, tail) = vec.split_first().unwrap();

    if tail.is_empty() {
        return (head.to_string(), None)
    }
    (head.to_string(), Some(tail.to_vec()))
}


//for each value in the given values, insert it's parent in the map.
//a parent is a key that points to the value
fn insert_parents(childs: Vec<String>, map: &mut HashMap<String, String>, rows: &[Vec<String>]) {
    //loop over all the items that need a parent
    for child in childs {

        //loop over all the lines from the input
        'inner: for line in rows.iter() {

            //parse the line to a key and value
            let (key, value) = key_value(line);

            //if the value was none, we dont care (the 'child' would be it's own parent)
            match value {
                None => continue,

                //if it was Some, 
                Some(values) => {
                    for v in values.iter() {
                        
                        //if one of the CURRENT child's from the CURRENT 'parent' points to the child
                        //we found a parent inserted.
                        if v == &child {
                            map.insert(child, key);
                            break 'inner;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let parsed = parse(PUZZLE);
    let mut map = HashMap::new();
    for item in parsed.iter() {
        let (key, opt_value) = key_value(item);

        match opt_value {
            Some(values) => {
                insert_parents(values, &mut map, &parsed[..])
            }
            None => {
                insert_parents(vec![key], &mut map, &parsed[..])
            }
        }
    }
    //loop over the map
    for (k, v) in map.iter() {
        //see if a given's child parent is in the map.
        //if it is, continue. Otherwise, it's the first element of the tree.
        match map.get(v) {
            Some(_) => continue,
            None => {println!("{}", v); break},
        }
    }
}
