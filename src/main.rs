#![deny(warnings)]
#![allow(unused_imports)]
use harmony_forge::{
    bpm_to_micros_per_beat,
    generators::{
        music_generator::MusicGenerator,
        note_generator::{NearNotes, NotesDependingBar, OneNote, RandomNotes},
        pattern_generator::{EmptyPattern, OnBeatPattern, OnClosurePattern},
        universal_generator::MultipleClosure,
    },
    note::{chords::Chord, octave, Scale, Step},
    probability_density_function,
    timers::TickerWithTime,
};
use midir::{MidiOutput, MidiOutputPort};
use midly::num::u7;
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
        .add_generator(Box::new(MultipleClosure::new(
            |models| match models.time.get_bars() {
                0 => 1,
                _ => 1,
            },
            vec![
                Box::new(OnClosurePattern::new(
                    |models| models.time.on_eight() && models.time.get_eights_i32() % 3 == 0,
                    NotesDependingBar::new(vec![
                        Chord::new_major(56).as_midi_notes(),
                        Chord::new_minor(53).as_midi_notes(),
                        Chord::new_minor(48).as_midi_notes(),
                        Chord::new_minor(48).as_midi_notes(),
                    ]),
                    0,
                )),
                Box::new(OnBeatPattern::new(
                    NotesDependingBar::new(vec![
                        Chord::new_major(56).as_midi_notes(),
                        Chord::new_minor(53).as_midi_notes(),
                        Chord::new_minor(48).as_midi_notes(),
                        Chord::new_minor(48).as_midi_notes(),
                    ]),
                    0,
                )),
            ],
        )))
        .add_generator(Box::new(MultipleClosure::new(
            |models| (models.time.get_bars() % 3) as usize,
            vec![
                Box::new(OnClosurePattern::new(
                    |models| {
                        models.time.on_eight()
                            && !models.time.on_quarter()
                            && models.time.get_eights_i32() % 3 == 0
                    },
                    OneNote::new(vec![61]),
                    1,
                )),
                Box::new(OnClosurePattern::new(
                    |models| models.time.on_eight() && !models.time.on_quarter(),
                    OneNote::new(vec![61]),
                    1,
                )),
            ],
        )))
        .add_generator(Box::new(OnClosurePattern::new(
            |models| models.time.on_quarter() && models.time.get_quarters_i32() % 2 == 0,
            NearNotes::new(Scale::new_minor(60), 1..3),
            2,
        )))
        .add_generator(Box::new(OnClosurePattern::new(
            |models| models.time.on_bar(),
            NotesDependingBar::new(vec![
                vec![octave(Chord::new_major(56).as_midi_notes()[0], -1)],
                vec![octave(Chord::new_major(53).as_midi_notes()[0], -1)],
                vec![octave(Chord::new_major(48).as_midi_notes()[0], -1)],
                vec![octave(Chord::new_major(48).as_midi_notes()[0], -1)],
            ]),
            3,
        )))
        .play(
            Ticker::with_initial_tempo(100, bpm_to_micros_per_beat(120)),
            con,
            4,
        );

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
