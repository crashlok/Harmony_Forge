use super::{multiple_notes_off, multiple_notes_on, Generator};
use crate::models::Models;
use midly::num::{u4, u7};
use nodi::Event;
pub struct EmptyPattern;

impl Generator for EmptyPattern {
    type Item = Vec<Event>;
    fn gen(&mut self, gen_models: Models) -> (Self::Item, Models) {
        (Vec::new(), gen_models)
    }
}

pub struct OnBeatPattern<N: Generator<Item = Vec<u7>>> {
    note_generator: N,
}

impl<N: Generator<Item = Vec<u7>>> OnBeatPattern<N> {
    pub fn new(note_generator: N) -> Self {
        Self { note_generator }
    }
}
impl<N: Generator<Item = Vec<u7>>> Generator for OnBeatPattern<N> {
    type Item = Vec<Event>;
    fn gen(&mut self, input_models: Models) -> (Self::Item, Models) {
        dbg!(input_models.time);
        if (input_models.time.get_rest_quarters() * 100.0).floor() == 0.0 {
            let mut gen_models: Models = input_models;
            let mut res = Vec::new();

            res.append(&mut multiple_notes_off(
                gen_models.playing.stop_all(),
                u7::new(100),
                u4::new(0),
            ));

            let (new_notes, end_models) = self.note_generator.gen(gen_models);

            res.append(&mut multiple_notes_on(new_notes, u7::new(100), u4::new(0)));

            return (res, end_models);
        }
        (Vec::new(), input_models)
    }
}
