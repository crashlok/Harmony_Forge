use super::{GenModels, Generator};
use crate::player::Player;
use midir::MidiOutputConnection;
use nodi::{Event, Moment};
use std::{sync::mpsc, thread};
pub struct MusicGenerator<C, M>
where
    C: Generator<Item = Vec<Event>> + Send + 'static,
    M: Generator<Item = Vec<Event>> + Send + 'static,
{
    rx: Option<mpsc::Receiver<()>>,
    melody_gen: M,
    chord_gen: C,
}

impl<C, M> MusicGenerator<C, M>
where
    C: Generator<Item = Vec<Event>> + Send + 'static,
    M: Generator<Item = Vec<Event>> + Send + 'static,
{
    pub fn new(chord_gen: C, melody_gen: M) -> Self {
        MusicGenerator {
            rx: None,
            melody_gen,
            chord_gen,
        }
    }

    pub fn play(
        mut self,
        t: crate::timers::TickerWithTime,
        con: MidiOutputConnection,
    ) -> (mpsc::Sender<()>, thread::JoinHandle<()>) {
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        let mut player = Player::new(self, con, t);
        let handle = thread::spawn(move || player.play());
        (tx, handle)
    }
}

impl<C, M> Generator for MusicGenerator<C, M>
where
    C: Generator<Item = Vec<Event>> + Send + 'static,
    M: Generator<Item = Vec<Event>> + Send + 'static,
{
    type Item = nodi::Moment;

    fn gen(&mut self, gen_models: &mut GenModels) -> Self::Item {
        let mut result: Moment = Moment { events: Vec::new() };

        for message in self.melody_gen.gen(gen_models) {
            result.push(message)
        }
        for message in self.chord_gen.gen(gen_models) {
            result.push(message)
        }
        result
    }
}
