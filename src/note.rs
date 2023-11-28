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
    key_note: i32,
    steps: Vec<Step>,
}

impl Scale {
    pub fn as_midi_notes(&self) -> Vec<i32> {
        self.steps
            .iter()
            .map(|s| s.as_chromatic() + self.key_note)
            .collect()
    }
    pub fn as_midi_notes_with_octave_range(&self, octave_range: Range<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let midi_scale: Vec<i32> = self.as_midi_notes();
        for o in octave_range {
            result.append(
                &mut midi_scale
                    .iter()
                    .map(|n| octave(*n, o))
                    .collect::<Vec<i32>>(),
            )
        }

        result
    }
}

impl Step {
    pub fn as_chromatic(&self) -> i32 {
        match self {
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
        }
    }
}

pub fn step_as_freq(step: &Step, key_note: f32) -> f32 {
    chromatic_step_as_freq(step.as_chromatic(), key_note)
}

fn chromatic_step_as_freq(cstep: i32, key_note: f32) -> f32 {
    (2.0 as f32).powf(cstep as f32 / 12.0) * key_note
}

pub fn octave(n: i32, o: i32) -> i32 {
    n + (12 * o)
}
