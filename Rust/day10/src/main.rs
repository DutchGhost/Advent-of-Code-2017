const PUZZLE: &'static str = include_str!("Input.txt");
const BYTESPUZZLE: &[u8] = include_bytes!("Input.txt");

mod p1solver;
mod p2solver;
use p1solver::Part1;
use p2solver::Part2;


fn main() {
    let lenghts = Part1::parse(PUZZLE);
    let nums = Part1::nums();
    println!("part 1: {}", Part1::solve(nums, lenghts));

    let lenths_part_2 = Part2::parse(BYTESPUZZLE);
    let mut nums_part2 = Part2::nums();
    println!("{}", Part2::solve(&mut nums_part2, lenths_part_2));
}
