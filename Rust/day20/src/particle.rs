use prelude::*;

macro_rules! Struct3 {
    ($s:ident) => {

        #[derive(Debug, Eq, PartialEq, Clone)]
        struct $s {
            x: i64,
            y: i64,
            z: i64,
        }
        
        impl FromStr for $s {
            type Err = ();

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let (x, y, z) = to_nums(s.chars());
                Ok($s { x: x, y: y, z: z })
            }
        }
    };
}

macro_rules! add {
    ($s1:expr, $s2:expr) => {
        $s1.x += $s2.x;
        $s1.y += $s2.y;
        $s1.z += $s2.z;
    };
}

#[inline]
fn to_nums<I: Iterator<Item = char>>(iter: I) -> (i64, i64, i64) {
    let stringified = iter
        .filter(|c| c.is_digit(10) || c == &'-' || c == &',')
        .collect::<String>();

    let mut it = stringified.split(",");

    let n1 = it.next().unwrap().parse::<i64>().unwrap();
    let n2 = it.next().unwrap().parse::<i64>().unwrap();
    let n3 = it.next().unwrap().parse::<i64>().unwrap();

    (n1, n2, n3)
}

Struct3!(Position);
Struct3!(Velocity);
Struct3!(Acceleration);

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
    #[inline]
    fn update(&mut self) {
        add!(self.velocity, self.acceleration);
        add!(self.position, self.velocity);
    }

    #[inline]
    fn distance(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    #[inline]
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

    #[inline]
    pub fn closest(&self) -> usize {
        self.particles
            .iter()
            .enumerate()
            .min_by_key(|&(_, particle)| particle.distance())
            .unwrap()
            .0
    }

    #[inline]
    pub fn countparticles(&self) -> usize {
        self.particles.len()
    }
}
