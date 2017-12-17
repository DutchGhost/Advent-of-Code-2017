#![feature(slice_rotate)]
const PUZZLE: usize = 316;

fn main() {
    let mut buff = vec![0];

    for i in 1..2018 {
        let l = buff.len();
        buff.rotate(PUZZLE % l);
        buff.push(i);
    }

    let idx = buff.iter().position(|item| item == &2017).unwrap();
    let l = buff.len();
    println!("part 1: {}", buff[(idx + 1) % l ]);

    let mut ans = 0;
    let mut nxt = 0;
    for i in 1..50_000_001 {
        nxt = (nxt + PUZZLE) % i;
        if nxt == 0 {ans = i;}
        nxt += 1;
    }
    println!("part 2: {}", ans);
}
