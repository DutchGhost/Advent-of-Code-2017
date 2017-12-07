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
    //childs pointing to a parent. K is the child, V the parent.
    let mut set = parse(PUZZLE);

    for (k, v) in set.iter() {
        if !set.contains_key(v) {
            println!("part 1: {}", v);
            break;
        }
    }
}

/*
           /     
         ugml - ebii
       /      \     
      |         jptl
      |        
      |         pbga
     /        /
tknk --- padx - havc
     \        \
      |         qoyq
      |             
      |         ktlj
       \      /     
         fwft - cntj
              \     
                xhth

*/      