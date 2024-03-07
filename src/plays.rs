use harmony_forge::{
    note::{
        chords::Chord,
        Scale,
        octave
    },


    generators::{
        music_generator::MusicGenerator,
        note_generator::{NearNotes, NotesDependingBar, OneNote, RandomNotes},
        pattern_generator::{OnBeatPattern, OnClosurePattern},
        universal_generator::MultipleClosure,
    }
};
pub fn random() -> MusicGenerator {
    MusicGenerator::new()
        .add_generator(Box::new(OnBeatPattern::new(
               OneNote::new(vec![62])
            , 1)
        ))
        .add_generator(Box::new(OnClosurePattern::new(
            |models|models.time.on_eight() && models.time.get_eights_i32() % 3 == 1 ,
            RandomNotes::new(Scale::new_major(60), 0..1),
            2)
        ))
}
pub fn piece() -> MusicGenerator {
    MusicGenerator::new()
        .add_generator(Box::new(MultipleClosure::new(
            |models| match models.time.get_bars() {
                0 => 1,
                _ => 1,
            },
            vec![
                Box::new(OnClosurePattern::new(
                    |models| models.time.on_eight() && models.time.get_eights_i32() % 3 == 0,
                    NotesDependingBar::new(vec![
                        Chord::new_major(56).as_midi_notes(),
                        Chord::new_minor(53).as_midi_notes(),
                        Chord::new_minor(48).as_midi_notes(),
                        Chord::new_minor(48).as_midi_notes(),
                    ]),
                    0,
                )),
                Box::new(OnBeatPattern::new(
                    NotesDependingBar::new(vec![
                        Chord::new_major(56).as_midi_notes(),
                        Chord::new_minor(53).as_midi_notes(),
                        Chord::new_minor(48).as_midi_notes(),
                        Chord::new_minor(48).as_midi_notes(),
                    ]),
                    0,
                )),
            ],
        )))
        .add_generator(Box::new(MultipleClosure::new(
            |models| (models.time.get_bars() % 3) as usize,
            vec![
                Box::new(OnClosurePattern::new(
                    |models| {
                        models.time.on_eight()
                            && !models.time.on_quarter()
                            && models.time.get_eights_i32() % 3 == 0
                    },
                    OneNote::new(vec![61]),
                    1,
                )),
                Box::new(OnClosurePattern::new(
                    |models| models.time.on_eight() && !models.time.on_quarter(),
                    OneNote::new(vec![61]),
                    1,
                )),
            ],
        )))
        .add_generator(Box::new(OnClosurePattern::new(
            |models| models.time.on_eight(),
            NearNotes::new(Scale::new_minor(60), 1..3),
            2,
        )))
        .add_generator(Box::new(OnClosurePattern::new(
            |models| models.time.on_bar(),
            NotesDependingBar::new(vec![
                vec![octave(Chord::new_major(56).as_midi_notes()[0], -1)],
                vec![octave(Chord::new_major(53).as_midi_notes()[0], -1)],
                vec![octave(Chord::new_major(48).as_midi_notes()[0], -1)],
                vec![octave(Chord::new_major(48).as_midi_notes()[0], -1)],
            ]),
            3,
        )))
}
