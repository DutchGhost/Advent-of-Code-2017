const PUZZLE: &'static str = include_str!("Input.txt");

fn parse(input: &str) -> Vec<i64> {
    input.split(",").map(|word| word.parse().expect("Failed to parse")).collect()
    
}

//0 to 255 is 0..256!
fn nums() -> Vec<i64> {
    (0..256).collect()
}

fn solve(mut nums: Vec<i64>, lenghts: Vec<i64>) -> i64 {
    let NUMSLENGHT = nums.len();
    let mut current_pos = 0;
    let mut skipsize = 0;
    for len in lenghts {
        //the selected items from nums. wraps around. also gets the index.
        let mut selected = nums
            .iter()
            .enumerate()
            .cycle()
            .skip(current_pos)
            .take(len as usize)
            .map(|(idx, n)| (idx, *n)).collect::<Vec<_>>();
        
        //this is really nice to have. A list of indecies that should be changed.
        let mut indecies = selected.iter().map(|&(idx, _)| idx).collect::<Vec<_>>();
        
        //reverse the selected items
        selected.reverse();

        //make it an iterator. We dont need the index anymore.
        let mut selecteds = selected.into_iter().map(|(_, n)| n);
        
        //for each indecie, get nums[indecie], and set it to selecteds.next().unwrap()
        indecies
            .into_iter()
            .for_each(|indecie| nums[indecie] = selecteds.next().unwrap());

        current_pos += ((len + skipsize) as usize) % NUMSLENGHT;
        skipsize += 1;
    }
    nums[0] * nums[1]
}

fn main() {
    let lenghts = parse(PUZZLE);
    let mut nums = nums();
    println!("{}", solve(nums, lenghts));
   
}
