use std::collections::HashMap;
const PUZZLE: &'static str = include_str!("Input.txt");

struct Hex<'look, 'split> {
    map: HashMap<&'look str, (i64, i64)>,
    iter: std::str::Split<'split, &'split str>,
}

impl<'look, 'split, 's> Hex<'look, 'split>
where 's: 'split
{
    fn new(input: &'s str) -> Hex<'look, 'split> {
        let mut map = HashMap::new();
        map.insert("n", (0, 1));
        map.insert("nw", (-1, 1));
        map.insert("sw", (-1, 0));
        map.insert("s", (0, -1));
        map.insert("se", (1, -1));
        map.insert("ne", (1, 0));

        Hex {
            map: map,
            iter: input.split(","),
        }
    }
}

impl<'look, 'split> Iterator for Hex<'look, 'split> {
    type Item = (i64, i64);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(ins) => self.map.get(&ins).cloned(),
            None => None,
        }
    }
}

fn dist(x: i64, y: i64) -> i64 {
    let z = -(x + y);
    (x.abs()).max(y.abs()).max(z.abs())
}
fn solve(input: &str) -> (i64, i64, i64) {
    let hex = Hex::new(input);
    hex.map(|(x, y)| (x, y)).fold((0, 0, 0), |(x, y, max), (newx, newy)| {
        (x + newx, y + newy, std::cmp::max(max, dist(x, y)))
    })
}

fn main() {
    let (x, y, part2) = solve(PUZZLE);
    println!("part 1: {}", dist(x, y));
    println!("part 2: {}", part2)

}

mod tests {
    use super::*;
    #[test]
    fn test_ne() {
        assert_eq!(solve("ne,ne,ne"), (3, 3));
    }

    #[test]
    fn test_ne_ne_sw_sw() {
        assert_eq!(solve("ne,ne,sw,sw"), (0, 2));
    }
    #[test]
    fn test_ne_ne_s_s() {
        assert_eq!(solve("ne,ne,s,s"), (2, 2));
    }
}
