pub struct Part2;

impl Part2 {
    pub fn parse(input: &'static [u8]) -> Vec<u8> {
        let mut bytes: Vec<u8> = input.into_iter().map(|b| *b).collect();
        bytes.extend(&[17, 31, 73, 47, 23]);
        bytes
    }

    pub fn nums() -> Vec<u8> {
        (0..).take(256).collect()
    }

    pub fn solve(nums: &mut Vec<u8>, lenghts: Vec<u8>) {
        let numslenght = nums.len();
        let mut current_pos: usize = 0;
        let mut skipsize: usize = 0;
        for _ in 0..64 {    
            for len in lenghts.iter() {
                //the selected items from nums. wraps around. also gets the index.
                let (indecies, selected): (Vec<usize>, Vec<u8>) = nums
                    .iter()
                    .enumerate()
                    .cycle()
                    .skip(current_pos)
                    .take(*len as usize)
                    .map(|(idx, n)| (idx, n))
                    .unzip();
             
                //make it an iterator, and reverse it.
                let mut selecteds = selected.into_iter().rev();
            
                //for each indecie, get nums[indecie], and set it to newnum
                indecies
                    .into_iter()
                    .zip(selecteds)
                    .for_each(|(indecie, newnum)| nums[indecie] = newnum);

                current_pos += ((*len as usize + skipsize) as usize) % numslenght;
                skipsize += 1;
            }
        }
    }

    pub fn xor(nums: &Vec<u8>) -> Vec<u8> {
        nums.chunks(16).map(|chunk| chunk.iter().fold(0, |n, acc| n ^ acc)).collect()
    }
}