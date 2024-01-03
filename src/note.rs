use midly::num::u7;
use std::ops::Range;
pub mod chords;
mod scales;

#[derive(Clone, Debug, Copy)]
pub enum Step {
    Minor(usize),
    Major(usize),
    Normal(usize),
}
#[derive(Clone, Debug)]
pub struct Scale {
    pub key_note: u7,
    pub steps: Vec<Step>,
}

impl Scale {
    pub fn as_midi_notes(&self) -> Vec<u7> {
        self.steps
            .iter()
            .map(|s| s.as_chromatic() + self.key_note)
            .collect()
    }
    pub fn as_midi_notes_with_octave_range(&self, octave_range: Range<i8>) -> Vec<u7> {
        let mut result: Vec<u7> = vec![];
        let midi_scale: Vec<u7> = self.as_midi_notes();
        for o in octave_range {
            result.append(
                &mut midi_scale
                    .iter()
                    .map(|n| octave(*n, o))
                    .collect::<Vec<u7>>(),
            )
        }

        result
    }
}

impl Step {
    pub fn as_chromatic(&self) -> u7 {
        u7::new(match self {
            Step::Normal(1) => 1,
            Step::Minor(2) => 2,
            Step::Major(2) => 3,
            Step::Minor(3) => 4,
            Step::Major(3) => 5,
            Step::Minor(4) => 6,
            Step::Major(4) => 7,
            Step::Normal(5) => 8,
            Step::Minor(6) => 9,
            Step::Major(6) => 10,
            Step::Minor(7) => 11,
            Step::Major(7) => 12,
            Step::Normal(8) => 13,
            _ => panic!("dont now what {:?} is", self),
        })
    }
}

pub fn step_as_freq(step: &Step, key_note: f32) -> f32 {
    chromatic_step_as_freq(step.as_chromatic(), key_note)
}

fn chromatic_step_as_freq(cstep: u7, key_note: f32) -> f32 {
    (2.0_f32).powf(cstep.as_int() as f32 / 12.0) * key_note
}

pub fn octave(n: u7, o: i8) -> u7 {
    u7::new((n.as_int() as i8 + (12 * o)).try_into().unwrap_or(0))
}
