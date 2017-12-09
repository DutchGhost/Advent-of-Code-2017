const PUZZLE: &'static str = include_str!("Input.txt");

fn solve(input: &str) -> (i64, i64) {
    let mut level = 0;
    let mut score = 0;

    let mut garbage = false;
    let mut gccount = 0;

    let mut cs = input.chars();

    while let Some(c) = cs.next() {
        match c {
            //on a '!', we ignore the next character
            '!' => {cs.next();},
            
            //if we're already IN some pile of garbage, increment the gccount.
            //and set garbage always to true
            '<' => {
                if garbage {
                    gccount += 1;
                }
                garbage = true;
            }
            //if we're leaving some garbage, set garbage to false.
            '>' => garbage = false,
            //if we are NOT in garbage, and we should go up a level, increment the level
            '{' if !garbage => level += 1,

            //if we're NOT in garbage, and we should go down a level,
            //update the score, and decrement the level
            '}' if !garbage => {
                score += level;
                level -= 1;
            }
            //otherewise, if we happen to be in some garbage, increment the gccount.
            _ => if garbage {
                gccount += 1;
            },
        };
    }
    (score, gccount)
}

fn main() {
    println!("{:?}", solve(PUZZLE));
}
