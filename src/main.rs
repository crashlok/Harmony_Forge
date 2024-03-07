#![deny(warnings)]
#![allow(unused_imports)]
use crate::plays::*;
use dialoguer::FuzzySelect;
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
use midir::{ConnectError, MidiOutput, MidiOutputConnection, MidiOutputPort};
use midly::num::u7;
use nodi::{self, timers::Ticker};
use std::{sync::mpsc, thread, time::Duration};

mod plays;

fn main() {
    let con = choose_connection(
        midir::MidiOutput::new("harmony_forge").expect("failed to instanciat output"),
    )
    .expect("failed to connect");
    let choosen_play = choose_play(vec![
        (piece, "piece"),
        (random,"random")
    ]);
    let (_tx, handle): (mpsc::Sender<()>, thread::JoinHandle<()>) = choosen_play().play(
        Ticker::with_initial_tempo(100, bpm_to_micros_per_beat(120)),
        con,
        4,
    );

    handle.join().expect("music generator paniced")
}

fn choose_play(plays: Vec<(fn() -> MusicGenerator, &str)>) -> fn() -> MusicGenerator {
    let play_index = FuzzySelect::new()
        .with_prompt("choose play")
        .items(&(plays.iter().map(|f| f.1).collect::<Vec<&str>>()))
        .interact()
        .expect("failed to choose");
    plays[play_index].0
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
fn choose_connection(out: MidiOutput) -> Result<MidiOutputConnection, ConnectError<MidiOutput>> {
    println!();
    let port_names = out
        .ports()
        .iter()
        .map(|port| out.port_name(port).unwrap_or("<no device name>".to_owned()))
        .collect::<Vec<String>>();
    let choosen_port_index = FuzzySelect::new()
        .with_prompt("choose port")
        .items(&port_names)
        .interact()
        .expect("failed to choose");

    let port: &MidiOutputPort = &out.ports()[choosen_port_index];
    println!();
    out.connect(port, "HarmonyForgeOut")
}
