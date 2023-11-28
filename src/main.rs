use harmony_forge::{
    music_generator::{
        note_generator::RandomNotes,
        pattern_generators::{EmptyPattern, OnBeatPattern},
        MusicGenerator,
    },
    note::Scale,
};
use midir::{MidiOutput, MidiOutputPort};
use nodi::timers::Ticker;
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let out: MidiOutput = midir::MidiOutput::new("harmony_forge").expect("very bad");
    for i in &out.ports() {
        dbg!(out.port_name(i).as_deref().unwrap_or("<no device name>"));
    }
    let port: &MidiOutputPort = &out.ports()[1];
    let con = out.connect(port, "1").expect("very bad");

    let m_gen = MusicGenerator::new(
        EmptyPattern,
        OnBeatPattern::new(RandomNotes::new(Scale::new_major(60), 0..1)),
    );

    let (_tx, handle): (mpsc::Sender<()>, thread::JoinHandle<()>) = m_gen.play(
        Ticker::with_initial_tempo(
            1,
            Duration::from_secs_f32(0.0001)
                .as_micros()
                .try_into()
                .unwrap(),
        ),
        con,
    );

    handle.join().expect("music generator paniced")
}
