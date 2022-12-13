pub enum ActionInput {
    Undo,
    Reload,
    Selection,
    Pick,
    Playtest,
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

    pub fn playtest() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Playtest,
        }
    }

    pub fn exit() -> ActionInputEvent {
        ActionInputEvent {
            value: ActionInput::Exit,
        }
    }
}
