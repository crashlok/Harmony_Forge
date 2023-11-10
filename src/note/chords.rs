use super::tone;
use super::*;

#[derive(Debug, Clone)]
pub struct Chord {
    key_freq: f32,
    steps: Vec<Step>,
}

impl Chord {
    pub fn new_major(key_freq: f32) -> Chord {
        Chord {
            key_freq: key_freq,
            steps: vec![Step::Normal(1), Step::Major(3), Step::Normal(5)],
        }
    }

    pub fn new_minor(key_freq: f32) -> Chord {
        Chord {
            key_freq: key_freq,
            steps: vec![Step::Normal(1), Step::Minor(3), Step::Normal(5)],
        }
    }

    pub fn new_major_seventh(key_freq: f32) -> Chord {
        Chord {
            key_freq: key_freq,
            steps: vec![
                Step::Normal(1),
                Step::Major(3),
                Step::Normal(5),
                Step::Major(7),
            ],
        }
    }

    pub fn new_minor_seventh(key_freq: f32) -> Chord {
        Chord {
            key_freq: key_freq,
            steps: vec![
                Step::Normal(1),
                Step::Minor(3),
                Step::Normal(5),
                Step::Major(7),
            ],
        }
    }

    pub fn new_dominant_seventh(key_freq: f32) -> Chord {
        Chord {
            key_freq: key_freq,
            steps: vec![
                Step::Normal(1),
                Step::Major(3),
                Step::Normal(5),
                Step::Minor(7),
            ],
        }
    }

    pub fn as_midi_notes(&self) -> Vec<f32> {
        self.steps
            .iter()
            .map(|s| s.as_chromatic() + key_note)
            .collect()
    }

    }
}
