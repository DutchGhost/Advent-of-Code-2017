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
        self.nodes[*(self.position.y_val())][*(self.position.x_val())] == Node::Void
    }

    ///first walk, then check the node.
    /// if its a letter, push it to the message.
    /// if it's a turn, then turn!
    pub fn walk(&mut self) {
        self.position.change(&self.direction, 1);
        self.checknode()
    }

    pub fn checknode(&mut self) {
        match self.nodes[*(self.position.y_val())][*(self.position.x_val())] {
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
                if *self.position.x_val() + 1 >= self.nodes[0].len() || self.nodes[*(self.position.y_val())][*(self.position.x_val()) + 1] == Node::Void {
                    None
                }
                else {
                    Some(())
                }
            }
            "leftright" => {
                if *self.position.y_val() > self.nodes.len() || self.nodes[*(self.position.y_val()) + 1][*(self.position.x_val())] == Node::Void {
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