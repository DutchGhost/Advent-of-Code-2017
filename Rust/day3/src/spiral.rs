use genitter::GeneratorAdaptor;

use std::ops::Generator;
use std::mem;

#[derive(Clone)]
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
    storage: Vec<(i64, Point)>
}

impl Spiral {
    //Init a spiral with the coordinate [0, 0] set to a value of 1.
    pub fn new() -> Spiral {
        let vec = vec![(1, Point {x: 0, y: 0})];
        Spiral {
            point: Point {x: 0, y: 0},
            direction: Direction::new(),
            storage: vec
        }
    }

    fn spiral<'g, 'a: 'g, F>(&'a mut self, next_value: F) -> impl Generator<Yield = (i64, Point), Return = ()> + 'g
    where
        F: Fn(&mut Self, i64) -> i64 + 'g,
    {
        let mut value = 1;
        let mut number_of_steps = 1;

        move || {
            loop {
                for _ in 0..2 {
                    for must_chage_direction in 0..number_of_steps {
                        
                        //yield the value directly.
                        yield (value, self.point);

                        //update the coordinates.
                        self.direction.step(&mut self.point);

                        //get the new value.
                        value = next_value(self, value);

                        //after the last step, we must change our direction.
                        if must_chage_direction == number_of_steps - 1 {
                            self.direction.change();
                        }
                    }
                }

                //after we've done half a 'layer', increment the steps we must do.
                number_of_steps += 1;
            }
        }
    }

    fn adjacents(&mut self) -> i64 {    
        let valids = [(0, 1), (1, 0), (1, 1)];
        
        self.storage
            .iter()
            .map(|&(value, ref p)| (value, ((p.x - self.point.x).abs(), (p.y - self.point.y).abs())))
            .filter(|&(value, coordinate)| valids.contains(&coordinate))
            .map(|(value, _)| value)
            .sum()
    }

    fn squaresum(&mut self) -> i64 {
        let value = self.adjacents();
        self.storage.push((value, self.point));

        value
    }

    //Get new values from the stream until we've found a point with a value that equals the input.
    //then return the distance from that point to (0, 0).
    pub fn part1(&mut self, input: i64) -> i64 {
        let spiral = self.spiral(|_, n| n + 1);

        let mut spiralizer = GeneratorAdaptor::new(spiral);
        
        spiralizer
            .find(|&(value, _)| value == input)
            .map(|(_, point)| point.x.abs() + point.y.abs())
            .unwrap()
    }

    //Get new values from the stream until the value is bigger than the input.
    //this time the value of a coordinate is the sum of the value's of all of it's neighboors.
    //return the first value that is bigger than the input.
    pub fn part2(&mut self, input: i64) -> i64 {
        let spiral = self.spiral(|ref mut spiral, _| {
            spiral.squaresum()
        });

        let mut spiralizer = GeneratorAdaptor::new(spiral);

        spiralizer
            .find(|&(value, _)| value > input)
            .map(|(value, _)| value)
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

    fn step(&self, point: &mut Point) {
        match self {
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
}