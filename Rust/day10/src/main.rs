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

fn solve(nums: &mut [usize], lenghts: &[usize], current_pos: &mut usize, skipsize: &mut usize) -> usize {
    let numslenght = nums.len();
    for len in lenghts.iter() {
        //the selected items from nums. wraps around. also gets the index.
        let (indecies, selected): (Vec<usize>, Vec<usize>) = nums
                .iter()
                .enumerate()
                .cycle()
                .skip(*current_pos)
                .take(*len)
                .unzip();
            
        //loop over the indecies zipped with the reversed of the selected.
        //for each indecie, set nums[indecie] to newnum.
        indecies
            .into_iter()
            .zip(selected.into_iter().rev())
            .for_each(|(indecie, newnum)| nums[indecie] = newnum);

        *current_pos += (*len + *skipsize) % numslenght;
        *skipsize += 1;
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
    println!("part 1: {}", solve(&mut nums_part1, &lenghts_part1, &mut 0, &mut 0));

    let mut current_pos = 0;
    let mut skipsize = 0;

    let mut nums_part2 = nums();
    let lenghts_part2 = parse_bytes(BYTESPUZZLE);
    (0..64).for_each(|_| { solve(&mut nums_part2, &lenghts_part2, &mut current_pos, &mut skipsize);});

    println!("part 2: {}", dense(&nums_part2));
}
