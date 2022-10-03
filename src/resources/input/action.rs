pub enum Action {
    Undo,
    Reload,
    Selection,
    Pick,
    Volume,
    Exit,
}

pub struct ActionEvent {
    pub value: Action,
}

impl ActionEvent {
    pub fn undo() -> ActionEvent {
        ActionEvent {
            value: Action::Undo,
        }
    }

    pub fn reload() -> ActionEvent {
        ActionEvent {
            value: Action::Reload,
        }
    }

    pub fn selection() -> ActionEvent {
        ActionEvent {
            value: Action::Selection,
        }
    }

    pub fn pick() -> ActionEvent {
        ActionEvent {
            value: Action::Pick,
        }
    }

    pub fn volume() -> ActionEvent {
        ActionEvent {
            value: Action::Volume,
        }
    }

    pub fn exit() -> ActionEvent {
        ActionEvent {
            value: Action::Exit,
        }
    }
}
