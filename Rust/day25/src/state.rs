use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Debug)]
pub struct Block {
    state: State,
    onzero: (i32, Direction, State),
    onone: (i32, Direction, State),
}

fn parse<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> (i32, Direction, State) {
    let write = match iter.next().unwrap().split_whitespace().last() {
        Some("0.") => 0,
        Some("1.") => 1,
        _ => panic!("I dont know how to parse this!"),
    };

    //the direction.
    let direction = match iter.next().unwrap().split_whitespace().last() {
        Some("left.") => Direction::Left,
        Some("right.") => Direction::Right,
        _ => panic!("I dont know how to parse this!"),
    };

    let newstate = match iter.next().unwrap().split_whitespace().last() {
        Some("A.") => State::A,
        Some("B.") => State::B,
        Some("C.") => State::C,
        Some("D.") => State::D,
        Some("E.") => State::E,
        Some("F.") => State::F,
        _ => panic!("cant parse this"),
    };

    (write, direction, newstate)
}
impl Block {
    pub fn new() -> Block {
        Block {
            state: State::A,
            onzero: (0, Direction::Left, State::A),
            onone: (0, Direction::Left, State::A),
        }
    }
}
impl FromStr for Block {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Block, Self::Err> {
        let mut iter = s.lines().filter(|line| line != &"");
        //the state
        let state = match iter.next().unwrap().split_whitespace().last() {
            Some("A:") => State::A,
            Some("B:") => State::B,
            Some("C:") => State::C,
            Some("D:") => State::D,
            Some("E:") => State::E,
            Some("F:") => State::F,
            _ => panic!("Could not parse"),
        };
        iter.next();
        //the last char

        let onezero = parse(&mut iter);
        iter.next();
        let onone = parse(&mut iter);
        Ok(Block {
            state: state,
            onzero: onezero,
            onone: onone,
        })
    }
}

#[derive(Debug)]
pub struct CPU {
    state: State,
    tape: VecDeque<i32>,
    cursor: usize,
    instruction: HashMap<(State, i32), (i32, Direction, State)>,
}

impl CPU {
    pub fn new(transitions: [Block; 6]) -> CPU {
        let mut map = HashMap::new();

        for transition in transitions.into_iter() {
            map.insert((transition.state, 0i32), transition.onzero);
            map.insert((transition.state, 1i32), transition.onone);
        }

        let mut deque = VecDeque::with_capacity(50_000);
        deque.push_front(0);

        CPU {
            state: State::A,
            tape: deque,
            cursor: 0,
            instruction: map,
        }
    }
    pub fn run(&mut self) {
        let current_value = *self.tape.get(self.cursor).unwrap();
        let &(newvalue, direction, newstate) =
            self.instruction.get(&(self.state, current_value)).unwrap();

        self.state = newstate;
        if let Some(n) = self.tape.get_mut(self.cursor) {
            *n = newvalue;
        };

        match direction {
            Direction::Right => self.move_right(),
            Direction::Left => self.move_left(),
        };
    }

    #[inline]
    fn move_left(&mut self) {
        if self.cursor == 0 {
            self.tape.push_front(0);
        } else {
            self.cursor -= 1;
        }
    }

    #[inline]
    fn move_right(&mut self) {
        if self.cursor == self.tape.len() - 1 {
            self.tape.push_back(0);
        }
        self.cursor += 1;
    }

    #[inline]
    pub fn count_one(self) -> usize {
        self.tape.into_iter().filter(|n| n == &1).count()
    }
}
