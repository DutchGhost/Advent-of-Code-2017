use genitter::GeneratorAdaptor;

use std::ops::Generator;
use std::mem;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

pub struct Spiral {
    point: Point,
    direction: Direction,
}

impl Spiral {
    pub fn new() -> Spiral {
        Spiral {
            point: Point {x: 0, y: 0},
            direction: Direction::new(),
        }
    }

    fn spiral_gen<'g, 'a: 'g>(&'a mut self) -> impl Generator<Yield = (i64, Point), Return = ()> + 'g {
        let mut n = 1;
        let mut number_of_moves = 1;
        move || {
            loop {
                for _ in 0..2 {
                    for must_move in 0..number_of_moves {
                        yield (n, self.point);
                        self.spiral();
                        n += 1;

                        if must_move == number_of_moves - 1 {
                            self.direction.moved();
                        }
                    }
                }
                number_of_moves += 1;
            }
        }
    }

    fn spiral(&mut self) {
        match self.direction {
            Direction::Up => {
                self.point.y += 1;
            },
            Direction::Down => {
                 self.point.y -= 1;
            }
            Direction::Left => {
                self.point.x -= 1;
            },
            Direction::Right => {
                self.point.x += 1;
            }
        }
    }

    pub fn part1(&mut self, input: i64) -> i64 {
        let spiral_iterator = GeneratorAdaptor::new(self.spiral_gen());
        spiral_iterator
            .filter(|&(n, _)| n == input)
            .map(|(_, point)| point.x.abs() + point.y.abs())
            .next()
            .unwrap()
    }
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
