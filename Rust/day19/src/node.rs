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
    pub x: i64,
    pub y: i64,
    direction: Direction,
    pub message: String,
}

impl Coordinates {
    ///A new node is initalized with just the x set to where you enter the maze.
    pub fn new(nodes: &[Vec<Node>]) -> Coordinates {
        let x = nodes[0]
            .iter()
            .position(|node| node == &Node::Pipe)
            .unwrap();

        Coordinates {
            x: x as i64,
            y: 0,
            direction: Direction::Down,
            message: String::new(),
        }
    }

    pub fn walk(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn checknode(&mut self, nodes: &[Vec<Node>]) {
        match nodes[self.y as usize][self.x as usize] {
            Node::Letter(c) => self.message.push(c),
            Node::Turn => self.change_direction(nodes),
            _ => return,
        }
    }

    //if we need to change, try to look in one way. If that succeeds, turn that way.
    //if it doesn't, turn the other.
    //also, checks if you even CAN look in a way, if you even't can't look in a way,
    //turn the other.
    pub fn change_direction(&mut self, nodes: &[Vec<Node>]) {
        match self.direction {
            Direction::Up | Direction::Down => {
                
                //can't even look to the right!
                if (self.x + 1) >= nodes[0].len() as i64 {
                    self.direction = Direction::Left
                }
                //if it's not a void..., turn right.
                else if nodes[self.y as usize][(self.x + 1) as usize] != Node::Void {
                    self.direction = Direction::Right;
                }
                //it was a void! turn left
                else {
                    self.direction = Direction::Left;
                }
            },
            Direction::Left | Direction::Right => {
                //can't even look down!
                if (self.y + 1) >= nodes.len() as i64 {
                    self.direction = Direction::Up;
                }
                //if it's not a void..., turn down
                else if nodes[(self.y + 1) as usize][self.x as usize] != Node::Void {
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
