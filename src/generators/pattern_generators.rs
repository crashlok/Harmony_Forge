use super::{midi_massage_event, GenModels, Generator};
use midly::num::{u4, u7};
use nodi::Event;

pub struct EmptyPattern;

impl Generator for EmptyPattern {
    type Item = Vec<Event>;
    fn gen(&mut self, _gen_models: GenModels) -> Self::Item {
        Vec::new()
    }
}

pub struct OnBeatPattern<N: Generator<Item = Vec<u7>>> {
    playing: Vec<(u7, u32)>,
    note_generator: N,
}

impl<N: Generator<Item = Vec<u7>>> OnBeatPattern<N> {
    pub fn new(note_generator: N) -> Self {
        Self {
            playing: Vec::new(),
            note_generator,
        }
    }
}
impl<N: Generator<Item = Vec<u7>>> Generator for OnBeatPattern<N> {
    type Item = Vec<Event>;
    fn gen(&mut self, gen_models: GenModels) -> Self::Item {
        self.playing = self.playing.iter().map(|(n, c)| (*n, c + 1)).collect();

        if self.playing.is_empty() {
            let n: u7 = self.note_generator.gen(gen_models)[0];
            self.playing.push((n, 0));
            return vec![midi_massage_event(
                midly::MidiMessage::NoteOn {
                    key: n,
                    vel: u7::new(100),
                },
                u4::new(0),
            )];
        } else if self.playing[0].1 >= 2500 {
            let n = self.playing[0].0;
            self.playing.pop();
            return vec![midi_massage_event(
                midly::MidiMessage::NoteOff {
                    key: n,
                    vel: u7::new(100),
                },
                u4::new(0),
            )];
        }
        Vec::new()
    }
}
