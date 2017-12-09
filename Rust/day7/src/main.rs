 #![feature(slice_patterns)]
extern crate regex;
use regex::Regex;

use std::collections::HashSet;
use std::collections::HashMap;

const PUZZLE: &'static str = include_str!("Input.txt");

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Tower {
    name: String,
    weights: i64,
    aboves: Vec<String>
}

impl Tower {
    fn new(name: &str, weights: &str, aboves: Vec<String>) -> Tower {
        Tower {
            name: name.to_string(), 
            weights: weights.parse().expect("Could not parse weight"),
            aboves: aboves,
        }
    }
}

fn parse(line: &str) -> Tower {
    let parts = line.split("->").collect::<Vec<_>>();
    let aboves = if parts.len() == 1 { Vec::new() } else {parts[1].split(",").map(|word| word.trim().to_string()).collect::<Vec<_>>()};
    let re = Regex::new(r"([a-z]+) \(([0-9]+)\)").unwrap();

    let capts = re.captures(parts[0]).unwrap();

    Tower::new(capts.get(1).map_or("", |m| m.as_str()), capts.get(2).map_or("", |m| m.as_str()), aboves)

}

fn find_bottem(Towers: &Vec<Tower>) -> String {
    let mut are_aboves = HashSet::new();
    let mut have_aboves = HashSet::new();

    for tower in Towers.iter() {
        if tower.aboves.len() > 0 {
            have_aboves.insert(tower.name.clone());
        }
        for above in tower.aboves.iter() {
            are_aboves.insert(above.clone());
        }
    }
    
    let diff = have_aboves.difference(&are_aboves).next().unwrap();
    diff.clone()
}

fn balance_towers(towers: Vec<Tower>) {

    let lookups = towers.iter().map(|tower| (tower.name.clone(), tower.clone())).collect::<HashMap<String, Tower>>();
    let root = lookups.get(&find_bottem(&towers)).unwrap();
    check(&root, &lookups);
    
}

fn check(tower: &Tower, lookup: &HashMap<String, Tower>) -> (i64, bool) {
    let subchecks = tower.aboves
        .iter()
        .map(|name| (name.clone(), check(lookup.get(name).unwrap(), lookup)))
        .collect::<HashMap<String, (i64, bool)>>();
    let subcheck_weights = subchecks.values().map(|&(w, _)| w).collect::<HashSet<_>>();
    let is_balanced = subcheck_weights.len() <= 1;
    let mut weight = subchecks.values().map(|&(w, _)| w).sum();
    weight += tower.weights;

    if subcheck_weights.len() > 1 && subchecks.values().all(|&(_, is_balanced)| is_balanced) {
        for (name, &(total_weight, is_balanced)) in subchecks.iter() {
            let above_tower = lookup.get(name).unwrap();
            
            println!("{}, {}, {}", name, total_weight, above_tower.weights);
        }
    }
    return (weight, is_balanced)
}



fn main() {
    let towers = PUZZLE.lines().map(|line| parse(line)).collect::<Vec<_>>();
    println!("{:?}", find_bottem(&towers));
    balance_towers(towers);
}