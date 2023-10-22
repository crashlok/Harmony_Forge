use harmony_forge::note::{self, Scale};
use rodio::{OutputStream, Sink};

const A: f32 = 440.0;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let scale_a_major: Scale = Scale::new_major(A);
    for s in scale_a_major.as_freq() {
        sink.append(note::sine_wave_octave(s, 0.25, 0))
    }

    sink.sleep_until_end();
}
