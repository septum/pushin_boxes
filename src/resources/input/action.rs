pub enum ActionInput {
    Undo,
    Reload,
    Select,
    Toggle,
    Exit,
}

pub struct ActionInputEvent {
    pub value: ActionInput,
}

impl ActionInputEvent {
    pub fn undo() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Undo,
        }
    }

    pub fn reload() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Reload,
        }
    }

    pub fn select() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Select,
        }
    }

    pub fn toggle() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Toggle,
        }
    }

    pub fn exit() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Exit,
        }
    }
}
