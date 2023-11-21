use harmony_forge::{
    music_generator::{
        note_generator::{self, RandomNotes},
        pattern_generators::{self, OnBeatPattern},
        MusicGenerator, NoteGenerator, PatternGenerator,
    },
    note::Scale,
};
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use midly::{Format, Smf};
use nodi::{timers::Ticker, Connection, Player, Sheet};
use std::{error::Error, fs, sync::mpsc, thread, time::Duration};

const A: f32 = 440.0;

fn main() {
    let m = fs::read("./src/Queen_-_Bohemian_Rhapsody.mid").unwrap();
    let Smf { header, tracks } = Smf::parse(&m).unwrap();
    //println!("{:#?}", tracks[1]);

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
