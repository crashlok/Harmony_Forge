use super::Generator;
use crate::{models::Models, player::Player};
use midir::MidiOutputConnection;
use nodi::{timers::Ticker, Event, Moment};
use std::{sync::mpsc, thread};
pub struct MusicGenerator<G>
where
    G: Generator<Item = Vec<Event>> + Send + 'static,
{
    gen_models: Models,
    rx: Option<mpsc::Receiver<()>>,
    gen_list: Vec<Box<G>>,
}

impl<G> MusicGenerator<G>
where
    G: Generator<Item = Vec<Event>> + Send + 'static,
{
    pub fn new() -> Self {
        MusicGenerator {
            gen_models: Models::new(),
            rx: None,
            gen_list: Vec::new(),
        }
    }

    pub fn add_generator(mut self, generator: G) -> Self {
        self.gen_list.push(Box::new(generator));
        self
    }

    pub fn play(
        mut self,
        t: Ticker,
        con: MidiOutputConnection,
        time_signature: u16,
    ) -> (mpsc::Sender<()>, thread::JoinHandle<()>) {
        self.gen_models.time.set_time_signature(time_signature);
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        let mut player = Player::new(self, con, t);
        let handle = thread::spawn(move || player.play());
        (tx, handle)
    }
}

impl<G> Iterator for MusicGenerator<G>
where
    G: Generator<Item = Vec<Event>> + Send + 'static,
{
    type Item = nodi::Moment;

    fn next(&mut self) -> Option<Self::Item> {
        self.gen_models.time.add_ticks(1, 100);
        let mut result: Moment = Moment { events: Vec::new() };

        for generator in &mut self.gen_list {
            for message in (**generator).gen(&mut self.gen_models) {
                result.push(message)
            }
        }
        Some(result)
    }
}
