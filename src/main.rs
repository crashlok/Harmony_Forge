use harmony_forge::music_generator::MusicGenerator;
use midly::Smf;
use std::fs;

const A: f32 = 440.0;

fn main() {
    let m = fs::read("./src/Queen_-_Bohemian_Rhapsody.mid").unwrap();
    println!("{:?}", Smf::parse(&m).unwrap())
}
