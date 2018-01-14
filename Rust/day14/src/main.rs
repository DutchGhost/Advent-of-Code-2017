const SALT: [u8; 5] = [17, 31, 73, 47, 23];
const PUZZLE: &'static str = "hxtvlmkl";

const NUMS: [usize; 256] = [
          0,  1,    2,   3,   4,   5,   6,   7,   8,   9,
         10,  11,  12,  13,  14,  15,  16,  17,  18,  19,
         20,  21,  22,  23,  24,  25,  26,  27,  28,  29,
         30,  31,  32,  33,  34,  35,  36,  37,  38,  39, 
         40,  41,  42,  43,  44,  45,  46,  47,  48,  49,
         50,  51,  52,  53,  54,  55,  56,  57,  58,  59,
         60,  61,  62,  63,  64,  65,  66,  67,  68,  69,
         70,  71,  72,  73,  74,  75,  76,  77,  78,  79,
         80,  81,  82,  83,  84,  85,  86,  87,  88,  89,
         90,  91,  92,  93,  94,  95,  96,  97,  98,  99,
        100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
        110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
        120, 121, 122, 123, 124, 125, 126, 127, 128, 129,
        130, 131, 132, 133, 134, 135, 136, 137, 138, 139,
        140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
        150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
        160, 161, 162, 163, 164, 165, 166, 167, 168, 169,
        170, 171, 172, 173, 174, 175, 176, 177, 178, 179,
        180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
        190, 191, 192, 193, 194, 195, 196, 197, 198, 199,
        200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
        210, 211, 212, 213, 214, 215, 216, 217, 218, 219,
        220, 221, 222, 223, 224, 225, 226, 227, 228, 229,
        230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
        240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
        250, 251, 252, 253, 254, 255
    ];

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
    let mut nums = NUMS;

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
    if r >= v.len() || c >= v[r].len() || v[r][c] == 0 {
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
