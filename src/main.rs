use rodio::{
    OutputStream, Sink,Source
};

use harmony_forge::note;
use std::time::Duration;

const A:f32 = 440.0;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(note::sine_wave_octave(A, 0.25,-1));
    sink.append(note::pause(0.25));
    sink.append(note::sine_wave_octave(A, 0.25,1).reverb(Duration::from_secs_f32(0.5),0.1));
    sink.append(note::pause(1.));
    sink.append(note::sine_wave_octave(A, 0.5, 0));
    sink.append(note::sine_wave_octave(A, 0.5,-1).mix(note::sine_wave_octave(A, 0.5,0)));

    sink.sleep_until_end();
}
    

