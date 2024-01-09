use super::Gen;
use crate::{models::Models, player::Player};
use midir::MidiOutputConnection;
use nodi::{timers::Ticker, Event, Moment};
use std::{sync::mpsc, thread};

pub struct MusicGeneratorAs {
    models: Models,
    rx: Option<mpsc::Receiver<()>>,
    sender: crossbeam_channel::Sender<Moment>,
    gen_list: Vec<Box<Gen<Vec<Event>>>>,
}

impl MusicGeneratorAs {
    pub fn new() -> Self {
        MusicGenerator {
            models: Models::new(),
            rx: None,
            gen_list: Vec::new(),
        }
    }

    pub fn add_generator(mut self, generator: Box<Gen<Vec<Event>>>) -> Self {
        self.gen_list.push(generator);
        self
    }

    pub fn play(
        mut self,
        t: Ticker,
        con: MidiOutputConnection,
        time_signature: u16,
    ) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
        self.models.time.set_time_signature(time_signature);
        let (tx, rx) = crossbeam_channel::bounded(3);
        self.Sender = tx;
        let mut player = Player::new(rx.iter(), con, t);
        let handle = thread::spawn(move || player.play());
        let handle2 = thread::spawn(move || loop {
            self.sender.send(self.next().unwrap)
        });
        (handle, handle2)
    }
}

impl Iterator for MusicGenerator {
    type Item = nodi::Moment;

    fn next(&mut self) -> Option<Self::Item> {
        self.models.time.add_ticks(1, 100);

        let (result, new_models) = self.gen_list.iter_mut().fold(
            (Vec::new(), self.models.clone()),
            |(mut result, input_models), generator| {
                let (mut events, end_models) = (**generator).gen(input_models);
                events.append(&mut result);
                (events, end_models)
            },
        );
        self.models = new_models;
        Some(Moment { events: result })
    }
}

impl Default for MusicGeneratorAs {
    fn default() -> Self {
        Self::new()
    }
}
