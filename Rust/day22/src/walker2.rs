use std::mem;

#[derive(Clone, Debug)]
pub enum Node2 {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl From<char> for Node2 {
    fn from(c: char) -> Node2 {
        match c {
            '.' => Node2::Clean,
            '#' => Node2::Infected,
            _ => panic!("INVALID!"),
        }
    }
}

impl Node2 {
    fn modify(&mut self) {
        mem::replace(self, match self {
            &mut Node2::Clean => Node2::Weakened,
            &mut Node2::Weakened => Node2::Infected,
            &mut Node2::Infected => Node2::Flagged,
            &mut Node2::Flagged => Node2::Clean,
        });
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        mem::replace(self, match self {
            &mut Direction::Right => Direction::Down,
            &mut Direction::Down => Direction::Left,
            &mut Direction::Left => Direction::Up,
            &mut Direction::Up => Direction::Right,
        });
    }
    fn turn_left(&mut self) {
        mem::replace(self, match self {
            &mut Direction::Left => Direction::Down,
            &mut Direction::Down => Direction::Right,
            &mut Direction::Right => Direction::Up,
            &mut Direction::Up => Direction::Left,
        });
    }
    fn reverse(&mut self) {
        mem::replace(self, match self {
            &mut Direction::Left => Direction::Right,
            &mut Direction::Down => Direction::Up,
            &mut Direction::Right => Direction::Left,
            &mut Direction::Up => Direction::Down,
        });
    }
}

#[derive(Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new() -> Pos {
        Pos {
            x: 12,
            y: 12,
        }
    }
}

pub struct Walker2 {
    pos: Pos,
    grid: Vec<Vec<Node2>>,
    direction: Direction,
}

impl Walker2 {
    pub fn new(node2: Vec<Vec<Node2>>) -> Walker2 {
        let p = Pos::new();

        let dir = match node2[p.y][p.x] {
            Node2::Clean => Direction::Up,
            Node2::Infected => Direction::Down,
            _ => panic!("impossible"),
        };
        
        Walker2 {
            pos: p,
            grid: node2,
            direction: dir,
        }
    }

    fn expand(&mut self) {
        if self.pos.x == 0 {
            for mut row in self.grid.iter_mut() {
                row.insert(0, Node2::Clean);
            }
            self.pos.x += 1;
        }
        if self.pos.x == self.grid[0].len() {
            for mut row in self.grid.iter_mut() {
                row.push(Node2::Clean);
            }
        }
        if self.pos.y == 0 {
            let len = self.grid[0].len();
            let insert = vec![Node2::Clean; len];
            self.grid.insert(0, insert);
            self.pos.y += 1;
        }

        if self.pos.y == self.grid.len() {
            let len = self.grid[0].len();
            let insert = vec![Node2::Clean; len];
            self.grid.push(insert);
        }
    }
}
//liftimes :). Sad this has te be outside the impl block... :(, else there are 2 &mut's
fn node2_at_pos<'m, 's: 'm>(node2: &'s mut [Vec<Node2>], pos: &Pos) -> Option<&'m mut Node2> {
    node2.get_mut(pos.y).unwrap().get_mut(pos.x)
}

impl Iterator for Walker2 {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.expand();
        let infected;
        match node2_at_pos(&mut self.grid, &self.pos) {
            Some(n) => {
                //change the position if needed. If weakened, set infected to 1.
                match n {
                    &mut Node2::Clean => {
                        self.direction.turn_left();
                        infected = 0;
                    },
                    &mut Node2::Weakened => {
                        infected = 1;
                    },
                    &mut Node2::Infected => {
                        self.direction.turn_right();
                        infected = 0;
                    }
                    &mut Node2::Flagged => {
                        self.direction.reverse();
                        infected = 0;
                    }
                }
                //lastly, update the node!
                n.modify();
            }
            None => panic!("something went horrible terribly wrong.")
        };

        match self.direction {
            Direction::Up => self.pos.y -= 1,
            Direction::Down => self.pos.y += 1,
            Direction::Right => self.pos.x += 1,
            Direction::Left => self.pos.x -= 1,
        };
        Some(infected)
    }
}