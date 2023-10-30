use crate::note::{
    chords::{self, Chord},
    tone::sine_wave_octave,
    Scale, Step,
};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use rodio::Sink;

#[derive(Clone, Debug)]
pub struct MusicGenerator {
    chords: [Chord; 4],
    scale: Scale,
    mood: i8,
    chord_counter: usize,
}

impl MusicGenerator {
    pub fn new(chords: [Chord; 4], scale: Scale, mood: i8) -> MusicGenerator {
        MusicGenerator {
            chords,
            scale,
            mood,
            chord_counter: 0,
        }
    }

    pub fn play(&mut self, chord_sink: Sink, melody_sink: Sink) {
        loop {
            if melody_sink.len() <= 4 {
                melody_sink.append(sine_wave_octave(self.gen_melody_note(), 0.25, -1));
            }

            if chord_sink.len() <= 4 {
                melody_sink.append(self.chords[self.chord_counter].as_sine_wave(1., -2));
                self.chord_counter += 1;
                if self.chord_counter >= self.chords.len() {
                    self.chord_counter = 0
                }
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
