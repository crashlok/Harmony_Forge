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
    channel: u4,
}

impl<N: Generator<Item = Vec<u7>>> OnBeatPattern<N> {
    pub fn new(note_generator: N, channel: u8) -> Self {
        Self {
            note_generator,
            channel: u4::new(channel),
        }
    }
}
impl<N: Generator<Item = Vec<u7>>> Generator for OnBeatPattern<N> {
    type Item = Vec<Event>;
    fn gen(&mut self, input_models: Models) -> (Self::Item, Models) {
        dbg!(&input_models.playing);
        if (input_models.time.get_rest_quarters() * 100.0).floor() == 0.0 {
            let mut gen_models: Models = input_models.clone();
            let mut res = Vec::new();

            res.append(&mut multiple_notes_off(
                gen_models.playing.stop_all(self.channel),
                u7::new(100),
                self.channel,
            ));

            let (new_notes, mut end_models) = self.note_generator.gen(gen_models);

            res.append(&mut multiple_notes_on(
                end_models
                    .playing
                    .start_multiple(new_notes, input_models.time, self.channel),
                u7::new(100),
                self.channel,
            ));

            return (res, end_models);
        }
        (Vec::new(), input_models)
    }
}
