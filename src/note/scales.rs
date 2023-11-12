use super::{Scale, Step};

impl Scale {
    pub fn new_major(key_note: i32) -> Scale {
        Scale {
            key_note,
            steps: vec![
                Step::Normal(1),
                Step::Major(2),
                Step::Major(3),
                Step::Minor(4),
                Step::Normal(5),
                Step::Major(6),
                Step::Major(7),
                Step::Normal(8),
            ],
        }
    }

    pub fn new_minor(key_note: i32) -> Scale {
        Scale {
            key_note,
            steps: vec![
                Step::Normal(1),
                Step::Major(2),
                Step::Minor(3),
                Step::Minor(4),
                Step::Normal(5),
                Step::Minor(6),
                Step::Minor(7),
                Step::Normal(8),
            ],
        }
    }
}
