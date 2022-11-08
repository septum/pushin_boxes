pub enum ActionInput {
    Undo,
    Reload,
    Selection,
    Pick,
    Volume,
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

    pub fn selection() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Selection,
        }
    }

    pub fn pick() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Pick,
        }
    }

    pub fn volume() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Volume,
        }
    }

    pub fn exit() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Exit,
        }
    }
}
