#[derive(Clone)]
pub enum LevelTag {
    Stock(usize),
}

impl Default for LevelTag {
    fn default() -> Self {
        LevelTag::Stock(0)
    }
}
