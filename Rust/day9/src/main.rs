const PUZZLE: &'static str = include_str!("Input.txt");
//on a '!', we ignore the next character
//on a '<', if we're already IN some pile of garbage, increment the gccount.
//  and set garbage always to true
//on a '>', we leave the garbage. Set garbage to false.
//on a '{', if we're not in some garbage, go up a level.
//on a '}', if we're not in some garbage, update the score, and go down a level.
//Otherewise, if we're currently in some garbage, increment the gccount.
fn solve(input: &str) -> (i64, i64) {
    let mut level = 0;
    let mut score = 0;

    let mut garbage = false;
    let mut gccount = 0;

    let mut cs = input.chars();

    while let Some(c) = cs.next() {
        match c {
            '!'             => { cs.next(); },
            '>'             => garbage = false,
            '{' if !garbage => level += 1,
            '}' if !garbage => { score += level; level -= 1; }
            '<'             => { if garbage { gccount += 1; } garbage = true; }
            _               => if garbage { gccount += 1; },
        };
    }
    (score, gccount)
}

fn main() {
    println!("{:?}", solve(PUZZLE));
}
