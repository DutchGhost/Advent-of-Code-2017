const PUZZLE: &'static str = include_str!("Input.txt");
const BYTESPUZZLE: &[u8] = include_bytes!("Input.txt");

mod p1solver;
mod p2solver;
use p1solver::part1;
use p2solver::part2;


fn main() {
    let lenghts = part1::parse(PUZZLE);
    let mut nums = part1::nums();
    println!("{}", part1::solve(nums, lenghts));

    let lenths_part_2 = part2::parse(BYTESPUZZLE);
    let mut nums_part2 = part2::nums();
    println!("MAIN NUMS {:?}", nums_part2);
    part2::solve(&mut nums_part2, lenths_part_2);
    let xored = part2::xor(&nums_part2);
    for chunk in xored {
        println!("chunk {} xor {}", chunk, format!("{:x}", chunk));
    }


}
