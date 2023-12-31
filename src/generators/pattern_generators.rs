use super::{midi_massage_event, Generator};
use crate::models::Models;
use midly::num::{u4, u7};
use nodi::Event;
pub struct EmptyPattern;

impl Generator for EmptyPattern {
    type Item = Vec<Event>;
    fn gen(&mut self, _gen_models: &mut Models) -> Self::Item {
        Vec::new()
    }
}

pub struct OnBeatPattern<N: Generator<Item = Vec<u7>>> {
    note_generator: N,
}

impl<N: Generator<Item = Vec<u7>>> OnBeatPattern<N> {
    pub fn new(note_generator: N) -> Self {
        Self { note_generator }
    }
    fn stop_all(&self, gen_models: &mut Models) -> Vec<Event> {
        gen_models
            .playing
            .stop_all()
            .iter()
            .map(|n| {
                midi_massage_event(
                    midly::MidiMessage::NoteOff {
                        key: *n,
                        vel: u7::new(100),
                    },
                    u4::new(0),
                )
            })
            .collect()
    }
}
impl<N: Generator<Item = Vec<u7>>> Generator for OnBeatPattern<N> {
    type Item = Vec<Event>;
    fn gen(&mut self, gen_models: &mut Models) -> Self::Item {
        let mut res = Vec::new();
        dbg!(gen_models.time);
        if (gen_models.time.get_rest_quarters() * 100.0).floor() == 0.0 {
            res.append(&mut self.stop_all(gen_models));
            let n: u7 = self.note_generator.gen(gen_models)[0];
            gen_models.playing.started(n, gen_models.time);
            res.push(midi_massage_event(
                midly::MidiMessage::NoteOn {
                    key: n,
                    vel: u7::new(100),
                },
                u4::new(0),
            ))
        }
        res
    }
}
