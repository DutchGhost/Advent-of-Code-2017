#[macro_use]
extern crate itertools;
use itertools::Itertools;

const PUZZLE: &'static str = include_str!("Input.txt");

use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Pixel {
    image: Vec<Vec<char>>,
}

impl Pixel {
    pub fn start() -> Self {
        Self {
            image: vec![
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ],
        }
    }

    fn print(&self) {
        for item in self.image.iter() {
            for c in item {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

impl FromStr for Pixel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut image = Vec::with_capacity(3);

        for substr in s.split(r"/") {
            image.push(substr.chars().collect::<Vec<_>>());
        }

        Ok(Self { image: image })
    }
}

fn parse_rules() -> Vec<(Pixel, Pixel)> {
    let mut v = Vec::new();

    for line in PUZZLE.lines() {
        let mut iter = line.split(" => ");

        let from = Pixel::from_str(iter.next().unwrap()).unwrap();
        let to = Pixel::from_str(iter.next().unwrap()).unwrap();

        v.push((from, to))
    }

    v
}

fn main() {
    let mut pix = Pixel::start();

    let parsed = parse_rules();

    for item in parsed {
        let cloned = pix.clone();
        let mut v = Vec::new();
        for i in iproduct!(
            cloned.image[0].iter(),
            cloned.image[1].iter(),
            cloned.image[2].iter()
        ) {
            v.push(i);

            if v.len() == 3 {
                let newpixel = Pixel {
                    image: vec![
                        vec![*v[0].0, *v[0].1, *v[0].2],
                        vec![*v[1].0, *v[1].1, *v[1].2],
                        vec![*v[2].0, *v[2].1, *v[2].2],
                    ],
                };

                v.clear();

                if newpixel.image == item.0.image {
                    pix.image = item.1.image.clone();
                    pix.print();
                }
            }
        }
    }
}
