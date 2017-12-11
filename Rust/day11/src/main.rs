const PUZZLE: &'static str = include_str!("Input.txt");
fn solve(input: &str) -> (i64, i64) {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut max = 0;
    for ins in input.split(",") {
        match ins {
            "n" => {
                y -= 1;
            },
            "nw" => {
                x -= 1;
            }
            "sw" => {
                x -= 1;
                y += 1;
            }
            "s" => {
                y += 1;
            }
            "se" => {
                x += 1;
            }
            "ne" => {
                x += 1;
                y -= 1;
            }
            _ => panic!("INVALID"),
        }
        if y < 0 {
            max = std::cmp::max(max, x);
        }
        else {
            max = std::cmp::max(max, x + y);
        }
    }
    if x.abs() * -1 == x {
        println!("x is neg");
        return (x.abs() - y, max)
    }
    if y.abs() * -1 == y {
        println!("y is neg");
        println!(" x plus y is {}", x + y);
        return (x, max)
    }
    if x.abs() == y.abs() {
        return (0, max)
    }
    if y < 0 {
        return (x, max)
    }
    if x < 0 {
        return (y, max)
    }
    return (x + y, max)
}

fn main() {
    let (part1, part2) = solve(PUZZLE);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
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