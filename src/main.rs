use core::time::Duration;
use harmony_forge::{
    note::{self, Scale},
    source_fadeout,
};
use rodio::{OutputStream, Sink, Source,source::{self, SineWave}};

const A: f32 = 440.0;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let mut source =source::SineWave::new(A).take_duration(Duration::from_secs_f32(1.0));
    source.set_filter_fadeout();
    sink.append(source);

    sink.append(note::sine_wave_octave(A, 1.0, 0).amplify(1.0));

    sink.append(note::sine_wave_octave(A, 1.0, 0).amplify(0.005));

    let scale_a_major: Scale = Scale::new_major(A);
    for s in scale_a_major.as_freq() {
        sink.append(note::sine_wave_octave(s, 0.25, 0))
    }

    sink.sleep_until_end();
}
