const PUZZLE: usize = 316;
use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let mut buff = VecDeque::with_capacity(100_000_000);
    buff.push_front(0);
    let mut current_pos = 0;

    let time = Instant::now();
    for i in 1..50_000_001 {
        //let (idx, _) = buff.iter().enumerate().cycle().skip(current_pos + 1).take(PUZZLE).last().unwrap();
        let idx = (current_pos + PUZZLE) % buff.len();
        if idx == 0 {
            println!("0!");
            buff.push_front(i);
        }
        else if idx == i {
            println!("LEN");
            buff.push_back(i);
        }
        else {
             buff.insert(idx + 1, i);
        }
        
        current_pos = idx + 1;
        if i % 10_000 == 0 {println!("{} {:?}", i, time.elapsed());}
    }

    let idx = buff.iter().position(|item| item == &0).unwrap();
    println!("part 2: {}", buff[idx + 1]);

    let mut ans = 0;
    let mut nxt = 0;
    for i in 1..50_000_001 {
        nxt = (nxt + PUZZLE) % i;
        if nxt == 0 {ans = i;}
        nxt += 1;
    }
    println!("part 2:{}", ans);
}
