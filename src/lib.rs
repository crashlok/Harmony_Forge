use std::f64::consts;

pub mod music_generator;
pub mod note;
pub mod timers;

pub fn probability_density_function(x: f64, mu: f64, sigma: f64) -> f64 {
    (1.0 / (sigma * (2.0 * consts::PI).sqrt()))
        * consts::E.powf(-0.5 * ((x - mu) / sigma).powf(2.0))
}

pub fn display_distributtion(input: &Vec<f64>) {
    for (i, x) in input.iter().enumerate() {
        let x = f64::floor(x * 12.0_f64.powf(2.0));
        print!(" {} {} ", i, x);
        for _ in 0..(x) as i32 {
            print!("#")
        }
        println!("");
    }
}
