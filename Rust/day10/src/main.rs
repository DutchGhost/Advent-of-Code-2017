extern crate rayon;

const PUZZLE: &'static str = "31,2,85,1,80,109,35,63,98,255,0,13,105,254,128,33";
const BYTESPUZZLE: [u8; 49] = *b"31,2,85,1,80,109,35,63,98,255,0,13,105,254,128,33";
const SALT: [u8; 5] = [17, 31, 73, 47, 23];

use rayon::prelude::*;
use std::time::Instant;

fn parse_str(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|word| word.parse().expect("Failed to parse"))
        .collect()
}

fn nums() -> Vec<usize> {
    (0..).take(256).collect()
}

fn parse_bytes(input: [u8; 49]) -> Vec<usize> {
    input
        .into_iter()
        .chain(SALT.iter())
        .map(|b| *b as usize)
        .collect()
}

//extends selecteds with the selected items from nums.
//the drain it in reversed order, now we only allocate 1 vector to do all the magic!
fn solve(rounds: i64, nums: &mut [usize], lenghts: &[usize]) -> usize {
    let mut cpos: usize = 0;
    let mut skipsize: usize = 0;
    let numslenght = nums.len();
    let mut selecteds = Vec::with_capacity(200);

    for _ in 0..rounds {
        for len in lenghts.iter() {
            selecteds.extend(nums.iter().cycle().skip(cpos).take(*len));

            (cpos..numslenght)
                .chain(0..)
                .zip(selecteds.drain(..).rev())
                .for_each(|(idx, newnum)| nums[idx] = newnum);

            cpos += (*len + skipsize);
            cpos = cpos % numslenght;
            skipsize += 1;
        }
    }
    nums[0] * nums[1]
}

fn solve2(rounds: i64, nums: &mut [usize], lenghts: &[usize]) {
    let mut cpos = 0;
    let mut skipsize = 0;
    let mut numslenght = nums.len();

    for _ in 0..rounds {
        for len in lenghts.iter() {
            
            //if we don't need to wrap around...
            if cpos + len < numslenght {
                let it = (cpos..cpos + len);
                it.clone().zip(it.rev()).take(len / 2).for_each(|(n1, n2)| nums.swap(n1, n2));
            }
            else {
                //these are before the wraparound (n..255).
                //len minus this will be how many items we get after wraparound
                let already_got = numslenght - cpos;
                let it = (cpos..numslenght).chain(0..(len-already_got));
                it.clone().zip(it.rev()).take(len / 2).for_each(|(n1, n2)| nums.swap(n1, n2));
            }
            cpos += (*len + skipsize);
            cpos = cpos % numslenght;
            skipsize += 1;
        }
    }
}
fn dense(nums: &[usize]) -> String {
    nums.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |n, acc| n ^ acc))
        .map(|chunk| format!("{:02x}", chunk).to_lowercase())
        .collect()
}
fn main() {
    let mut nums_part1 = nums();
    let lenghts_part1 = parse_str(PUZZLE);
    println!("part 1: {}", solve(1, &mut nums_part1, &lenghts_part1));

    let mut nums_part2 = nums();
    let lenghts_part2 = parse_bytes(BYTESPUZZLE);
    let mut n = 1000_0000;
    
    rayon::join(|| {
        let mut test = nums();
    let s = Instant::now();
    solve2(n, &mut test, &lenghts_part2);
    println!("part2 (it):\t{}", dense(&test));
    println!("{:?}", s.elapsed());
    }, || {
        let mut t = Instant::now();
    solve(n, &mut nums_part2, &lenghts_part2);
    println!("part 2:\t\t{}", dense(&nums_part2));
    println!("{:?}", t.elapsed());
    });

}
