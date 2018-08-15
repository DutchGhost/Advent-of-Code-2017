use std::ops::{Generator, GeneratorState};

/// A GeneratorAdaptor.
pub struct GeneratorAdaptor<G>(G);

impl <G>GeneratorAdaptor<G> {
    pub fn new(g: G) -> GeneratorAdaptor<G> {
        GeneratorAdaptor(g)
    }
}

impl <G, Y, R> Iterator for GeneratorAdaptor<G>
where G: Generator<Yield = Y, Return = R> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
          match self.0.resume() {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}