const SALT: [u8; 5] = [17, 31, 73, 47, 23];
const PUZZLE: &'static str = "hxtvlmkl";

use std::iter::*;
use std::ops::Range;

//wrapper for the 'range' of nums that need to be changed.
enum Wrapper {
    Wrapped(Zip<Chain<Range<usize>, Range<usize>>, Rev<Chain<Range<usize>, Range<usize>>>>),
    Nonwrapped(Zip<Range<usize>, Rev<Range<usize>>>),
}

//'Wrapped' goes from cpos to numslenght, and then from 0 to (len - (numslenght - cpos))
//'Nonwrapped' goes from cpos to cpos + len
fn wrapping(cpos: usize, len: usize, numslenght: usize) -> Wrapper {
    if cpos + len < numslenght {
        Wrapper::Nonwrapped((cpos..cpos + len).zip((cpos..cpos + len).rev()))
    } else {
        
        let already_got = numslenght - cpos;

        Wrapper::Wrapped((cpos..numslenght).chain(0..(len - already_got)).zip(
            (cpos..numslenght).chain(0..(len - already_got)).rev(),
        ))
    }
}
fn solve(lenghts: &[usize]) -> String {
    let mut nums =  (0..).take(256).collect::<Vec<usize>>();
    let mut cpos = 0;
    let mut skipsize = 0;
    let mut numslenght = nums.len();

    for _ in 0..64 {
        for len in lenghts.iter() {

            let it = wrapping(cpos, *len, numslenght);

            match it {
                Wrapper::Wrapped(iter) => iter.take(len / 2).for_each(|(n1, n2)| nums.swap(n1, n2)),
                Wrapper::Nonwrapped(iter) => iter.take(len / 2).for_each(|(n1, n2)| nums.swap(n1, n2)),
            };

            cpos += (*len + skipsize);
            cpos = cpos % numslenght;
            skipsize += 1;
        }
    }
    nums.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |n, acc| n ^ acc))
        .map(|chunk| format!("{:02x}", chunk).to_lowercase())
        .map(|hex| u32::from_str_radix(&hex, 16).unwrap())
        .map(|i| format!("{:04b}", i))
        .collect()
}

fn main() {
    println!("{}", (0..128)
        .map(|n| format!("{}-{}", PUZZLE, n))
        .map(|s| s.as_bytes().iter().chain(SALT.iter()).map(|b| *b as usize).collect::<Vec<usize>>())
        .map(|saltedbytes| solve(&saltedbytes))
        .map(|bin| bin.chars().filter(|c| c == &'1').count() as i32)
        .sum::<i32>())
}
