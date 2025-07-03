use super::state::LevelState;

const MAX_SNAPSHOTS: usize = 4;

pub struct LevelSnapshots {
    undos: usize,
    inner: [Option<LevelState>; MAX_SNAPSHOTS],
}

impl Default for LevelSnapshots {
    fn default() -> Self {
        Self {
            undos: MAX_SNAPSHOTS,
            inner: [None; MAX_SNAPSHOTS],
        }
    }
}

impl LevelSnapshots {
    pub fn reset(&mut self) {
        *self = LevelSnapshots::default();
    }

    pub fn max_undos_available(&self) -> bool {
        self.undos == MAX_SNAPSHOTS
    }

    pub fn undos_string(&self) -> String {
        self.undos.to_string()
    }

    pub fn shift(&mut self) -> Option<LevelState> {
        if self.undos == 0 || self.inner[0].is_none() {
            return None;
        }

        let state = self.inner[0];
        self.inner.rotate_left(1);
        self.inner[MAX_SNAPSHOTS - 1] = None;
        self.undos -= 1;
        state
    }
    pub fn capture(&mut self, state: LevelState) {
        self.inner.rotate_right(1);
        self.inner[0] = Some(state);
    }
}
