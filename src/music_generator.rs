use midir::MidiOutputConnection;
use midly::live::SystemCommon;
use nodi::{timers::Ticker, Event, Moment};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use std::{
    rc::{Rc, Weak},
    sync::mpsc,
    thread,
};

use midi_player::MidiPlayer;

pub mod chord_generators;
pub mod melody_generators;
mod midi_player;

//#[derive(Debug)]
pub struct MusicGenerator<C, M>
where
    C: ChordGen,
    M: MelodyGen,
{
    rx: Option<mpsc::Receiver<()>>,
    melody_gen: M,
    chord_gen: C,
}

impl<C, M> MusicGenerator<C, M>
where
    C: ChordGen,
    M: MelodyGen,
{
    pub fn new(chord_gen: C, melody_gen: M) -> Self {
        MusicGenerator {
            rx: None,
            melody_gen,
            chord_gen,
        }

        //let midi_player = MidiPlayer::new(m);
        //m.player = Rc::downgrade(&Rc::new(midi_player));
    }

    pub fn play(mut self, t: Ticker, con: MidiOutputConnection) -> mpsc::Sender<()> {
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        let midi_player = MidiPlayer::new(self, con, t);
        thread::spawn(move || midi_player.play());
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
        let mut result: Moment = Moment { events: Vec::new() };

        for message in self.chord_gen.gen() {
            result.push(*message)
        }

        for message in self.melody_gen.gen() {
            result.push(*message)
        }

        Some(result)
    }
}

pub trait MelodyGen {
    fn gen(&mut self) -> &[Event];
}

pub trait ChordGen {
    fn gen(&mut self) -> &[Event];
}
