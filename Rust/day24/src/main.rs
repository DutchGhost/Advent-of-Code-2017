extern crate itertools;
use itertools::*;

const PUZZLE: &'static str = include_str!("Input.txt");

mod component;
use component::*;

use std::collections::HashMap;

use std::str::FromStr;

fn parse(input: &str) -> Vec<Component> {
    input.lines().map(|line| Component::from_str(line).unwrap()).collect()
}

fn make_comb_tree(components: &[Component]) -> HashMap<&Component, Vec<&Component>>{
    let mut map = HashMap::new();
    for (idx, component) in components.iter().enumerate() {
        for comp2 in components[idx + 1..].iter() {
            if component.connectable(&comp2) {
                map.entry(component).or_insert(Vec::new()).push(comp2);
            }
        }
    }

    let mut secondmap = HashMap::new();

    for (k, v) in map.iter() {
        for component in v.iter() {
            if let Some(comps) = map.get(component) {
                let mut vclone = v.clone();
                vclone.extend(comps.clone());
                secondmap.insert(*k, vclone);
            }
            else {
                secondmap.insert(*k, v.to_vec());
            }
        }
    }

    let mut thirdmap = HashMap::new();

    for (k, v) in secondmap.iter() {
        for component in v.iter() {
            if let Some(comps) = map.get(component) {
                let mut vclone = v.clone();
                vclone.extend(comps.clone());
                thirdmap.insert(*k, vclone);
            }
            else {
                thirdmap.insert(*k, v.to_vec());
            }
        }
    }

    thirdmap
}


fn main() {
    let mut components = parse(PUZZLE);
    
    let map = make_comb_tree(&components);

    for (k, v) in map.iter() {
        println!("{:?} -> {:?}", k, v);
    }
}
