use super::NoteGenerator;
use crate::note::{octave, Scale};
use midly::num::u7;
use rand::distributions;
use std::ops::Range;

pub struct RandomNotes {
    scale: Vec<i32>,
}

impl RandomNotes {
    pub fn new(scale: Scale, octave_range: Range<i32>) -> Self {
        let mut scale_resut: Vec<i32> = vec![];
        let scale: Vec<i32> = scale.as_midi_notes();
        for o in octave_range {
            scale_resut.append(&mut scale.iter().map(|n| octave(*n, o)).collect::<Vec<i32>>())
        }
    }
}
impl NoteGenerator for RandomNotes {
    fn gen(&mut self) -> u7 {
        dbg!(&self.scale);
        let dist = distributions::Uniform::<usize>::new(0, self.scale.len());
        u7::new(
            self.scale[dist.sample(&mut rand::thread_rng())]
                .try_into()
                .unwrap(),
        )
    }
}
