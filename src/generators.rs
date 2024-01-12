use crate::models::Models;

pub mod music_generator;
pub mod music_generator_as;
pub mod note_generator;
pub mod pattern_generator;
pub mod universal_generator;
pub trait Generator {
    type Item;
    fn gen(&mut self, gen_models: Models) -> (Self::Item, Models);
}

type Gen<I> = dyn Generator<Item = I> + Send;

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
