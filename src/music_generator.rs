use midir::MidiOutputConnection;
use nodi::{timers::Ticker, Event, Moment};
use std::{sync::mpsc, thread};

use midi_player::MidiPlayer;

pub mod chord_generators;
pub mod melody_generators;
mod midi_player;

//#[derive(Debug)]
pub struct MusicGenerator<C, M>
where
    C: ChordGen + Send + 'static,
    M: MelodyGen + Send + 'static,
{
    rx: Option<mpsc::Receiver<()>>,
    melody_gen: M,
    chord_gen: C,
}

impl<C, M> MusicGenerator<C, M>
where
    C: ChordGen + Send + 'static,
    M: MelodyGen + Send + 'static,
{
    pub fn new(chord_gen: C, melody_gen: M) -> Self {
        MusicGenerator {
            rx: None,
            melody_gen,
            chord_gen,
        }
    }

    pub fn play(mut self, t: Ticker, con: MidiOutputConnection) -> mpsc::Sender<()> {
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        let mut midi_player = MidiPlayer::new(self, con, t);
        thread::spawn(move || midi_player.play());
        tx
    }
}

impl<C, M> Iterator for MusicGenerator<C, M>
where
    C: ChordGen + Send + 'static,
    M: MelodyGen + Send + 'static,
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
