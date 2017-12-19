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
pub struct Coordinates {
    x: i64,
    y: i64,
    direction: Direction,
    message: String,
    nodes: Vec<Vec<Node>>
}

impl Coordinates {
    ///A new node is initalized with just the x set to where you enter the maze.
    pub fn new(nodes: Vec<Vec<Node>>) -> Coordinates {
        let x = nodes[0]
            .iter()
            .position(|node| node == &Node::Pipe)
            .unwrap();

        Coordinates {
            x: x as i64,
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
        self.nodes[self.y as usize][self.x as usize] == Node::Void
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
        match self.nodes[self.y as usize][self.x as usize] {
            Node::Letter(c) => self.message.push(c),
            Node::Turn => self.turn(),
            _ => return,
        }
    }

    //if we need to change, try to look in one way. If that succeeds, turn that way.
    //if it doesn't, turn the other.
    //also, checks if you even CAN look in a way, if you even't can't look in a way,
    //turn the other.
    pub fn turn(&mut self) {
        match self.direction {
            Direction::Up | Direction::Down => {
                
                //can't even look to the right!
                if (self.x + 1) >= self.nodes[0].len() as i64 {
                    self.direction = Direction::Left
                }
                //if it's not a void..., turn right.
                else if self.nodes[self.y as usize][(self.x + 1) as usize] != Node::Void {
                    self.direction = Direction::Right;
                }
                //it was a void! turn left
                else {
                    self.direction = Direction::Left;
                }
            },
            Direction::Left | Direction::Right => {
                //can't even look down!
                if (self.y + 1) >= self.nodes.len() as i64 {
                    self.direction = Direction::Up;
                }
                //if it's not a void..., turn down
                else if self.nodes[(self.y + 1) as usize][self.x as usize] != Node::Void {
                    self.direction = Direction::Down;
                }
                //it was a void! turn up!
                else {
                    self.direction = Direction::Up;
                }
            },
        }
    }
}
