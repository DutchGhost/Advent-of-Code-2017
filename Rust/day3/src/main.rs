#![feature(conservative_impl_trait)]
#![feature(generator_trait, generators)]

use std::ops::{GeneratorState, Generator};
use std::mem;

use std::io::{self, Write};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new() -> Direction {
        Direction::Right
    }

    fn moved(&mut self) {
        mem::replace(self, match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        });
    }
}

fn spiral(n: &mut i64, x: &mut i64, y: &mut i64, direction: &Direction) {
    match direction {
        &Direction::Up => {
            *n += 1;
            *y += 1;
        },
        &Direction::Down => {
            *n += 1;
            *y -= 1;
        }
        &Direction::Left => {
            *n += 1;
            *x -= 1;
        },
        &Direction::Right => {
            *n += 1;
            *x += 1;
        }
    }
}

fn spiral_generator(stop: i64) -> impl Generator<Yield = (i64, i64, i64), Return = ()> {
    move || {
        let mut n = 1;
        let mut number_of_moves = 1;
        let mut x = 0;
        let mut y = 0;
        let mut direction = Direction::new();
        'outer: loop {
            for _ in 0..2 {
                for must_change in 0..number_of_moves {
                    yield (n, x, y);
                    if n == stop {
                        break 'outer;
                    }
                    spiral(&mut n, &mut x, &mut y, &direction);
                    
                    //we only change direction after the last move in a given direction
                    if must_change == number_of_moves - 1 {
                        direction.moved();
                    }
                }
            }
            number_of_moves += 1;
        }
    }
}

fn main() {
    let mut gen = spiral_generator(361527);
    let mut summed = 0;

    while let GeneratorState::Yielded((n, x, y)) = gen.resume() {
        summed = x.abs() + y.abs();
    }

    println!("{}", summed);
}
