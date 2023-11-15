use super::{midi_massage_event, MelodyGen};
use crate::note::{octave, Scale};
use midly::num::{u4, u7};
use std::ops::Range;

pub struct Random {
    playing: Vec<(u7, u32)>,
    scale: Vec<i32>,
}

impl Random {
    pub fn new(scale: Scale, octave_range: Range<i32>) -> Random {
        let mut scale_resut: Vec<i32> = vec![];
        let scale: Vec<i32> = scale.as_midi_notes();
        for o in octave_range {
            scale_resut.append(&mut scale.iter().map(|n| octave(*n, o)).collect::<Vec<i32>>())
        }

        Self {
            playing: Vec::new(),
            scale: scale_resut,
        }
    }
}

impl MelodyGen for Random {
    fn gen(&mut self) -> Vec<nodi::Event> {
        self.playing = self.playing.iter().map(|(n, c)| (*n, c + 1)).collect();

        if self.playing.is_empty() {
            let n: u7 = u7::new(60);
            self.playing.push((n, 0));
            return vec![midi_massage_event(
                midly::MidiMessage::NoteOn {
                    key: n,
                    vel: u7::new(100),
                },
                u4::new(0),
            )];
        } else if rand::random() && self.playing[0].1 >= 10000 {
            println!("{}", self.playing[0].1);
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
