use nodi::{Event, Moment};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use std::{
    rc::{Rc, Weak},
    sync::mpsc,
};

use midi_player::MidiPlayer;

pub mod chord_generators;
pub mod melody_generators;
mod midi_player;

#[derive(Debug)]
pub struct MusicGenerator<C, M>
where
    C: ChordGen,
    M: MelodyGen,
{
    rx: mpsc::Receiver<()>,
    player: Weak<MidiPlayer>,
    melody_gen: M,
    chord_gen: C,
}

impl<C, M> MusicGenerator<C, M>
where
    C: ChordGen,
    M: MelodyGen,
{
    pub fn new(chord_gen: C, melody_gen: M) -> mpsc::Sender<()> {
        let (tx, rx) = mpsc::channel();
        let m = MusicGenerator {
            rx,
            player: Weak::new(),
            melody_gen,
            chord_gen,
        };

        let midi_player = MidiPlayer::new(m);
        m.player = Rc::downgrade(&Rc::new(midi_player));
        tx
    }
}

impl<C, M> Iterator for MusicGenerator<C, M>
where
    C: ChordGen,
    M: MelodyGen,
{
    type Item = nodi::Moment;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Moment = Moment { events: Vec::new() };

        self.chord_gen
            .gen()
            .iter()
            .map(|message| result.push(*message));

        self.melody_gen
            .gen()
            .iter()
            .map(|message| result.push(*message));

        Some(result)
    }
}

trait MelodyGen {
    fn gen(&mut self) -> &[Event];
}

trait ChordGen {
    fn gen(&mut self) -> &[Event];
}
