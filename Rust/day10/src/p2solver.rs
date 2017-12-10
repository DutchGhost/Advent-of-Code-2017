pub struct part2;

impl part2 {
    pub fn parse(input: &'static [u8]) -> Vec<u8> {
        let test = "1,2,4".chars().map(|c| c as u8).collect::<Vec<_>>();
        let mut bytes: Vec<u8> = test.into_iter().map(|b| b).collect();
        bytes.extend(&[17, 31, 73, 47, 23]);
        println!("{:?}", bytes);
        bytes
    }

    pub fn nums() -> Vec<u8> {
        (0u8..256u8).collect()
    }

    pub fn solve(nums: &mut Vec<u8>, lenghts: Vec<u8>) {
        println!("NUMS: {:?}", nums);
        let NUMSLENGHT = nums.len();
        let mut current_pos: i64 = 0;
        let mut skipsize: i64 = 0;
        for i in 0..64 {    
            for len in lenghts.iter() {
                //the selected items from nums. wraps around. also gets the index.
                let mut selected = nums
                    .iter()
                    .enumerate()
                    .cycle()
                    .skip(current_pos as usize)
                    .take(*len as usize)
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
                println!("LENNN{} {} {}", len, skipsize, NUMSLENGHT);
                current_pos += ((*len as i64 + skipsize) % NUMSLENGHT as i64);
                skipsize += 1;
            }
        }
    }

    pub fn xor(nums: &Vec<u8>) -> Vec<u8> {
        nums.chunks(16).map(|chunk| chunk.iter().fold(0, |mut n, acc| n ^ acc)).collect()
    }
}