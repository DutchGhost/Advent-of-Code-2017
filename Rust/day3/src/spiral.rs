use genitter::GeneratorAdaptor;

use std::ops::Generator;
use std::mem;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct SpecialSpiral {
    point: Point,
    direction: Direction,
    storage: Vec<(i64, Point)>
}

impl SpecialSpiral {
    pub fn new() -> SpecialSpiral {
        SpecialSpiral {
            point: Point{x: 0, y: 0},
            direction: Direction::new(),
            storage: Vec::new(),
        }
    }

    fn special_spiral_generator<'g, 'a: 'g>(&'a mut self) -> impl Generator<Yield = i64, Return = ()> + 'g {
        move || {
            let mut number_of_moves = 1;
            loop {
                for _ in 0..2 {
                    for must_move in 0..number_of_moves {
                        let to_yield = self.adjecents().iter().sum();
                        self.storage.push((to_yield, self.point));
                        yield to_yield;
                        self.spiral();
                        if must_move == number_of_moves - 1 {
                            self.direction.moved();
                        }
                    }
                }
                number_of_moves += 1;
            }
        }
    }
    
    pub fn part2(&mut self, input: i64) -> i64 {
        let special_spiralizer = GeneratorAdaptor::new(self.special_spiral_generator());

        special_spiralizer
            .filter(|n| n > &input)
            .next()
            .unwrap()
    }

    fn adjecents(&mut self) -> Vec<i64> {
        let valids = [(0, 1), (1, 0), (1, 1)];

        let mut results: Vec<i64> = Vec::new();
        for &(n, p) in self.storage.iter() {
            let diff_x = (p.x - self.point.x).abs();
            let diff_y = (p.y - self.point.y).abs();

            // get all the neighboors
            if valids.contains(&(diff_x, diff_y)) {
                results.push(n);
            }
        }

        if results.len() == 0 {
            results.push(1i64);
        }
        results
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
}