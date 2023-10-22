use rodio::{
    source::{Delay, FadeIn, SineWave, TakeDuration},
    Source,
};
use std::time::Duration;

use crate::source_fadeout::SourceUpgrade;

#[derive(Debug)]
pub enum Step {
    Minor(usize),
    Major(usize),
    Normal(usize),
}
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

    pub fn new_major(key_freq: f32) -> Scale {
        Scale {
            key_freq,
            steps: vec![
                Step::Normal(1),
                Step::Major(2),
                Step::Major(3),
                Step::Minor(4),
                Step::Normal(5),
                Step::Major(6),
                Step::Major(7),
                Step::Normal(8),
            ],
        }
    }

    pub fn new_minor(key_freq: f32) -> Scale {
        Scale {
            key_freq,
            steps: vec![
                Step::Normal(1),
                Step::Major(2),
                Step::Major(3),
                Step::Minor(4),
                Step::Normal(5),
                Step::Major(6),
                Step::Major(7),
                Step::Normal(8),
            ],
        }
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

pub fn octave(x: f32, o: i32) -> f32 {
    x * (2.0_f64.powi(o) as f32)
}

pub fn sine_wave_octave(freq: f32, duration: f32, o: i32) -> FadeIn<TakeDuration<SineWave>> {
    sine_wave(octave(freq, o), duration)
}

pub fn sine_wave(freq: f32, duration: f32) -> FadeIn<TakeDuration<SineWave>> {
    SineWave::new(freq)
        .take_duration_fadeout(Duration::from_secs_f32(duration))
        .fade_in(Duration::from_secs_f32(0.05))
}

pub fn pause(duration: f32) -> TakeDuration<SineWave> {
    SineWave::new(0.0).take_duration(Duration::from_secs_f32(duration))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn octaviert() {
        assert_eq!(octave(440.0, -1), 220.0);
        assert_eq!(octave(440.0, 1), 880.0);
        assert_eq!(octave(440.0, 0), 440.0);
    }
}
