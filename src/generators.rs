use crate::models::{notes_playing::NotesPlaying, time::MusicTime};

pub mod music_generator;
pub mod note_generator;
pub mod pattern_generators;

type GenModels<'a> = (&'a mut MusicTime, &'a mut NotesPlaying);

pub trait Generator {
    type Item;
    fn gen(&mut self, gen_models: &mut GenModels) -> Self::Item;
}

fn midi_massage_event(message: midly::MidiMessage, channel: midly::num::u4) -> nodi::Event {
    nodi::Event::Midi(nodi::MidiEvent { channel, message })
}
