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

fn multiple_massage(
    notes: Vec<midly::num::u7>,
    message: midly::MidiMessage,
    channel: midly::num::u4,
) -> Vec<nodi::Event> {
    let result: Vec<nodi::Event> = Vec::new();
    for n in notes {
        let spec_massage: midly::MidiMessage = message.clone();
        spec_massage::key = n;
        result.push(midi_massage_event(spec_massage, channel))
    }
    result
}
