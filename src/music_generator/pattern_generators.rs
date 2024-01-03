use super::{midi_massage_event, NoteGenerator, PatternGenerator};
use midly::num::{u4, u7};

pub struct EmptyPattern;

impl PatternGenerator for EmptyPattern {
    fn gen(&mut self) -> Vec<nodi::Event> {
        Vec::new()
    }
}

pub struct OnBeatPattern<N: NoteGenerator> {
    playing: Vec<(u7, u32)>,
    note_generator: N,
}

impl<N: NoteGenerator + 'static> OnBeatPattern<N> {
    pub fn new(note_generator: N) -> Self {
        Self {
            playing: Vec::new(),
            note_generator,
        }
    }
}
impl<N: NoteGenerator> PatternGenerator for OnBeatPattern<N> {
    fn gen(&mut self) -> Vec<nodi::Event> {
        self.playing = self.playing.iter().map(|(n, c)| (*n, c + 1)).collect();

        if self.playing.is_empty() {
            let n: u7 = self.note_generator.gen()[0];
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