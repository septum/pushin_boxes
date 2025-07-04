#[derive(Clone, Copy)]
pub enum Action {
    Undo,
    Reload,
    Select,
    Toggle,
    Delete,
    Exit,
}
