use bevy::prelude::*;

pub struct Colors;

impl Colors {
    pub const PRIMARY: Color = Color::Rgba {
        red: 245.0 / u8::MAX as f32,
        green: 210.0 / u8::MAX as f32,
        blue: 70.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const PRIMARY_DARK: Color = Color::Rgba {
        red: 225.0 / u8::MAX as f32,
        green: 190.0 / u8::MAX as f32,
        blue: 50.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const SECONDARY: Color = Color::Rgba {
        red: 108.0 / u8::MAX as f32,
        green: 255.0 / u8::MAX as f32,
        blue: 91.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const LIGHT: Color = Color::Rgba {
        red: 255.0 / u8::MAX as f32,
        green: 255.0 / u8::MAX as f32,
        blue: 255.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const DARK: Color = Color::Rgba {
        red: 0.0 / u8::MAX as f32,
        green: 0.0 / u8::MAX as f32,
        blue: 0.0 / u8::MAX as f32,
        alpha: 1.0,
    };
    pub const TRANSPARENT: Color = Color::Rgba {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 0.0,
    };
}
