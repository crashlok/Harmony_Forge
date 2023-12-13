use nodi::Timer;
use std::{fmt, time::Duration};

use crate::music_generator::NoteGenerator;

pub struct TickerWithTime {
    ticks_per_quarter: u16,
    micros_per_tick: f64,
    done_ticks: u64,
    /// Speed modifier, a value of `1.0` is the default and affects nothing.
    ///
    /// Important: Do not set to 0.0, this value is used as a denominator.
    pub speed: f32,
    pub time: MusicTime,
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
            time: MusicTime::new(),
        }
    }

    pub fn set_time_signature(mut self, time_signature_quarters: u16) -> Self {
        self.time.set_time_signature(time_signature_quarters);
        self
    }

    pub fn get_time(&mut self) -> Option<&MusicTime> {
        self.time
            .add_ticks(self.done_ticks, self.ticks_per_quarter)?;
        self.done_ticks = 0;
        Some(&self.time)
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
    time_signature_quarters: Option<u16>,
    bars: i32,
    quarters: f64,
}

impl fmt::Debug for MusicTime {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("MusicTime")
            .field("Bars", &self.get_bars())
            .field("Quarters", &self.get_quarters_i32())
            .field("Eights", &self.get_eights_i32())
            .field("Rest(Quarters)", &self.get_rest_quarters())
            .finish()
    }
}

impl MusicTime {
    pub const fn new() -> Self {
        Self {
            time_signature_quarters: None,
            bars: 0,
            quarters: 0.0,
        }
    }

    pub fn set_time_signature(&mut self, time_signature_quarters: u16) -> &mut Self {
        self.time_signature_quarters = Some(time_signature_quarters);
        self
    }

    pub fn add_ticks(&mut self, ticks: u64, ticks_per_quarter: u16) -> Option<&mut Self> {
        self.quarters += ticks as f64 / ticks_per_quarter as f64;
        self.update_bars()
    }

    fn update_bars(&mut self) -> Option<&mut Self> {
        self.bars += (self.quarters / self.time_signature_quarters? as f64).floor() as i32;
        self.quarters %= self.time_signature_quarters? as f64;
        Some(self)
    }
    pub fn get_bars(&self) -> i32 {
        self.bars
    }

    pub fn get_eights_f64(&self) -> f64 {
        self.quarters * 2.0
    }

    pub fn get_eights_i32(&self) -> i32 {
        self.quarters.floor() as i32 * 2
    }
    pub fn get_quarters_f64(&self) -> f64 {
        self.quarters
    }

    pub fn get_quarters_i32(&self) -> i32 {
        self.quarters.floor() as i32
    }

    pub fn get_rest_quarters(&self) -> f64 {
        self.quarters % 1.
    }
}
