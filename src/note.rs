pub mod chords;
mod scales;
pub mod tone;

#[derive(Clone, Debug, Copy)]
pub enum Step {
    Minor(usize),
    Major(usize),
    Normal(usize),
}
#[derive(Clone, Debug)]
pub struct Scale {
    key_freq: f32,
    steps: Vec<Step>,
}

impl Scale {
    pub fn as_freq(&self) -> Vec<f32> {
        self.steps
            .iter()
            .map(|s| step_as_freq(s, self.key_freq))
            .collect()
    }
}

pub fn step_as_freq(step: &Step, key_freq: f32) -> f32 {
    chromatic_step_as_freq(step_to_chromatic(step), key_freq)
}

fn chromatic_step_as_freq(cstep: usize, key_freq: f32) -> f32 {
    (2.0 as f32).powf(cstep as f32 / 12.0) * key_freq
}

fn step_to_chromatic(step: &Step) -> usize {
    match step {
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
        _ => panic!("dont now what {:?} is", step),
    }
}
