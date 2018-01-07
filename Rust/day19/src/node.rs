use libaoc::{Direction, Position};
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Pipe,
    Void,
    Turn,
    Line,
    Letter(char),
}

impl From<char> for Node {
    fn from(c: char) -> Node {
        match c {
            '|' => Node::Pipe,
            '+' => Node::Turn,
            ' ' => Node::Void,
            '-' => Node::Line,
            _ => Node::Letter(c),
        }
    }
}

#[derive(Debug)]
pub struct Walker {
    position: Position<usize>,
    direction: Direction,
    message: String,
    nodes: Vec<Vec<Node>>
}

impl Walker {
    ///A new Walker is initalized with just the x set to where you enter the maze.
    pub fn new(nodes: Vec<Vec<Node>>) -> Walker {
        let x = nodes[0]
            .iter()
            .position(|node| node == &Node::Pipe)
            .unwrap();
        Walker {
            position: Position::from((x, 0)),
            direction: Direction::Down,
            message: String::new(),
            nodes: nodes
        }
    }

    pub fn getstr(self) -> String {
        self.message
    }
    
    pub fn atvoidnode(&self) -> bool {
        let (x, y) = self.position.get_ref();
        self.nodes[*y][*x] == Node::Void
    }

    /// first walk, then check the node.
    /// if its a letter, push it to the message.
    /// if it's a turn, then turn!
    pub fn walk(&mut self) {
        self.position.change(&self.direction, 1);
        self.checknode()
    }

    pub fn checknode(&mut self) {
        let (x, y) = 
        {
            let (_x, _y) = self.position.get_ref();
            (*_x, *_y)
        };
        match self.nodes[y][x] {
            Node::Letter(c) => self.message.push(c),
            Node::Turn => self.turn(),
            _ => return,
        }
    }

    //return None if you can't even look to the right / down
    //return None if x + 1 or y + 1 equals Node::Void.
    fn node_at_pos(&self, s: &str) -> Option<()> {
        let (x, y) = self.position.get_ref();
        let (x, y) = (*x, *y);
        match s {
            "updown" => {
                if x + 1 >= self.nodes[0].len() || self.nodes[y][x + 1] == Node::Void {
                    None
                }
                else {
                    Some(())
                }
            }
            "leftright" => {
                if y > self.nodes.len() || self.nodes[y + 1][x] == Node::Void {
                    None
                }
                else {
                    Some(())
                }
            }
            _ => panic!()
        }
    }
    pub fn turn(&mut self) {
        match self.direction {
            Direction::Up | Direction::Down => {
                match self.node_at_pos("updown") {
                    Some(_) => self.direction = Direction::init_right(),
                    None => self.direction = Direction::init_left(),
                }
            },
            Direction::Left | Direction::Right => {
                match self.node_at_pos("leftright") {
                    Some(_) => self.direction = Direction::init_down(),
                    None => self.direction = Direction::init_up(),
                }
            },
            _ => return,
        }
    }
}