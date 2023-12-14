use harmony_forge::{
    generators::{
        music_generator::MusicGenerator,
        note_generator::NearNotes,
        pattern_generators::{EmptyPattern, OnBeatPattern},
    },
    note::Scale,
    probability_density_function,
    timers::TickerWithTime,
};

use midir::{MidiOutput, MidiOutputPort};
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
        OnBeatPattern::new(NearNotes::new(Scale::new_major(60), 0..2)),
    );

    let (_tx, handle): (mpsc::Sender<()>, thread::JoinHandle<()>) = m_gen.play(
        TickerWithTime::with_initial_tempo(
            1,
            Duration::from_secs_f32(0.0001)
                .as_micros()
                .try_into()
                .unwrap(),
        )
        .set_time_signature(4),
        con,
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
