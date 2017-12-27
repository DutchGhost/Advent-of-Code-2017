use std::mem;

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

    fn change(&mut self) {
        mem::replace(self, match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        });
    }
}

#[derive(Debug, Clone, Copy)]
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

impl Into<(i64, i64)> for Point {
    fn into(self) -> (i64, i64) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Spiral {
    direction: Direction,
    point: Point,
    _needed: i64,
    must_change: i64,
    number_of_steps: i64,
}

impl Spiral {
    #[inline]
    pub fn new() -> Spiral {
        Spiral {
            direction: Direction::Right,
            point: Point {x: 0, y: 0},
            _needed: 0,
            must_change: 0,
            number_of_steps: 1,
        }
    }
}


//FIX ME
impl Iterator for Spiral {
    type Item = (i64, i64);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.point.clone().into();
        
        self.point.update(&self.direction);
        
        if self._needed % 2 == 1  {
            self.number_of_steps += 1;
        }
            self._needed += 1;

        if self.must_change == self.number_of_steps - 1 {
            self.direction.change();
            self.must_change = 0;
        }
        else {
            self.must_change += 1;
        }
        println!("{:?}", self);
        return Some(ret)

    }
}