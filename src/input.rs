pub enum ActionKind {
    Undo,
    Reload,
    Exit,
    Selection,
}
pub enum DirectionKind {
    Up,
    Down,
    Left,
    Right,
}

pub enum InputKind {
    Direction(DirectionKind),
    Action(ActionKind),
}

pub struct InputBuffer {
    pub buffer: Vec<InputKind>,
}

impl Default for InputBuffer {
    fn default() -> InputBuffer {
        InputBuffer::new()
    }
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer { buffer: vec![] }
    }

    pub fn insert(&mut self, input: InputKind) {
        self.buffer.insert(0, input);
    }
}
