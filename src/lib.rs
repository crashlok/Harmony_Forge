#![deny(warnings)]

use std::f64::consts;

pub mod generators;
mod models;
pub mod note;
mod player;
pub mod timers;

pub fn probability_density_function(x: f64, mu: f64, sigma: f64) -> f64 {
    (1.0 / (sigma * (2.0 * consts::PI).sqrt()))
        * consts::E.powf(-0.5 * ((x - mu) / sigma).powf(2.0))
}

pub fn display_distributtion(input: &[f64]) {
    for (i, x) in input.iter().enumerate() {
        let x = f64::floor(x * 12.0_f64.powf(2.0));
        print!(" {} {} ", i, x);
        for _ in 0..(x) as i32 {
            print!("#")
        }
        println!();
    }
}
pub fn bpm_to_micros_per_beat(bpm: u16) -> u32 {
    ((60_f64 / bpm as f64) * 1000000.0) as u32
}
