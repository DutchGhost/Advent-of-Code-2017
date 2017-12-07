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


fn insert_single_parent(s: String, map: &mut HashMap<String, String>, rows: &[Vec<String>]) {
    //loop over all the lines in the input
    'outer: for line in rows.iter() {

        //get the key and values of the line
        let (key, value) = key_value(line);
        match value {
            None => continue,

            //if there is Some value,
            Some(values) => {

                //iterate over the values
                for v in values.iter() {

                    //if the current value equals the thing we're looking, for insert it into the map.
                    if v == &s {
                        map.insert(s, key);
                        break 'outer;
                    }
                }
            }
        }
    }
}

//for each value in the given values, insert it's parent in the map.
//a parent is a key that points to the value
fn insert_multiple_parents(values: Vec<String>, map: &mut HashMap<String, String>, rows: &[Vec<String>]) {
    //loop over all the items that need a parent
    for val in values {

        //loop over all the lines from the input
        'inner: for line in rows.iter() {

            //parse the line to a key and value
            let (key, value) = key_value(line);

            //if the value was none, we dont care
            match value {
                None => continue,

                //if it was some, 
                Some(values) => {
                    for v in values.iter() {
                        //if one of the CURRENT values from the CURRENT key is the value, we found the key to be inserted.
                        if v == &val {
                            map.insert(val, key);
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
                insert_multiple_parents(values, &mut map, &parsed[..])
            }
            None => {
                insert_single_parent(key, &mut map, &parsed[..])
            }
        }
    }
    for (k, v) in map.iter() {
        match map.get(v) {
            Some(_) => continue,
            None => {println!("{}", v); break},
        }
    }
}
