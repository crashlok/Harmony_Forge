use crate::source_fadeout::SourceUpgrade;
use rodio::{
    source::{Mix, SineWave, TakeDuration},
    Sample, Source,
};
use std::time::Duration;

pub fn chord<S>(tones: [S; 5]) -> Mix<Mix<Mix<Mix<S, S>, S>, S>, S>
where
    S: Source + Copy,
    S::Item: Sample,
{
    tones[0]
        .mix(tones[1])
        .mix(tones[2])
        .mix(tones[3])
        .mix(tones[4])
}

pub fn octave(x: f32, o: i32) -> f32 {
    x * (2.0_f64.powi(o) as f32)
}

pub fn sine_wave_octave(freq: f32, duration: f32, o: i32) -> TakeDuration<SineWave> {
    sine_wave(octave(freq, o), duration)
}

pub fn sine_wave(freq: f32, duration: f32) -> TakeDuration<SineWave> {
    SineWave::new(freq).take_duration_fadeout(Duration::from_secs_f32(duration))
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
