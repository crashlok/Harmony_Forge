use rodio::source::{SineWave, TakeDuration};

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

    pub fn as_sine_wave(&self, duration: f32, o: i32) -> tone::ChordSource<TakeDuration<SineWave>> {
        tone::chord(
            self.as_5_freq()
                .iter()
                .map(|f| return tone::sine_wave_octave(*f, duration, o))
                .collect(),
        )
    }

    pub fn as_freq(&self) -> Vec<f32> {
        self.steps
            .iter()
            .map(|s| step_as_freq(s, self.key_freq))
            .collect()
    }

    pub fn as_5_freq(&self) -> Vec<f32> {
        [0. as f32; 5]
            .iter()
            .enumerate()
            .map(|(index, _)| match self.steps.iter().next() {
                Some(s) => step_as_freq(s, self.key_freq),
                None => 0.0,
            })
            .collect::<Vec<f32>>()
    }
}
