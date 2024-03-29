use self::{notes_playing::NotesPlaying, time_models::MusicTime};

pub mod notes_playing;
pub mod parameter;
pub mod time_models;

#[derive(Debug, Clone)]
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

impl Default for Models {
    fn default() -> Self {
        Self::new()
    }
}
