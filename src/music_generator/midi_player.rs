use midir::MidiOutputConnection;
use nodi::{Connection, Event, Moment, Timer};
//#[derive(Debug)]
pub struct MidiPlayer<T, I>
where
    T: Timer,
    I: Iterator<Item = Moment>,
{
    generator: I,
    /// An active midi connection.
    pub con: MidiOutputConnection,
    timer: T,
}

impl<I, T> MidiPlayer<T, I>
where
    T: Timer,
    I: Iterator<Item = Moment>,
{
    pub fn new(generator: I, con: MidiOutputConnection, timer: T) -> Self {
        MidiPlayer {
            generator,
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
                            if !play(self.con, *msg) {
                                panic!()
                            }
                            todo!()
                        }
                        _ => (),
                    };
                }
            }

            counter += 1;
        }
    }
}

fn play(mut con: midir::MidiOutputConnection, msg: nodi::MidiEvent) -> bool {
    let mut buf = Vec::with_capacity(8);
    let _ = msg.write(&mut buf);

    let _ = con.send(&buf);
    true
}
