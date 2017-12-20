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

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Walker {
    x: usize,
    y: usize,
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
            x: x,
            y: 0,
            direction: Direction::Down,
            message: String::new(),
            nodes: nodes
        }
    }

    pub fn getstr(self) -> String {
        self.message
    }
    
    pub fn voidnode(&self) -> bool {
        self.nodes[self.y][self.x] == Node::Void
    }

    ///first walk, then check the node.
    /// if its a letter, push it to the message.
    /// if it's a turn, then turn!
    pub fn walk(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
        self.checknode()
    }

    pub fn checknode(&mut self) {
        match self.nodes[self.y][self.x] {
            Node::Letter(c) => self.message.push(c),
            Node::Turn => self.turn(),
            _ => return,
        }
    }

    //return None if you can't even look to the right / down
    //return None if x + 1 or y + 1 equals Node::Void.
    fn node_at_pos(&self, s: &str) -> Option<()> {
        match s {
            "updown" => {
                if self.x + 1 >= self.nodes[0].len() || self.nodes[self.y][self.x + 1] == Node::Void {
                    None
                }
                else {
                    Some(())
                }
            }
            "leftright" => {
                if self.y > self.nodes.len() || self.nodes[self.y + 1][self.x] == Node::Void {
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
                    Some(_) => self.direction = Direction::Right,
                    None => self.direction = Direction::Left,
                }
            },
            Direction::Left | Direction::Right => {
                match self.node_at_pos("leftright") {
                    Some(_) => self.direction = Direction::Down,
                    None => self.direction = Direction::Up,
                }
            },
        }
    }
}