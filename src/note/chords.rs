use super::{Scale, Step};

pub struct Triad {
    scale: Scale,
}

pub trait Chord {
    fn as_freq(&self) -> Vec<f32>;
    fn as_5_array();
}
