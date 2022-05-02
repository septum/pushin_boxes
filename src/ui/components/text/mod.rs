mod dynamic;
mod embossed;
mod simple;

use bevy::prelude::*;

pub use dynamic::DynamicText;
pub use embossed::EmbossedText;
pub use simple::SimpleText;

const ALIGNMENT: TextAlignment = TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
};
