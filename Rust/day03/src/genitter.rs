use std::ops::{Generator, GeneratorState};

pub struct GeneratorAdaptor<G, Y, R>(G)
where G:
    Generator<Yield = Y, Return = R>;

impl <G, Y, R>GeneratorAdaptor<G, Y, R>
where G: Generator<Yield = Y, Return = R>
{

    #[inline]
    pub fn new(g: G) -> GeneratorAdaptor<G, Y, R> {
        GeneratorAdaptor(g)
    }

    #[inline]
    pub fn with_box(g: Box<G>) -> GeneratorAdaptor<Box<G>, Y, R> {
        GeneratorAdaptor(g)
    }

    #[inline]
    pub fn generator(self) -> G {
        self.0
    }
}


impl <G, Y, R> Iterator for GeneratorAdaptor<G, Y, R>
where G: Generator<Yield = Y, Return = R>
{
    type Item = G::Yield;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
          match self.0.resume() {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}

impl <G, Y, R>::std::convert::AsRef<G> for GeneratorAdaptor<G, Y, R>
where G: Generator<Yield = Y, Return = R>
{
    fn as_ref<'a>(&'a self) -> &G {
        &(*self).0
    }
}
