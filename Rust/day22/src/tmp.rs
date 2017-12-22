use std::mem;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl From<char> for Node {
    
    #[inline]
    fn from(c: char) -> Node {
        match c {
            '.' => Node::Clean,
            '#' => Node::Infected,
            _ => panic!("Invalid nodetype.")
        }
    }
}

struct Pos {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Pos {
    
    #[inline]
    fn from((n1, n2): (usize, usize)) -> Pos {
        Pos {x: n1, y: n2}
    }
}

struct Grid {
    grid: Vec<Vec<Node>>
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid{ grid: s.lines().map(|line| line.chars().map(|c| Node::from(c)).collect()).collect()})
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    
    #[inline]
    fn turn_right(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    #[inline]
    fn turn_left(self) -> Self {
        match self {
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }

    #[inline]
    fn reverse(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
        }
    }
}
struct Walker {
    grid: Grid,
    pos: Pos,
    facing: Direction,
}

