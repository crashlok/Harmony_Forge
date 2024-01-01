use super::Generator;
use crate::{models::Models, player::Player};
use midir::MidiOutputConnection;
use nodi::{timers::Ticker, Event, Moment};
use std::{sync::mpsc, thread};
pub struct MusicGenerator<G>
where
    G: Generator<Item = Vec<Event>> + Send + 'static,
{
    models: Models,
    rx: Option<mpsc::Receiver<()>>,
    gen_list: Vec<Box<G>>,
}

impl<G> MusicGenerator<G>
where
    G: Generator<Item = Vec<Event>> + Send + 'static,
{
    pub fn new() -> Self {
        MusicGenerator {
            models: Models::new(),
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
        self.models.time.set_time_signature(time_signature);
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
        self.models.time.add_ticks(1, 100);

        let (result, new_models) = self.gen_list.iter_mut().fold(
            (Vec::new(), self.models.clone()),
            |(result, input_models), generator| {
                let (mut events, end_models) = (**generator).gen(input_models);
                events.append(&mut result.clone());
                (events, end_models)
            },
        );
        self.models = new_models;
        Some(Moment { events: result })
    }
}
