use harmony_forge::{
    music_generator::{
        note_generator::{NearNotes, RandomNotes},
        pattern_generators::{EmptyPattern, OnBeatPattern},
        MusicGenerator,
    },
    note::Scale,
    probability_density_function,
    timers::TickerWithTime,
};
use midir::{MidiOutput, MidiOutputPort};
use nodi::{timers::Ticker, Timer};
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let mut ticker = TickerWithTime::with_initial_tempo(100, 60).set_time_signature(4);
    loop {
        ticker.sleep(1);
        dbg!(ticker.get_time());
    }

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

fn probability_test() {
    for i in 0..100 {
        let n = f64::floor(probability_density_function(i as f64, 50.0, 3.5) * 12.0_f64.powf(2.0));
        print!(" {} {} ", i, n);
        for _ in 0..(n) as i32 {
            print!("#")
        }
        println!("");
    }
}
