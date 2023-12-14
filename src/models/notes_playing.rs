use midly::num::u7;

pub struct NotesPlaying(Vec<u7>);

impl NotesPlaying {
    fn new() -> Self {
        NotesPlaying(Vec::new())
    }
    fn started(note:u7) -> &Self{
        self.0.push(note)
    }
    fn stopped(note:u7)->
}
