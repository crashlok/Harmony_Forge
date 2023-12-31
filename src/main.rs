#![deny(warnings)]
#![allow(unused_imports)]
use harmony_forge::{
    generators::{
        music_generator::MusicGenerator,
        note_generator::{NearNotes, NotesDependingBar},
        pattern_generators::{EmptyPattern, OnBeatPattern},
    },
    note::{chords::Chord, Scale},
    probability_density_function,
    timers::TickerWithTime,
};
use midir::{MidiOutput, MidiOutputPort};
use nodi::{self, timers::Ticker};
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let out: MidiOutput = midir::MidiOutput::new("harmony_forge").expect("very bad");
    for i in &out.ports() {
        dbg!(out.port_name(i).as_deref().unwrap_or("<no device name>"));
    }
    let port: &MidiOutputPort = &out.ports()[0];
    let con = out.connect(port, "HarmonyForgeOut").expect("very bad");
    let (_tx, handle): (mpsc::Sender<()>, thread::JoinHandle<()>) = MusicGenerator::new()
        .add_generator(OnBeatPattern::new(NotesDependingBar::new(vec![
            Chord::new_major(60).as_midi_notes(),
            Chord::new_major(67).as_midi_notes(),
            Chord::new_major(65).as_midi_notes(),
            Chord::new_major(60).as_midi_notes(),
        ])))
        .play(Ticker::with_initial_tempo(100, 700000), con, 3);

    handle.join().expect("music generator paniced")
}

fn _probability_test() {
    for i in 0..100 {
        let n = f64::floor(probability_density_function(i as f64, 50.0, 3.5) * 12.0_f64.powf(2.0));
        print!(" {} {} ", i, n);
        for _ in 0..(n) as i32 {
            print!("#")
        }
        println!();
    }
}
