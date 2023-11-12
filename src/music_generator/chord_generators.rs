use super::ChordGen;

pub struct Test {}

impl ChordGen for Test {
    fn gen(&mut self) -> Vec<nodi::Event> {
        Vec::new()
    }
}
