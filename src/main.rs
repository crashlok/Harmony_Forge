
    use rodio::{
        source::{SineWave, Empty, Source,TakeDuration},
        OutputStream, Sink,
    };
    use std::time::Duration;
    
    fn main() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
    
 
        let source = SineWave::new(440.0)
            .take_duration(Duration::from_secs_f32(0.25))
            .amplify(0.20);
        sink.append(source);
    
        let source = SineWave::new(0.).take_duration(Duration::from_secs_f32(0.25));
        sink.append(source);

        sink.sleep_until_end();
    }
    

