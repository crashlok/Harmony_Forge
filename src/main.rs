use harmony_forge::{
    music_generator::{chord_generators::Test, melody_generators::Random, MusicGenerator},
    note::Scale,
};
use midly::Smf;
use nodi::timers::Ticker;
use std::fs;

const A: f32 = 440.0;

fn main() {
    let m = fs::read("./src/Queen_-_Bohemian_Rhapsody.mid").unwrap();
    println!("{:?}", Smf::parse(&m).unwrap());

    let con = get_connection(device_no)?;

    let m_gen = MusicGenerator::new(Test {}, Random::new(Scale::new_major(60), 0..0));

    m_gen.play(Ticker::with_initial_tempo(100, 100), con)
}
