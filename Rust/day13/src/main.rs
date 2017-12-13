#![feature(test)]
extern crate rayon;
extern crate test;

use rayon::prelude::*;

const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(": ").map(|item| {
                item.parse::<i32>().expect("Failed to parse")
            });
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn solve(firewall: &[(i32, i32)]) -> i32 {
    firewall
        .iter()
        .filter(|&&(depth, range)| depth % ((range - 1) * 2) == 0)
        .map(|&(depth, range)| depth * range)
        .sum()
}

fn par_solve(firewall: &[(i32, i32)]) -> i32 {
    firewall
        .par_iter()
        .filter(|&&(depth, range)| depth % ((range - 1) * 2) == 0)
        .map(|&(depth, range)| depth * range)
        .sum()
}


fn par_solve2(firewall: &[(i32, i32)]) -> i32 {
    (0..10_000_000).into_par_iter()
        .find_first(|wait| {
            firewall.iter().all(|&(depth, range)| {
                (depth + wait) % ((range - 1) * 2) != 0
            })
        })
        .unwrap()
}

fn solve2(firewall: &[(i32, i32)]) -> i32 {
    (0..10_000_000)
        .find(|wait| {
            firewall.iter().all(|&(depth, range)| {
                (depth + wait) % ((range - 1) * 2) != 0
            })
        })
        .unwrap()
}



fn main() {
    let parsed = parse(PUZZLE);
    
    println!("{}", par_solve(&parsed));
    println!("{}", par_solve2(&parsed));

    println!("{}", solve(&parsed));
    println!("{}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;
    
    #[bench]
    fn par_solve_1(b: &mut Bencher) {
        let parsed = parse(PUZZLE);
        b.iter(|| par_solve(&parsed));
    }

    #[bench]
    fn singe_solve_1(b: &mut Bencher) {
        let parsed = parse(PUZZLE);
        b.iter(|| solve(&parsed));
    }

    #[bench]
    fn par_solve_2(b: &mut Bencher) {
        let parsed = parse(PUZZLE);
        b.iter(|| par_solve2(&parsed))
    }

    #[bench]
    fn single_solve_2(b: &mut Bencher) {
        let parsed = parse(PUZZLE);
        b.iter(|| solve2(&parsed));
    }
}