use harmony_forge::{
    music_generator::MusicGenerator,
    note::{chords::Chord, Scale, Step},
};
use rodio::{OutputStream, Sink};

const A: f32 = 440.0;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sink2 = Sink::try_new(&stream_handle).unwrap();

    let a_major = Scale::new_major(A);
    dbg!(a_major.as_freq());

    MusicGenerator::new(
        [
            Chord::new_minor_seventh(a_major.as_freq()[2 - 1]),
            Chord::new_dominant_seventh(a_major.as_freq()[5 - 1]),
            Chord::new_major_seventh(a_major.as_freq()[1 - 1]),
            Chord::new_major_seventh(a_major.as_freq()[1 - 1]),
        ],
        a_major,
        0,
    )
    .play(sink, sink2)
}
