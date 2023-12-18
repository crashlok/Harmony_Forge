use super::*;
use midly::num::u7;

#[derive(Debug, Clone)]
pub struct Chord {
    key_note: u7,
    steps: Vec<Step>,
}

impl Chord {
    pub fn new_major(key_note: u8) -> Chord {
        Chord {
            key_note: u7::new(key_note),
            steps: vec![Step::Normal(1), Step::Major(3), Step::Normal(5)],
        }
    }

    pub fn new_minor(key_note: u8) -> Chord {
        Chord {
            key_note: u7::new(key_note),
            steps: vec![Step::Normal(1), Step::Minor(3), Step::Normal(5)],
        }
    }

    pub fn new_major_seventh(key_note: u8) -> Chord {
        Chord {
            key_note: u7::new(key_note),
            steps: vec![
                Step::Normal(1),
                Step::Major(3),
                Step::Normal(5),
                Step::Major(7),
            ],
        }
    }

    pub fn new_minor_seventh(key_note: u8) -> Chord {
        Chord {
            key_note: u7::new(key_note),
            steps: vec![
                Step::Normal(1),
                Step::Minor(3),
                Step::Normal(5),
                Step::Major(7),
            ],
        }
    }

    pub fn new_dominant_seventh(key_note: u8) -> Chord {
        Chord {
            key_note: u7::new(key_note),
            steps: vec![
                Step::Normal(1),
                Step::Major(3),
                Step::Normal(5),
                Step::Minor(7),
            ],
        }
    }

    pub fn as_midi_notes(&self) -> Vec<u7> {
        self.steps
            .iter()
            .map(|s| s.as_chromatic() + self.key_note)
            .collect()
    }
}
