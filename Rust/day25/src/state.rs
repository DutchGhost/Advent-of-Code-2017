use std::collections::VecDeque;

enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
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
    leftright: bool,
}

fn move_left(tape: &mut VecDeque<i32>, cursor: &mut usize) {
        if *cursor == 0 {
            tape.push_front(0);
        }
        else {
            *cursor -= 1;
        }
    }

    //len - 1 or len??
    fn move_right(tape: &mut VecDeque<i32>, cursor: &mut usize) {
        if *cursor == tape.len() -1 {
            tape.push_back(0);
        }
        *cursor += 1;
    }


//Probably do something with a HashMap? <(State, value), (leftright, newState)> ????
impl CPU {
    pub fn new() -> CPU {
        let mut deque = VecDeque::with_capacity(50_000);
        for _ in 0..6 {
            deque.push_front(0);
        }
        CPU {
            state: State::A,
            tape: deque,
            cursor: 3,
            leftright: false,
        }
    }
    pub fn run(&mut self) {
        match self.tape.get_mut(self.cursor) {
            Some(n) => {
                match self.state {
                    State::A => {
                        if *n == 0 {
                            *n = 1;
                            self.leftright = true;
                            self.state = State::B;
                        }
                        else {
                            *n = 0;
                            self.leftright = false;
                            self.state = State::F;
                        }
                    }
                    State::B => {
                        if *n == 0 {
                            *n = 0;
                            self.leftright = true;
                            self.state = State::C;
                        }
                        else {
                            *n = 0;
                            self.leftright = true;
                            self.state = State::D;
                        }
                    }
                    State::C => {
                        if *n == 0 {
                            *n = 1;
                            self.leftright = false;
                            self.state = State::D;
                        }
                        else {
                            *n = 1;
                            self.leftright = true;
                            self.state = State::E;
                        }
                    }
                    State::D => {
                        if *n == 0 {
                            *n = 0;
                            self.leftright = false;
                            self.state = State::E;
                        }
                        else {
                            *n = 0;
                            self.leftright = false;
                            self.state = State::D;
                        }
                    }
                    State::E => {
                        if *n == 0 {
                            *n = 0;
                            self.leftright = true;
                            self.state = State::A;
                        }
                        else {
                            *n = 1;
                            self.leftright = true;
                            self.state = State::C;
                        }
                    }
                    State::F => {
                        if *n == 0 {
                            *n = 1;
                            self.leftright = false;
                            self.state = State::A;
                        }
                        else {
                            *n = 1;
                            self.leftright = true;
                            self.state = State::A;
                        }
                    }
                }
            }
            None => panic!("something whent horribly terribly wrong!"),
        }
        
        if self.leftright { move_right(&mut self.tape, &mut self.cursor); } else { move_left(&mut self.tape, &mut self.cursor); }
    }
    pub fn count_one(&self) -> usize {
        self.tape.iter().filter(|n| n == &&1).count()
    }
}