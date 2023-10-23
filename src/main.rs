use harmony_forge::{
    music_generator::MusicGenerator,
    note::{Scale, Step},
};
use rodio::{OutputStream, Sink};

const A: f32 = 440.0;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sink2 = Sink::try_new(&stream_handle).unwrap();
    MusicGenerator::new([[Step::Normal(0); 3]; 4], Scale::new_minor(A), 0).play(sink, sink2)
}
