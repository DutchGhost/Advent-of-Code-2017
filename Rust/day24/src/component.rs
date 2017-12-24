use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Component {
    front: i64,
    back: i64,
}

impl FromStr for Component {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(r"/");

        Ok(Component { front: it.next().unwrap().parse().unwrap(), back: it.next().unwrap().parse().unwrap()})
    }
}

impl Component {
    pub fn connectable(&self, other: &Component) -> bool {
        self.back == other.front
    }

    pub fn value(&self) -> i64 {
        self.front + self.back
    }
}
