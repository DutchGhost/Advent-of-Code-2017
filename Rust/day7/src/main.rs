#![feature(slice_patterns)]
use std::collections::HashMap;

const PUZZLE: &'static str = include_str!("Input.txt");
mod Node;

//for each [Node0] -> [Node1, Node2, Node3],
//store:
//  node1 -> node0,
//  node2 -> node0,
//  node3 -> node0
//in the HashMap.
//The only node that's only a value in the hashmap, is the root!
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