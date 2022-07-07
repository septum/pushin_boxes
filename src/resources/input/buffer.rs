use super::GameInput;

pub struct GameInputBuffer {
    pub buffer: Vec<GameInput>,
}

impl Default for GameInputBuffer {
    fn default() -> GameInputBuffer {
        GameInputBuffer::new()
    }
}

impl GameInputBuffer {
    #[must_use] pub fn new() -> GameInputBuffer {
        GameInputBuffer { buffer: vec![] }
    }

    pub fn insert(&mut self, input: GameInput) {
        self.buffer.insert(0, input);
    }

    pub fn pop(&mut self) -> Option<GameInput> {
        self.buffer.pop()
    }
}
