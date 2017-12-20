use prelude::*;

//takes in any Iterator I with items of char, and any filter F.
//F will be filtering '<', '>', '=' and the letter 'v', 'a' or 'p' out of the Iterator.
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

fn filter(ch: char) -> impl Fn(&char) -> bool {
    move |c| !(c == &'<' || c == &'>' || c == &'=' || c == &ch)
}

#[derive(Eq, PartialEq, Clone)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Position {
    type Err = ();
    fn from_str(s: &str) -> Result<Position, Self::Err> {
        let (x, y, z) = to_nums(s.chars(), filter('p'));
        Ok(Position { x: x, y: y, z: z })
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Velocity {
    type Err = ();
    fn from_str(s: &str) -> Result<Velocity, Self::Err> {
        let (x, y, z) = to_nums(s.chars(), filter('v'));
        Ok(Velocity { x: x, y: y, z: z })
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Acceleration {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Acceleration {
    type Err = ();
    fn from_str(s: &str) -> Result<Acceleration, Self::Err> {
        let (x, y, z) = to_nums(s.chars(), filter('a'));
        Ok(Acceleration { x: x, y: y, z: z })
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Particle {
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
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

    fn collide(&self, other: &Particle) -> bool {
        self.position == other.position
    }
}

pub struct GPU {
    particles: Vec<Particle>,
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

    //includes the current particle in the collision-vector...
    //however, we check for v.len() > 1!
    pub fn collisionupdate(&mut self) {
        self.update();

        let collided = self.particles
            .iter()
            .enumerate()
            .map(|(idx, p1)| {
                self.particles[idx..]
                    .iter()
                    .filter(|p2| p1.collide(&p2))
                    .collect::<Vec<_>>()
            })
            .filter(|v| v.len() > 1)
            .flat_map(|v| v)
            .cloned()
            .collect::<Vec<_>>();

        self.particles.retain(|p| !collided.contains(&p));
    }
    pub fn closest(&self) -> usize {
        self.particles
            .iter()
            .enumerate()
            .min_by_key(|&(_, particle)| particle.distance())
            .unwrap()
            .0
    }
    pub fn countparticles(&self) -> usize {
        self.particles.len()
    }
}
