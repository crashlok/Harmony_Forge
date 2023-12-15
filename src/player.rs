use midir::MidiOutputConnection;
use nodi::{Event, Moment, Timer};

use crate::{generators::Generator, models::notes_playing::NotesPlaying, timers::TickerWithTime};

#[derive()]
pub struct Player<I>
where
    I: Generator<Item = Moment>,
{
    generator: Box<I>,
    pub con: MidiOutputConnection,
    timer: TickerWithTime,
    notes_playing: NotesPlaying,
}

impl<I> Player<I>
where
    I: Generator<Item = Moment>,
{
    pub fn new(generator: I, con: MidiOutputConnection, timer: TickerWithTime) -> Self {
        Player {
            generator: Box::new(generator),
            con,
            timer,
            notes_playing: NotesPlaying::new(),
        }
    }

    pub fn play(&mut self) {
        let mut counter = 0_u32;
        loop {
            let moment: Moment = (*self.generator).gen(&mut (
                self.timer
                    .get_time()
                    .expect("didn't initialize tickers time signature"),
                &mut self.notes_playing,
            ));

            if !moment.is_empty() {
                self.timer.sleep(counter);
                counter = 0;

                for event in &moment.events {
                    match event {
                        Event::Tempo(val) => self.timer.change_tempo(*val),
                        Event::Midi(msg) => {
                            if !play(&mut self.con, *msg) {
                                panic!()
                            }
                        }
                        _ => (),
                    };
                }
            }

            counter += 1;
        }
    }
}

fn play(con: &mut midir::MidiOutputConnection, msg: nodi::MidiEvent) -> bool {
    let mut buf = Vec::with_capacity(8);
    let _ = msg.write(&mut buf);

    let _ = con.send(&buf);
    true
}
