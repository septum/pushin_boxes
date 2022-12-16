use super::kind::LevelKind;

pub struct LevelInsertionEvent {
    pub kind: LevelKind,
}

impl LevelInsertionEvent {
    pub fn new(kind: LevelKind) -> LevelInsertionEvent {
        LevelInsertionEvent { kind }
    }
}
