use midi_player::MidiPlayer;
use midir::MidiOutputConnection;
use midly::num::u7;
use nodi::{timers::Ticker, Event, Moment};
use std::{sync::mpsc, thread};

mod midi_player;
pub mod note_generator;
pub mod pattern_generators;

//#[derive(Debug)]
pub struct MusicGenerator<C, M>
where
    C: PatternGenerator + Send + 'static,
    M: PatternGenerator + Send + 'static,
{
    rx: Option<mpsc::Receiver<()>>,
    melody_gen: M,
    chord_gen: C,
}

impl<C, M> MusicGenerator<C, M>
where
    C: PatternGenerator + Send + 'static,
    M: PatternGenerator + Send + 'static,
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
        t: Ticker,
        con: MidiOutputConnection,
    ) -> (mpsc::Sender<()>, thread::JoinHandle<()>) {
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        let mut midi_player = MidiPlayer::new(self, con, t);
        let handle = thread::spawn(move || midi_player.play());
        (tx, handle)
    }
}

impl<C, M> Iterator for MusicGenerator<C, M>
where
    C: PatternGenerator + Send + 'static,
    M: PatternGenerator + Send + 'static,
{
    type Item = nodi::Moment;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Moment = Moment { events: Vec::new() };

        for message in self.melody_gen.gen() {
            result.push(message)
        }
        for message in self.chord_gen.gen() {
            result.push(message)
        }
        Some(result)
    }
}

pub trait PatternGenerator {
    fn gen(&mut self) -> Vec<Event>;
}

pub trait NoteGenerator {
    fn gen(&mut self) -> Vec<u7>;
}

fn midi_massage_event(message: midly::MidiMessage, channel: midly::num::u4) -> nodi::Event {
    nodi::Event::Midi(nodi::MidiEvent { channel, message })
}
