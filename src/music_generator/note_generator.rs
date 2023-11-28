use super::NoteGenerator;
use crate::note::{octave, Scale};
use midly::num::u7;
use rand::distributions::{self, Distribution};
use std::ops::Range;

pub struct NearNotes {
    scale: Vec<i32>,
    lastnotes: Vec<i32>,
}

impl NearNotes {
    pub fn new(scale: Scale, octave_range: Range<i32>) -> Self {
        Self {
            scale: scale.as_midi_notes_with_octave_range(octave_range),
            lastnotes: Vec::new(),
        }
    }

    fn gen_dist(&self) -> Vec<f64> {
        let mut result = Vec::new();
        for x in 0..self.scale.len() {
            result.push(f64::floor(
                crate::probability_density_function(x as f64, self.scale.len() as f64 / 2.0, 3.5)
                    * 12.0_f64.powf(2.0),
            ))
        }
        result
    }
}

impl NoteGenerator for NearNotes {
    fn gen(&mut self) -> Vec<u7> {
        let dist = distributions::WeightedIndex::new(0, self.scale.);

        vec![u7::new(
            self.scale[dist.sample(&mut rand::thread_rng())]
                .try_into()
                .unwrap(),
        )]
    }
}

pub struct RandomNotes {
    scale: Vec<i32>,
}

impl RandomNotes {
    pub fn new(scale: Scale, octave_range: Range<i32>) -> Self {
        Self {
            scale: scale.as_midi_notes_with_octave_range(octave_range),
        }
    }
}

impl NoteGenerator for RandomNotes {
    fn gen(&mut self) -> Vec<u7> {
        let dist = distributions::Uniform::<usize>::new(0, self.scale.len());
        vec![u7::new(
            self.scale[dist.sample(&mut rand::thread_rng())]
                .try_into()
                .unwrap(),
        )]
    }
}
