use super::{Gen, Generator};
use crate::models::Models;
pub struct MultipleClosure<I, C: Fn(&Models) -> usize> {
    generators: Vec<Box<Gen<I>>>,
    indexing_closure: C,
}

impl<I, C: Fn(&Models) -> usize> MultipleClosure<I, C> {
    pub fn new(indexing_closure: C, generators: Vec<Box<Gen<I>>>) -> Self {
        Self {
            generators,
            indexing_closure,
        }
    }
}

impl<I, C: Fn(&Models) -> usize> Generator for MultipleClosure<I, C> {
    type Item = I;
    fn gen(&mut self, gen_models: Models) -> (Self::Item, Models) {
        let index = (self.indexing_closure)(&gen_models) % self.generators.len();
        (*(self.generators[index])).gen(gen_models)
    }
}
