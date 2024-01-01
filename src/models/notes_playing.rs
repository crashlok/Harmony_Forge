use midly::num::u7;

use super::time_models::MusicTime;

#[derive(Debug)]
pub struct NotesPlaying(Vec<(u7, MusicTime)>);

impl NotesPlaying {
    pub fn new() -> Self {
        NotesPlaying(Vec::new())
    }
    pub fn started(&mut self, note: u7, at: MusicTime) -> &Self {
        self.0.push((note, at));
        self
    }

    pub fn stop_all(&mut self) -> Vec<u7> {
        let res = self.0.iter().map(|l| l.0).collect::<Vec<u7>>();
        res.iter().for_each(|n| {
            self.stopped(*n);
        });
        res
    }

    pub fn stopped(&mut self, note: u7) -> Option<&Self> {
        self.0
            .remove(self.0.iter().enumerate().find(|(_nr, x)| x.0 == note)?.0);
        Some(self)
    }
    pub fn get_notesplaying(&self) -> &[(u7, MusicTime)] {
        &self.0
    }
}

impl Default for NotesPlaying {
    fn default() -> Self {
        Self::new()
    }
}
