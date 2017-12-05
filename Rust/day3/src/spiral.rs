use genitter::GeneratorAdaptor;

use std::ops::Generator;
use std::mem;

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

    fn change(&mut self) {
        mem::replace(self, match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        });
    }
}

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn update(&mut self, direction: &Direction) {
        match direction {
            &Direction::Up => {
                self.y += 1;
            },
            &Direction::Down => {
                self.y -= 1;
            }
            &Direction::Left => {
                self.x -= 1;
            },
            &Direction::Right => {
                self.x += 1;
            }
        }
    }
}

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
    pub fn reset(&mut self) {
        self.point = Point {x: 0, y: 0};
        self.direction = Direction::new();
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
                        let to_yield = self.point.clone();
                        yield (value, to_yield);

                        //update the coordinates.
                        self.point.update(&mut self.direction);

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

    //returns the sum of the value's of the current point's adjecent point's.
    //also inserts the current value with the current coordinate to the storage.
    fn sum_of_adjecents(&mut self) -> i64 {    
        let valids = [(0, 1), (1, 0), (1, 1)];
        
        let value = self.storage
            .iter()
            .map(|&(value, ref p)| (value, ((p.x - self.point.x).abs(), (p.y - self.point.y).abs())))
            .filter(|&(_, coordinate)| valids.contains(&coordinate))
            .map(|(value, _)| value)
            .sum();
        self.storage.push((value, self.point.clone()));
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
            spiral.sum_of_adjecents()
        });

        let mut spiralizer = GeneratorAdaptor::new(spiral);

        spiralizer
            .find(|&(value, _)| value > input)
            .map(|(value, _)| value)
            .unwrap()
    }
}
