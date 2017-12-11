//Don't know if I like this usized all over the place...
//if go back to u8, change *b as usize in parse_bytes to *b

const PUZZLE: &'static str = include_str!("Input.txt");
const BYTESPUZZLE: &[u8] = include_bytes!("Input.txt");

fn parse_str(input: &str) -> Vec<usize> {
    input.split(",").map(|word| word.parse().expect("Failed to parse")).collect()    
}

fn nums() -> Vec<usize> {
    (0..).take(256).collect()
}

fn parse_bytes(input: &'static [u8]) -> Vec<usize> {
    input.into_iter().chain([17, 31, 73, 47, 23].iter()).map(|b| *b as usize).collect()
}

fn solve(rounds: i64, nums: &mut [usize], lenghts: &[usize], cpos: &mut usize, skipsize: &mut usize) -> usize {
    let numslenght = nums.len();
    let mut indecies = Vec::with_capacity(200);
    let mut selected = Vec::with_capacity(200);
    for _ in 0..rounds {
        for len in lenghts.iter() {
            //the selected items from nums. wraps around. also gets the index.
            (indecies, selected) = nums
                    .iter()
                    .enumerate()
                    .cycle()
                    .skip(*cpos)
                    .take(*len)
                    .unzip();
                
            //loop over the indecies zipped with the reversed of the selected.
            //for each indecie, set nums[indecie] to newnum.
            indecies
                .drain(..)
                .zip(selected.drain(..).rev())
                .for_each(|(indecie, newnum)| nums[indecie] = newnum);

            *cpos += (*len + *skipsize) % numslenght;
            *skipsize += 1;
        }
    }
    nums[0] * nums[1]
}

fn dense(nums: &[usize]) -> String {
    nums
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |n, acc| n ^ acc))
        .map(|chunk| format!("{:02x}", chunk).to_lowercase())
        .collect()
}
fn main() {
    let mut nums_part1 = nums();
    let lenghts_part1 = parse_str(PUZZLE);
    println!("part 1: {}", solve(1, &mut nums_part1, &lenghts_part1, &mut 0, &mut 0));

    let mut current_pos = 0;
    let mut skipsize = 0;

    let mut nums_part2 = nums();
    let lenghts_part2 = parse_bytes(BYTESPUZZLE);
    solve(64, &mut nums_part2, &lenghts_part2, &mut current_pos, &mut skipsize);

    println!("part 2: {}", dense(&nums_part2));
}
