use std::str::FromStr;

fn to_nums<I, F>(iter: I, filter: F) -> (i64, i64, i64)
where
    I: Iterator<Item = char>,
    F: Fn(&char) -> bool,
{
    let stringified = iter.filter(filter).collect::<String>();
    let mut it = stringified.split(",");
    let n1 = it.next().unwrap().parse::<i64>().unwrap();
    let n2 = it.next().unwrap().parse::<i64>().unwrap();
    let n3 = it.next().unwrap().parse::<i64>().unwrap();
    (n1, n2, n3)
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Acceleration {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
pub struct Particle {
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}

pub struct GPU {
    particles: Vec<Particle>,
}

impl FromStr for Position {
    type Err = ();
    fn from_str(s: &str) -> Result<Position, Self::Err> {
        let (x, y, z) = to_nums(s.chars(), |c| {
            !(c == &'p' || c == &'=' || c == &'<' || c == &'>')
        });
        Ok(Position { x: x, y: y, z: z })
    }
}

impl FromStr for Velocity {
    type Err = ();
    fn from_str(s: &str) -> Result<Velocity, Self::Err> {
        let (x, y, z) = to_nums(s.chars(), |c| {
            !(c == &'v' || c == &'=' || c == &'<' || c == &'>')
        });
        Ok(Velocity { x: x, y: y, z: z })
    }
}

impl FromStr for Acceleration {
    type Err = ();
    fn from_str(s: &str) -> Result<Acceleration, Self::Err> {
        let (x, y, z) = to_nums(s.chars(), |c| {
            !(c == &'a' || c == &'=' || c == &'<' || c == &'>')
        });
        Ok(Acceleration { x: x, y: y, z: z })
    }
}

impl FromStr for Particle {
    type Err = ();
    fn from_str(s: &str) -> Result<Particle, Self::Err> {
        let mut pos_vel_acc = s.split(", ");

        let p = pos_vel_acc.next().unwrap();
        let pos = Position::from_str(p).unwrap();

        let v = pos_vel_acc.next().unwrap();
        let vel = Velocity::from_str(v).unwrap();

        let a = pos_vel_acc.next().unwrap();
        let acc = Acceleration::from_str(a).unwrap();

        Ok(Particle {
            position: pos,
            velocity: vel,
            acceleration: acc,
        })
    }
}

impl Particle {
    ///Increase the X velocity by the X acceleration.
    ///Increase the Y velocity by the Y acceleration.
    ///Increase the Z velocity by the Z acceleration.
    ///Increase the X position by the X velocity.
    ///Increase the Y position by the Y velocity.
    ///Increase the Z position by the Z velocity.

    fn update(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn distance(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }
}

impl FromStr for GPU {
    type Err = ();
    fn from_str(s: &str) -> Result<GPU, Self::Err> {
        Ok(GPU {
            particles: s.lines()
                .map(|line| Particle::from_str(line).unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

impl GPU {
    pub fn update(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.update()
        }
    }

    pub fn closest(&self) -> usize {
        self.particles.iter().enumerate().min_by_key(|&(idx, particle)| particle.distance()).unwrap().0
    }
}