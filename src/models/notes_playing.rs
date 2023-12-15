use midly::num::u7;

pub struct NotesPlaying(Vec<u7>);

impl NotesPlaying {
    pub fn new() -> Self {
        NotesPlaying(Vec::new())
    }
    pub fn started(&mut self, note: u7) -> &Self {
        self.0.push(note);
        self
    }
    pub fn stopped(&mut self, note: u7) -> Option<&Self> {
        self.0
            .remove(self.0.iter().enumerate().find(|(_nr, x)| **x == note)?.0);
        Some(self)
    }
    pub fn get_notesplaying(&self) -> &[u7] {
        &self.0
    }
}

impl Default for NotesPlaying {
    fn default() -> Self {
        Self::new()
    }
}
