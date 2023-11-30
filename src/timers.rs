use nodi::Timer;
use std::time::Duration;

pub struct TickerWithTime {
    ticks_per_quarter: u16,
    micros_per_tick: f64,
    done_ticks: u64,
    /// Speed modifier, a value of `1.0` is the default and affects nothing.
    ///
    /// Important: Do not set to 0.0, this value is used as a denominator.
    pub speed: f32,
}

impl TickerWithTime {
    /// Creates an instance of [Self] with the given ticks-per-beat.
    /// The tempo will be infinitely rapid, meaning no sleeps will happen.
    /// However this is rarely an issue since a tempo change message will set
    /// it, and this usually happens before any non-0 offset event.
    pub const fn new(ticks_per_quarter: u16) -> Self {
        Self {
            ticks_per_quarter,
            micros_per_tick: 0.0,
            speed: 1.0,
            done_ticks: 0,
        }
    }

    pub fn get_time(&self) -> MusicTime {
        let quarters: i32 =
            <f64>::floor(self.done_ticks as f64 / self.ticks_per_quarter as f64) as i32;
        let mut rest: u64 = self.done_ticks % self.ticks_per_quarter as u64;
        let eighths: i32 = <f64>::floor(rest as f64 / (self.ticks_per_quarter * 2) as f64) as i32;
    }
    /// Will create an instance of [Self] with a provided tempo.
    pub fn with_initial_tempo(ticks_per_beat: u16, tempo_bpm: u32) -> Self {
        let mut s = Self::new(ticks_per_beat);
        s.change_tempo(tempo_bpm);
        s
    }
}

impl nodi::Timer for TickerWithTime {
    fn change_tempo(&mut self, tempo_bpm: u32) {
        let micros_per_tick =
            (60.0 / tempo_bpm as f64) / self.ticks_per_quarter as f64 * 1_000_000.0;
        self.micros_per_tick = micros_per_tick;
    }

    fn sleep_duration(&mut self, n_ticks: u32) -> Duration {
        self.done_ticks += n_ticks as u64;
        let t = self.micros_per_tick * n_ticks as f64 / self.speed as f64;
        if t > 0.0 {
            Duration::from_micros(t as u64)
        } else {
            Duration::default()
        }
    }
}

pub struct MusicTime {
    time_signatur_quarters: i32,
    bars: i32,
    quarters: i32,
    eights: i32,
    rest_quarters: f64,
}
