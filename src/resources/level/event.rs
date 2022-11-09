use super::tag::LevelTag;

pub struct LevelInsertionEvent {
    pub tag: LevelTag,
}

impl LevelInsertionEvent {
    pub fn new(tag: LevelTag) -> LevelInsertionEvent {
        LevelInsertionEvent { tag }
    }
}
