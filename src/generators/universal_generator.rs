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

fn add_arrays<const N: usize, T: Default + Copy>(mut input: Vec<[T; N]>) -> [T; N]
where
    for<'a> &'a mut T: std::ops::AddAssign<&'a T>,
{
    return match input.len() {
        0 => [T::default(); N],
        1 => *input.first().unwrap(),
        _ => add_arrays({
            let input2 = input.clone();
            input
                .first_mut()
                .unwrap()
                .iter_mut()
                .zip(input2.last().unwrap())
                .for_each(|(mut a, b)| a += b);
            input.pop();
            input
        }),
    };
}
struct MultipleAdd<const N: usize, I> {
    generators: Vec<Box<Gen<[I; N]>>>,
}
impl<const N: usize, I> MultipleAdd<N, I> {}

impl<const N: usize, I> Generator for MultipleAdd<N, I>
where
    for<'a> &'a mut I: std::ops::AddAssign<&'a I>,
    I: Default + Copy,
{
    type Item = [I; N];
    fn gen(&mut self, gen_models: Models) -> (Self::Item, Models) {
        let (results, new_models) =
            self.generators
                .iter_mut()
                .fold((Vec::new(), gen_models), |(mut res, models), gen| {
                    let (re, models) = gen.gen(models);
                    res.push(re);
                    (res, models)
                });
        (add_arrays(results), new_models)
    }
}
