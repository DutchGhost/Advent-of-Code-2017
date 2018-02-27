use genitter::GeneratorAdaptor;

use std::ops::Generator;

use libaoc::movement::{Position, Direction, ManhattenDst};

//turn left, on dir::up,
pub struct Spiral {
    point: Position<i64>,
    direction: Direction,
    storage: Vec<(i64, Position<i64>)>
}

impl Spiral {
    //Init a spiral with the coordinate [0, 0] set to a value of 1.
    pub fn new() -> Spiral {
        let vec = vec![(1, Position::new(0, 0))];
        Spiral {
            point: Position::new(0, 0),
            direction: Direction::init_right(),
            storage: vec
        }
    }
    pub fn reset(&mut self) {
        self.point = Position::new(0, 0);
        self.direction = Direction::init_right();
    }

    fn spiral<'g, 'a: 'g, F>(&'a mut self, next_value: F) -> impl Generator<Yield = (i64, Position<i64>), Return = ()> + 'g
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
                        self.point.rev_change(&self.direction, 1);

                        //get the new value.
                        value = next_value(self, value);

                        //after the last step, we must change our direction.
                        if must_chage_direction == number_of_steps - 1 {
                            self.direction = self.direction.turn_left();
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

        let value = self.storage
            .iter()
            .filter(|&&(_, coordinate)| self.point.is_adjecent(&coordinate))
            .map(|&(value, _)| value)
            .sum();

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
            .map(|(_, point)| point.manhattendst())
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
