use std::fmt;

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
