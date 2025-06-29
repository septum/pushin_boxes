mod fonts;
mod images;
mod plugin;
mod sounds;
mod systems;

pub mod prelude {
    pub use super::fonts::Fonts;
    pub use super::images::{
        BLINK_ROW_LAST_FRAME_INDEX, CharacterAnimation, CharacterMarker, Images,
    };
    pub use super::sounds::{Music, Sfx, Sounds};
    pub use super::systems::cleanup;
}

pub use plugin::Plugin;
