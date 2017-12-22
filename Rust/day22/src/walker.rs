use std::mem;

#[derive(Debug, Clone)]
pub enum Node {
    Infected,
    Clean,
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        match c {
            '.' => Node::Clean,
            '#' => Node::Infected,
            _ => panic!("I don't know this kind of node")
        }
    }
}

#[derive(Debug)]
pub enum Direction {
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

pub struct Walker {
    pub pos: Pos,
    grid: Vec<Vec<Node>>,
    direction: Direction,
}

//maybe change the direction for infected??
impl Walker {
    pub fn new(nodes: Vec<Vec<Node>>) -> Walker {
        let p = Pos::new();

        let dir = match nodes[p.y][p.x] {
            Node::Clean => Direction::Up,
            Node::Infected => Direction::Down
        };
        
        Walker {
            pos: p,
            grid: nodes,
            direction: dir,
        }
    }

    fn expand(&mut self) {
        if self.pos.x == 0 {
            for mut row in self.grid.iter_mut() {
                row.insert(0, Node::Clean);
            }
            self.pos.x += 1;
        }
        if self.pos.x == self.grid[0].len() {
            for mut row in self.grid.iter_mut() {
                row.push(Node::Clean);
            }
        }
        if self.pos.y == 0 {
            let len = self.grid[0].len();
            let insert = vec![Node::Clean; len];
            self.grid.insert(0, insert);
            self.pos.y += 1;
        }

        if self.pos.y == self.grid.len() {
            let len = self.grid[0].len();
            let insert = vec![Node::Clean; len];
            self.grid.push(insert);
        }
    }
}
//liftimes :). Sad this has te be outside the impl block... :(, else there are 2 &mut's
fn node_at_pos<'m, 's: 'm>(nodes: &'s mut Vec<Vec<Node>>, pos: &Pos) -> Option<&'m mut Node> {
    nodes.get_mut(pos.y).unwrap().get_mut(pos.x)
}

impl Iterator for Walker {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.expand();
        
        
        let infected = match node_at_pos(&mut self.grid, &self.pos) {
            Some(n) => {
                match n {
                    &mut Node::Clean => {
                        self.direction.turn_left();
                        *n = Node::Infected;
                        1
                    }
                    &mut Node::Infected => {
                        self.direction.turn_right();
                        *n = Node::Clean;
                        0
                    }
                }
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