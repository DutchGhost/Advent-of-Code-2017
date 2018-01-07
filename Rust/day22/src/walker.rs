use std::str::FromStr;
use libaoc::{Direction, Position, ToVec};

#[derive(Clone)]
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
            _ => panic!("Invalid NodeType.")
        }
    }
}

struct Grid {
    grid: Vec<Vec<Node>>
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid{ grid: s.lines().map(|line| line.chars().convert()).collect()})
    }
}

impl Grid {
    
    #[inline]
    fn node_at_pos<'m, 's: 'm>(&'s mut self, pos: &Position<usize>) -> Option<&'m mut Node> {
        let (x, y) = pos.get_ref();
        self.grid.get_mut(*y).and_then(|row| row.get_mut(*x))
    }

    #[inline]
    fn extend_left(&mut self) {
        for mut row in self.grid.iter_mut() {
            row.insert(0, Node::Clean);
        }
    }

    #[inline]
    fn extend_right(&mut self) {
        for row in self.grid.iter_mut() {
            row.push(Node::Clean);
        }
    }

    #[inline]
    fn extend_top(&mut self) {
        let len = self.grid[0].len();
        self.grid.insert(0, vec![Node::Clean; len])
    }

    #[inline]
    fn extend_bottem(&mut self) {
        let len = self.grid[0].len();
        self.grid.push(vec![Node::Clean; len])
    }
}

pub enum Part {
    Part1,
    Part2,
}

pub struct Walker {
    grid: Grid,
    pos: Position<usize>,
    facing: Direction,
    part: Part,
}

impl Walker {
    pub fn new(s: &str, part: Part) -> Walker {
        let grid = Grid::from_str(s).unwrap();

        let middle = grid.grid.len() / 2;

        Walker {
            grid: grid,
            pos: Position::from((middle, middle)),
            facing: Direction::init_init(),
            part: part,
        }
    }

    fn diagnostics(&mut self) -> i32 {
        match self.grid.node_at_pos(&self.pos) {
            Some(n) => {
                match n {
                    &mut Node::Clean => {
                        self.facing = self.facing.turn_left();
                        *n = Node::Infected;
                        1
                    },
                    &mut Node::Infected => {
                        self.facing = self.facing.turn_right();
                        *n = Node::Clean;
                        0
                    }
                    _ => panic!("cant happen on part 1!")
                }
            },
            None => panic!("Something went terribly horribly wrong with part 1!"),
        }
    }

    fn advanced_diagnostics(&mut self) -> i32 {
        match self.grid.node_at_pos(&self.pos) {
            Some(n) => {
                match n {
                    &mut Node::Clean => {
                        self.facing = self.facing.turn_left();
                        *n = Node::Weakened;
                        0
                    },
                    &mut Node::Weakened => {
                        *n = Node::Infected;
                        1
                    }
                    &mut Node::Infected => {
                        self.facing = self.facing.turn_right();
                        *n = Node::Flagged;
                        0
                    }
                    &mut Node::Flagged => {
                        self.facing = self.facing.reverse();
                        *n = Node::Clean;
                        0
                    }
                }
            },
            None => panic!("Something went terribly horribly wrong with part 2!"),
        }
    }
}

impl Iterator for Walker {
    type Item = i32;

    #[inline]
    fn next(&mut self) -> Option<i32> {
        let (x, y) = {
            let (_x, _y) = self.pos.get_ref();
            (*_x, *_y)
        };
        if x == 0 {
            self.grid.extend_left();
            self.pos.increment_x(1);
        }
        if x == self.grid.grid[0].len() {
            self.grid.extend_right();
        }
        if y == 0 {
            self.grid.extend_top();
            self.pos.increment_y(1);
        }
        if y == self.grid.grid.len() {
            self.grid.extend_bottem();
        }
        
        let infected = match self.part {
            Part::Part1 => Some(self.diagnostics()),
            Part::Part2 => Some(self.advanced_diagnostics()),
        };

        self.pos.change(&self.facing, 1);
        return infected;
    }
}