use harmony_forge::{
    music_generator::{chord_generators::Test, melody_generators::Random, MusicGenerator},
    note::Scale,
};
use midir::{MidiOutput, MidiOutputPort};
use midly::Smf;
use nodi::timers::Ticker;
use std::{fs, sync::mpsc, thread};

const A: f32 = 440.0;

fn main() {
    //let m = fs::read("./src/Queen_-_Bohemian_Rhapsody.mid").unwrap();
    //println!("{:?}", Smf::parse(&m).unwrap());

    let out: MidiOutput = midir::MidiOutput::new("harmony_forge").expect("very bad");
    for i in &out.ports() {
        dbg!(out.port_name(i).as_deref().unwrap_or("<no device name>"));
    }
    let port: &MidiOutputPort = &out.ports()[1];
    let con = out.connect(port, "1").expect("very bad");

    let m_gen = MusicGenerator::new(Test {}, Random::new(Scale::new_major(60), 0..0));

    let (_tx, handle): (mpsc::Sender<()>, thread::JoinHandle<()>) =
        m_gen.play(Ticker::with_initial_tempo(100, 100), con);

    handle.join().expect("music generator paniced")
}
