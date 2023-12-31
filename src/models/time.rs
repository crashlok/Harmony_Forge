use std::{
    cmp,
    fmt::{self, Debug},
    ops,
};

#[derive(Clone, Copy)]
pub struct MusicTime {
    time_signature_quarters: Option<u16>,
    bars: i32,
    quarters: f64,
}

impl ops::Add for MusicTime {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self {
            time_signature_quarters: self.get_time_signature_in_quarters(),
            bars: self.get_bars() + rhs.get_bars(),
            quarters: self.get_quarters_f64() + rhs.get_quarters_f64(),
        };
        res.update_bars();
        res
    }
}
impl ops::Sub for MusicTime {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Self {
            time_signature_quarters: self.get_time_signature_in_quarters(),
            bars: self.get_bars() - rhs.get_bars(),
            quarters: self.get_quarters_f64() - rhs.get_quarters_f64(),
        };
        res.update_bars();
        res
    }
}
impl cmp::PartialEq for MusicTime {
    fn eq(&self, other: &Self) -> bool {
        self.get_full_quarters() == other.get_full_quarters()
    }
}

impl cmp::PartialOrd for MusicTime {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.get_full_quarters()
            .partial_cmp(&other.get_full_quarters())
    }
}

impl Debug for MusicTime {
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
    pub fn from_quarters(quarters: f64) -> Self {
        let mut res = Self {
            time_signature_quarters: None,
            bars: 0,
            quarters,
        };
        res.update_bars();
        res
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

    pub fn get_time_signature_in_quarters(&self) -> Option<u16> {
        self.time_signature_quarters
    }

    pub fn get_time_signature_in_eights(&self) -> Option<u16> {
        self.time_signature_quarters.map(|t| t * 2)
    }

    pub fn get_eights_i32(&self) -> i32 {
        self.quarters.floor() as i32 * 2
    }
    pub fn get_quarters_f64(&self) -> f64 {
        self.quarters
    }
    pub fn get_full_quarters(&self) -> f64 {
        if let Some(qbars) = self
            .get_time_signature_in_quarters()
            .map(|t| t as i32 * self.get_bars())
        {
            return self.get_quarters_f64() + qbars as f64;
        }
        self.get_quarters_f64()
    }

    pub fn get_quarters_i32(&self) -> i32 {
        self.quarters.floor() as i32
    }

    pub fn get_rest_quarters(&self) -> f64 {
        self.quarters % 1.
    }
}

impl Default for MusicTime {
    fn default() -> Self {
        Self::new()
    }
}
