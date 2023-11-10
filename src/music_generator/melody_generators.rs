use super::{self, midi_massage_event, MelodyGen};

pub struct Random {
    playing: Vec<i32>,
    scale: Vec<i32>,
}

impl Random {
    fn new(scale: Scale, octave_range: range) -> Random {
        let scale_resut: Vec<i32> = vec![];
        let scale: Vec<i32> = scale.as_midi_notes();
        for o in octave_range {
            scale_resut.push(scale.iter().map(|n| octave(n, o)).collect())
        }

        Self {
            playing: vec![],
            scale: scale_resut,
        }
    }
}

impl MelodyGen for Random {
    fn gen(&mut self) -> &[nodi::Event] {
        if self.playing.len() == 0 {
            let n = 1;
            midi_massage_event(midly::MidiMessage::NoteOn { key: i, vel: 1 }, 1);
            self.playing.append(i)
        } else {
            midi_massage_event(
                midly::MidiMessage::NoteOff {
                    key: self.playing[0],
                    vel: 1,
                },
                1,
            );
            self.playing.pop();
        }
    }
}
