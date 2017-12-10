const PUZZLE: &'static str = include_str!("Input.txt");
const BYTESPUZZLE: &[u8] = include_bytes!("Input.txt");
//-------------------------------------------------------------------------------
//TODO:
//  -   Make solve() be sharable over part 1 and part 2.
//      Maybe by calling it 64 times in main or something,
//      Also it should be able to take both a Vector<i64> and a Vector<u8>.
//      And we only really need 1 function to generate the numbers.
//      Also, maybe PUZZLE can be converted into a bytearray...??
//-------------------------------------------------------------------------------

mod p1solver;
mod p2solver;
use p1solver::Part1;
use p2solver::Part2;


fn main() {
    let lenghts = Part1::parse(PUZZLE);
    let nums = Part1::nums();
    println!("part 1: {}", Part1::solve(nums, lenghts));

    let lenghts_part_2 = Part2::parse(BYTESPUZZLE);
    let mut nums_part2 = Part2::nums();
    println!("{}", Part2::solve(&mut nums_part2, lenghts_part_2));
}
