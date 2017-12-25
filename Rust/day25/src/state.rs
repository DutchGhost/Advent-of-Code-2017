use std::collections::{VecDeque, HashMap};

#[derive(Debug, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F
}

#[derive(Debug)]
pub struct CPU {
    state: State,
    tape: VecDeque<i32>,
    cursor: usize,
    instruction: HashMap<(State, i32), (i32, Direction, State)>
}

impl CPU {
    pub fn new() -> CPU {
        let mut map = HashMap::new();
        map.insert((State::A, 0), (1, Direction::Right, State::B));
        map.insert((State::A, 1), (0, Direction::Left, State::F));
        map.insert((State::B, 0), (0, Direction::Right, State::C));
        map.insert((State::B, 1), (0, Direction::Right, State::D));
        map.insert((State::C, 0), (1, Direction::Left, State::D));
        map.insert((State::C, 1), (1, Direction::Right, State::E));
        map.insert((State::D, 0), (0, Direction::Left, State::E));
        map.insert((State::D, 1), (0, Direction::Left, State::D));
        map.insert((State::E, 0), (0, Direction::Right, State::A));
        map.insert((State::E, 1), (1, Direction::Right, State::C));
        map.insert((State::F, 0), (1, Direction::Left, State::A));
        map.insert((State::F, 1), (1, Direction::Right, State::A));

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
        let &(newvalue, direction, newstate) = self.instruction.get(&(self.state, current_value)).unwrap();
        
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
        }
        else {
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