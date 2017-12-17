const PUZZLE: usize = 316;

fn main() {
    let mut buff = vec![0];

    let mut current_pos = 0;
    for i in 1..2018 {
        let (idx, _) = buff.iter().enumerate().cycle().skip(current_pos + 1).take(PUZZLE).last().unwrap();
        current_pos = idx + 1;
        buff.insert(idx + 1, i);
    }

    let idx = buff.iter().position(|item| item == &2017).unwrap();
    println!("part 1: {}", buff[idx + 1]);

    let mut ans = 0;
    let mut nxt = 0;
    for i in 1..50_000_001 {
        nxt = (nxt + PUZZLE) % i;
        if nxt == 0 {ans = i;}
        nxt += 1;
    }
    println!("part 2: {}", ans);
}
