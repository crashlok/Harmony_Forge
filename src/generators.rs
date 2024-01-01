use crate::models::Models;

pub mod music_generator;
pub mod note_generator;
pub mod pattern_generators;

pub trait Generator {
    type Item;
    fn gen(&mut self, gen_models: &mut Models) -> Self::Item;
}

fn midi_massage_event(message: midly::MidiMessage, channel: midly::num::u4) -> nodi::Event {
    nodi::Event::Midi(nodi::MidiEvent { channel, message })
}

fn multiple_notes_on(
    notes: Vec<midly::num::u7>,
    velocity: midly::num::u7,
    channel: midly::num::u4,
) -> Vec<nodi::Event> {
    notes
        .iter()
        .map(|n| {
            midi_massage_event(
                midly::MidiMessage::NoteOn {
                    key: *n,
                    vel: velocity,
                },
                channel,
            )
        })
        .collect()
}

fn multiple_notes_off(
    notes: Vec<midly::num::u7>,
    velocity: midly::num::u7,
    channel: midly::num::u4,
) -> Vec<nodi::Event> {
    notes
        .iter()
        .map(|n| {
            midi_massage_event(
                midly::MidiMessage::NoteOff {
                    key: *n,
                    vel: velocity,
                },
                channel,
            )
        })
        .collect()
}
