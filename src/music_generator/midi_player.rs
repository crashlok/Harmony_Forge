use nodi::{Connection, Event, Moment, Timer};
use rand::seq::IteratorRandom;
use std::rc::Rc;

#[derive(Debug)]
pub struct MidiPlayer<T, C, I>
where
    T: Timer,
    C: Connection,
    I: Iterator<Item = Moment>,
{
    generator: Rc<I>,
    /// An active midi connection.
    pub con: C,
    timer: T,
}

impl<I, C, T> MidiPlayer<T, C, I>
where
    T: Timer,
    C: Connection,
    I: Iterator<Item = Moment>,
{
    pub fn new(generator: I, con: C, timer: T) -> Self {
        MidiPlayer {
            generator: Rc::new(generator),
            con,
            timer,
        }
    }

    pub fn play(&mut self) {
        let mut counter = 0_u32;
        loop {
            let moment: Moment = match self.generator.next() {
                None => return,
                Some(m) => m,
            };
            if !moment.is_empty() {
                self.timer.sleep(counter);
                counter = 0;

                for event in &moment.events {
                    match event {
                        Event::Tempo(val) => self.timer.change_tempo(*val),
                        Event::Midi(msg) => {
                            if !self.con.play(*msg) {
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
