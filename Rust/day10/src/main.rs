//Don't know if I like this usized all over the place...
//if go back to u8, change *b as usize in parse_bytes to *b
const PUZZLE: &'static str = include_str!("Input.txt");
const BYTESPUZZLE: &[u8] = include_bytes!("Input.txt");

fn parse_str(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|word| word.parse().expect("Failed to parse"))
        .collect()
}

fn nums() -> Vec<usize> {
    (0..).take(256).collect()
}

fn parse_bytes(input: &'static [u8]) -> Vec<usize> {
    input
        .into_iter()
        .chain([17, 31, 73, 47, 23].iter())
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

            (cpos % numslenght..numslenght)
                .chain(0..)
                .zip(selecteds.drain(..).rev())
                .for_each(|(idx, newnum)| nums[idx] = newnum);

            cpos += (*len + skipsize) % numslenght;
            skipsize += 1;
        }
    }
    nums[0] * nums[1]
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
    solve(64, &mut nums_part2, &lenghts_part2);

    println!("part 2: {}", dense(&nums_part2));
}
