use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Port {
    pub front: u8,
    pub back: u8,
}

impl FromStr for Port {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(r"/");

        Ok(Port { front: it.next().unwrap().parse().unwrap(), back: it.next().unwrap().parse().unwrap()})
    }
}

impl Port {
    pub fn connectable(&self, other: &Port) -> bool {
        self.back == other.front || self.back == other.back || self.front == other.front
    }

    fn zeroport(&self) -> bool {
        self.front == 0 || self.back == 0
    }

    pub fn value(&self) -> u8 {
        self.front + self.back
    }
}
