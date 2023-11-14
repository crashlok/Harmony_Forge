use super::{midi_massage_event, MelodyGen};
use crate::note::{octave, Scale};
use midly::num::{u4, u7};
use std::ops::Range;

pub struct Random {
    playing: Vec<u7>,
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
            playing: vec![],
            scale: scale_resut,
        }
    }
}

impl MelodyGen for Random {
    fn gen(&mut self) -> Vec<nodi::Event> {
        if self.playing.len() == 0 {
            let n: u7 = u7::new(1);
            self.playing.push(n);
            return vec![midi_massage_event(
                midly::MidiMessage::NoteOn {
                    key: n,
                    vel: u7::new(1),
                },
                u4::new(3),
            )];
        } else if rand::random() {
            let n = self.playing[0];
            self.playing.pop();
            //return vec![midi_massage_event(
            //    midly::MidiMessage::NoteOff {
            //        key: n,
            //        vel: u7::new(1),
            //    },
            //    u4::new(1),
            //)];
        }
        Vec::new()
    }
}
