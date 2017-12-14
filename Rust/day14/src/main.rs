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
//return the 16 bytes at the end, so we can do both part 1 and part 2 with this function.
fn solve(lenghts: &[usize]) -> Vec<usize> {
    let mut nums =  (0..).take(256).collect::<Vec<usize>>();
    let mut cpos = 0;
    let mut skipsize = 0;
    let numslenght = nums.len();

    for _ in 0..64 {
        for len in lenghts.iter() {

            let it = wrapping(cpos, *len, numslenght);

            match it {
                Wrapper::Wrapped(iter) => iter.take(len / 2).for_each(|(n1, n2)| nums.swap(n1, n2)),
                Wrapper::Nonwrapped(iter) => iter.take(len / 2).for_each(|(n1, n2)| nums.swap(n1, n2)),
            };

            cpos += *len + skipsize;
            cpos = cpos % numslenght;
            skipsize += 1;
        }
    }
    nums.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |n, acc| n ^ acc))
        .collect()
}

fn part2() {
    let mut grid = Vec::new();
    for y in 0..128 {
        let str_to_hash = format!("{}-{}", PUZZLE, y);
        let bytes_vec = str_to_hash.as_bytes().iter().chain(SALT.iter()).map(|b| *b as usize).collect::<Vec<_>>();
        let bin = solve(&bytes_vec);
        let binairy = bin.iter().map(|n| format!("{:08b}", n)).collect::<String>();
        let bitvec = binairy.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        grid.push(bitvec);
    }

    let mut s = 0;
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            s += wipe_region(&mut grid[..], j, i);
        }
    }
    println!("{}", s);
}

fn wipe_region(v: &mut [Vec<u32>], r: usize, c: usize) -> i64 {
    if r < 0 || c < 0 || r >= v.len() || c >= v[r].len() || v[r][c] == 0 {
        return 0;
    }
    v[r][c] = 0;
    wipe_region(v, r + 1, c);
    wipe_region(v, r, c + 1);
    wipe_region(v, r - 1, c);
    wipe_region(v, r, c - 1);
    return 1;
}

fn main() {
    println!("part 1: {}", (0..128)
        .map(|n| format!("{}-{}", PUZZLE, n))
        .map(|s| s.as_bytes().iter().chain(SALT.iter()).map(|b| *b as usize).collect::<Vec<_>>())
        .map(|saltedbytes| solve(&saltedbytes))
        .map(|hash| hash.iter().map(|i| i.count_ones()).sum::<u32>())
        .sum::<u32>());

    part2()
}
