use bevy::prelude::*;

pub enum BrushSprite {
    Box,
    Player,
    Wall,
    Floor,
    Zone,
}

#[derive(Component)]
pub struct Brush(BrushSprite);

impl Default for Brush {
    fn default() -> Brush {
        Brush(BrushSprite::Box)
    }
}

impl Brush {
    #[must_use]
    pub fn current_sprite(&self) -> &BrushSprite {
        &self.0
    }

    pub fn rotate_sprite(&mut self, forward: bool) {
        if forward {
            self.next_sprite();
        } else {
            self.prev_sprite();
        }
    }

    pub fn next_sprite(&mut self) {
        let sprite = match self.current_sprite() {
            BrushSprite::Box => BrushSprite::Player,
            BrushSprite::Player => BrushSprite::Wall,
            BrushSprite::Wall => BrushSprite::Floor,
            BrushSprite::Floor => BrushSprite::Zone,
            BrushSprite::Zone => BrushSprite::Box,
        };
        self.0 = sprite;
    }

    pub fn prev_sprite(&mut self) {
        let sprite = match self.current_sprite() {
            BrushSprite::Box => BrushSprite::Zone,
            BrushSprite::Player => BrushSprite::Box,
            BrushSprite::Wall => BrushSprite::Player,
            BrushSprite::Floor => BrushSprite::Wall,
            BrushSprite::Zone => BrushSprite::Floor,
        };
        self.0 = sprite;
    }
}
