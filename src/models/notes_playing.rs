use super::time_models::MusicTime;
use midly::num::{u4, u7};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NotesPlaying(HashMap<u4, Vec<(u7, MusicTime)>>);

impl NotesPlaying {
    pub fn new() -> Self {
        NotesPlaying(HashMap::new())
    }
    pub fn start_multiple(&mut self, notes: Vec<u7>, at: MusicTime, channel: u4) -> Vec<u7> {
        notes
            .iter()
            .map(|n| self.started(*n, at, channel))
            .collect()
    }

    pub fn started(&mut self, note: u7, at: MusicTime, channel: u4) -> u7 {
        self.get_mut_channel_or_create(channel).push((note, at));
        note
    }

    fn create_new_channel(&mut self, channel: u4) -> &mut Vec<(u7, MusicTime)> {
        match self.0.insert(channel, Vec::new()) {
            None => self.0.get_mut(&channel).unwrap(),
            Some(_) => panic!("wanted to create channel that already existed"),
        }
    }

    fn get_mut_channel_or_create(&mut self, channel: u4) -> &mut Vec<(u7, MusicTime)> {
        match self.0.get(&channel) {
            None => self.create_new_channel(channel),
            Some(_) => self.0.get_mut(&channel).unwrap(),
        }
    }
    pub fn stop_all(&mut self, channel: u4) -> Vec<u7> {
        let res = self
            .get_mut_channel_or_create(channel)
            .iter()
            .map(|l| l.0)
            .collect::<Vec<u7>>();
        res.iter().for_each(|n| {
            self.stopped(*n, channel);
        });
        res
    }

    pub fn stopped(&mut self, note: u7, channel: u4) -> Option<&Self> {
        let channel = self.get_mut_channel_or_create(channel);
        channel.remove(channel.iter().enumerate().find(|(_nr, x)| x.0 == note)?.0);
        Some(self)
    }
    pub fn get_all_notesplaying(&self) -> &HashMap<u4, Vec<(u7, MusicTime)>> {
        &self.0
    }
}

impl Default for NotesPlaying {
    fn default() -> Self {
        Self::new()
    }
}
