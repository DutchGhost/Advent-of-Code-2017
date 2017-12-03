use genitter::GeneratorAdaptor;

use std::ops::Generator;
use std::mem;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Point {
    x: i64,
    y: i64,
}

pub struct Spiral {
    point: Point,
    direction: Direction,
}

pub struct SpecialSpiral {
    point: Point,
    direction: Direction,
    storage: Vec<(i64, Point)>
}

impl Spiral {
    pub fn new() -> Spiral {
        Spiral {
            point: Point {x: 0, y: 0},
            direction: Direction::new(),
        }
    }

    fn spiral_gen<'g, 'a: 'g>(&'a mut self) -> impl Generator<Yield = (i64, Point), Return = ()> + 'g {
        let mut value = 1;
        let mut number_of_moves = 1;
        move || {
            loop {
                //in a spiral, there is continuously 2 times the same number of steps,
                //and after those 2 times, the number of steps is incremented with 1.
                for _ in 0..2 {
                    for must_move in 0..number_of_moves {
                        
                        //yield the current number, with it's position,
                        //so we can calculate the absolute distance to the first field.
                        yield (value, Point {x: self.point.x, y: self.point.y});
                        
                        //now 'spiral', (aka set the new postion), and increment the numer
                        spiral(&self.direction, &mut self.point);
                        value += 1;

                        //after the last step in a given direction, change the direction.
                        if must_move == number_of_moves - 1 {
                            self.direction.change();
                        }
                    }
                }
                number_of_moves += 1;
            }
        }
    }

    pub fn part1(&mut self, input: i64) -> i64 {
        let spiral_iterator = GeneratorAdaptor::new(self.spiral_gen());
        spiral_iterator
            .filter(|&(value, _)| value == input)
            .map(|(_, point)| point.x.abs() + point.y.abs())
            .next()
            .unwrap()
    }
}
impl Direction {
    fn new() -> Direction {
        Direction::Right
    }

    fn change(&mut self) {
        mem::replace(self, match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        });
    }
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
                //in a spiral, there is continuously 2 times the same number of steps,
                //and after those 2 times, the number of steps is incremented with 1.
                for _ in 0..2 {
                    for must_move in 0..number_of_moves {

                        //get the 'adjacent' fields, and sum them up.
                        let value = self.adjacents().iter().sum();

                        //store the value of the field, and the coordinates,
                        //the field might become an adjacent field for another field in the future.
                        self.storage.push((value, Point {x: self.point.x, y: self.point.y}));

                        //yield it
                        yield value;

                        //now 'spiral' (aka, set the new postion)
                        spiral(&self.direction, &mut self.point);
                        
                        //after the last step in a given direction, change the direction.
                        if must_move == number_of_moves - 1 {
                            self.direction.change();
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
            .filter(|value| value > &input)
            .next()
            .unwrap()
    }

    fn adjacents(&mut self) -> Vec<i64> {
        let valids = [(0, 1), (1, 0), (1, 1)];

        let mut results: Vec<i64> = Vec::new();
        for &(ref value, ref p) in self.storage.iter() {
            let diff_x = (p.x - self.point.x).abs();
            let diff_y = (p.y - self.point.y).abs();

            // get all the neighboors
            if valids.contains(&(diff_x, diff_y)) {
                results.push(value.clone());
            }
        }

        //if there are no neighboors, the value of the field becomes just 1
        if results.len() == 0 {
            results.push(1i64);
        }
        results
    }
}

fn spiral(direction: &Direction, point: &mut Point) {
    match direction {
        &Direction::Up => {
            point.y += 1;
        },
        &Direction::Down => {
             point.y -= 1;
        }
        &Direction::Left => {
            point.x -= 1;
        },
        &Direction::Right => {
            point.x += 1;
        }
    }
}
