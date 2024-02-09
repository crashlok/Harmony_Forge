use std::fmt::Debug;

use super::{Gen, Generator};
use crate::models::Models;
pub struct MultipleClosure<I, C: Fn(&Models) -> usize> {
    gen_list: Vec<Box<Gen<I>>>,
    indexing_closure: C,
}

impl<I, C: Fn(&Models) -> usize> MultipleClosure<I, C> {
    pub fn new(indexing_closure: C) -> Self {
        Self {
            gen_list: Vec::new(),
            indexing_closure,
        }
    }

    pub fn add_generator(mut self, gen: impl 'static + Generator<Item = I> + Send) -> Self {
        self.gen_list.push(Box::new(gen));
        self
    }
}

impl<I, C: Fn(&Models) -> usize> Generator for MultipleClosure<I, C> {
    type Item = I;
    fn gen(&mut self, gen_models: Models) -> (Self::Item, Models) {
        let index = (self.indexing_closure)(&gen_models) % self.gen_list.len();
        (*(self.gen_list[index])).gen(gen_models)
    }
}

/*
fn __add_arrays<const N: usize, T: Default + Copy + Debug>(mut input: Vec<[T; N]>) -> [T; N]
where
    T: std::ops::Add<T, Output = T>,
{
    dbg!(&input);
    dbg!(&input.len());

    return match input.len() {
        0 => [T::default(); N],
        1 => *input.first().unwrap(),
        _ => add_arrays({
            let input2 = input.clone();
            input.first_mut().unwrap() = input
                .first()
                .unwrap()
                .iter()
                .zip(input2.last().unwrap())
                .map(|(&a, &b)| a + b)
                .collect::<Vec<T>>()
                .try_into()
                .unwrap();
            input.pop();
            dbg!(&input);
            input
        }),
    };
}
*/
fn add_arrays<const N: usize, T: Default + Copy + Debug>(input: &Vec<[T; N]>) -> [T; N]
where
    T: std::ops::AddAssign<T>,
{
    let mut result = [T::default(); N];
    for i in 0..N {
        for array in input {
            result[i] += array[i];
        }
    }
    result
}

struct MultipleAdd<const N: usize, I> {
    generators: Vec<Box<Gen<[I; N]>>>,
}
impl<const N: usize, I> MultipleAdd<N, I> {}

impl<const N: usize, I> Generator for MultipleAdd<N, I>
where
    I: std::ops::AddAssign<I>,
    I: Default + Copy + Debug,
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
        (add_arrays(&results), new_models)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_arrays() {
        let a = [1, 2, 3, 4, 5];
        let o = [0, 0, 0, 0, 0];
        let b = [1, 1, 1, 1, 1];
        assert_eq!(add_arrays(&vec![a]), a);
        assert_eq!(add_arrays(&vec![a, a]), [2, 4, 6, 8, 10]);
        assert_eq!(add_arrays(&vec![a, a, a]), [3, 6, 9, 12, 15]);
        assert_eq!(add_arrays(&vec![a, o]), a);
        assert_eq!(
            add_arrays(&vec![a, a, b, a, o, o]),
            add_arrays(&vec![a, a, b, a])
        );
        assert_eq!(add_arrays(&vec![a, b]), [2, 3, 4, 5, 6]);
    }
}
