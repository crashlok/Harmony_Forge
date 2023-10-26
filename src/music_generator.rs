use crate::note::{chords, tone::sine_wave_octave, Scale, Step};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use rodio::Sink;

#[derive(Clone, Debug)]
pub struct MusicGenerator {
    chords: [chords::Triad; 4],
    scale: Scale,
    mood: i8,
}

impl MusicGenerator {
    pub fn new(chords: [[Step; 3]; 4], scale: Scale, mood: i8) -> MusicGenerator {
        MusicGenerator {
            chords,
            scale,
            mood,
        }
    }

    pub fn play(&self, chord_sink: Sink, melody_sink: Sink) {
        loop {
            if melody_sink.len() <= 4 {
                melody_sink.append(sine_wave_octave(self.gen_melody_note(), 0.25, -1))
            }
        }
    }

    fn gen_melody_note(&self) -> f32 {
        let scale = self.scale.as_freq();
        scale[Uniform::from(0..scale.len()).sample(&mut thread_rng())]
    }

    pub fn set_mood(&mut self, mood: i8) -> Self {
        todo!()
    }
}
