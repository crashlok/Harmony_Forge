use super::Generator;
use crate::{display_distributtion,models::Models, note::Scale};
use find_all::FindAll;
use midly::num::u7;
use rand::distributions::{self, Distribution};
use std::ops::Range;

pub struct PatternNotes {
    scale: Vec<u7>,
    lastnotes: Vec<usize>,
}

impl PatternNotes {
    pub fn new(scale: Scale, octave_range: Range<i8>) -> Self {
        Self {
            scale: scale.as_midi_notes_with_octave_range(octave_range),
            lastnotes: Vec::new(),
        }
    }

    fn gen_dist(&self) -> Vec<f64> {
        let mut result = vec![0.2; self.scale.len()];
        let last_note: usize = *self.lastnotes.last().unwrap_or(&(self.scale.len() / 2));
        result[last_note] /= 5.0;
        if let Some(indexes) = self.lastnotes.iter().find_all(|&&x| x == last_note) {
            indexes
                .iter()
                .for_each(|i| result[self.lastnotes[i + 1]] *= 1.2)
        }
        result
    }
}

impl Generator for PatternNotes {
    type Item = Vec<u7>;

    fn gen(&mut self, _gen_models: &mut Models) -> Self::Item {
        let raw_dist = self.gen_dist();
        display_distributtion(&raw_dist);
        let dist = distributions::WeightedIndex::new(raw_dist).unwrap();

        let n: usize = dist.sample(&mut rand::thread_rng());

        println!(" {} \n", n);
        self.lastnotes.push(n);
        vec![u7::new(self.scale[n].try_into().unwrap())]
    }
}

pub struct NearNotes {
    scale: Vec<u7>,
    lastnotes: Vec<usize>,
}

impl NearNotes {
    pub fn new(scale: Scale, octave_range: Range<i8>) -> Self {
        Self {
            scale: scale.as_midi_notes_with_octave_range(octave_range),
            lastnotes: Vec::new(),
        }
    }

    fn gen_dist(&self) -> Vec<f64> {
        let mut result = Vec::new();
        let last_note: usize = *self.lastnotes.last().unwrap_or(&(self.scale.len() / 2));
        for x in 0..self.scale.len() {
            result.push(crate::probability_density_function(
                x as f64,
                last_note as f64,
                2.0,
            ))
        }
        result[last_note] /= 5.0;
        result
    }
}

impl Generator for NearNotes {
    type Item = Vec<u7>;

    fn gen(&mut self, _gen_models: &mut Models) -> Self::Item {
        let raw_dist = self.gen_dist();
        display_distributtion(&raw_dist);
        let dist = distributions::WeightedIndex::new(raw_dist).unwrap();

        let n: usize = dist.sample(&mut rand::thread_rng());

        println!(" {} \n", n);
        self.lastnotes.push(n);
        vec![u7::new(self.scale[n].try_into().unwrap())]
    }
}

pub struct RandomNotes {
    scale: Vec<u7>,
}

impl RandomNotes {
    pub fn new(scale: Scale, octave_range: Range<i8>) -> Self {
        Self {
            scale: scale.as_midi_notes_with_octave_range(octave_range),
        }
    }
}

impl Generator for RandomNotes {
    type Item = Vec<u7>;

    fn gen(&mut self, _gen_models: &mut Models) -> Self::Item {
        let dist = distributions::Uniform::<usize>::new(0, self.scale.len());
        vec![u7::new(
            self.scale[dist.sample(&mut rand::thread_rng())]
                .try_into()
                .unwrap(),
        )]
    }
}

pub struct NotesDependingBar {
    notes: Vec<Vec<u7>>,
}

impl NotesDependingBar {
    pub fn new(notes: Vec<Vec<u7>>) -> Self {
        Self { notes }
    }
}

impl Generator for NotesDependingBar {
    type Item = Vec<u7>;

    fn gen(&mut self, gen_models: &mut Models) -> Self::Item {
        self.notes[(gen_models.time.get_bars() % self.notes.len() as i32) as usize].clone()
    }
}
