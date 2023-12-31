use self::{notes_playing::NotesPlaying, time::MusicTime};

pub mod notes_playing;
pub mod parameter;
pub mod time;

#[derive(Debug)]
pub struct Models {
    pub time: MusicTime,
    pub playing: NotesPlaying,
}

impl Models {
    pub fn new() -> Self {
        Models {
            time: MusicTime::default(),
            playing: NotesPlaying::default(),
        }
    }
}
