use super::Input;

pub struct InputBuffer {
    pub buffer: Vec<Input>,
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

    pub fn insert(&mut self, input: Input) {
        self.buffer.insert(0, input);
    }

    pub fn pop(&mut self) -> Option<Input> {
        self.buffer.pop()
    }
}
