pub mod note{
    use rodio::{source::{TakeDuration, SineWave,Mix}, Source};
    use std::time::Duration;

    //pub fn poly<S,S1,S2>(sources:Vec<S>)-> Mix<S1,S2>
    //where
    //S:Source,
    //S1:Source,
    //S2:Source,
    //{}
    fn build_scale(){

    }


    pub fn octave(x:f32,o:i32)->f32{
        x*(2.0_f64.powi(o)as f32)
    }

    pub fn sine_wave_octave(freq:f32,duration:f32,o:i32)->TakeDuration<SineWave>{
        SineWave::new(octave(freq,o)).take_duration(Duration::from_secs_f32(duration))
    }

    pub fn sine_wave(freq:f32,duration:f32)->TakeDuration<SineWave>{
        SineWave::new(freq).take_duration(Duration::from_secs_f32(duration))
    }

    pub fn pause(duration:f32)-> TakeDuration<SineWave>{
        sine_wave(0., duration)
    }

    #[cfg(test)]
    mod tests{
        use super::*;
        
        #[test]
        fn octaviert(){
            assert_eq!(octave(440.0,-1),220.0);
            assert_eq!(octave(440.0,1),880.0);
            assert_eq!(octave(440.0,0),440.0);
        }
    }

}