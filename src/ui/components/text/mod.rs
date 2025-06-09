mod dynamic;
mod embossed;
mod simple;

pub use dynamic::{DynamicText, DynamicTextData};
pub use embossed::EmbossedText;
pub use simple::SimpleText;

use std::default::Default;

use bevy::prelude::*;

use crate::resources::prelude::*;

pub trait GameText: Default {
    const SIZE_SMALL: f32 = 18.0;
    const SIZE_MEDIUM: f32 = 36.0;
    const SIZE_LARGE: f32 = 90.0;
    const SIZE_EXTRA_LARGE: f32 = 108.0;

    fn small<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        *game_text.get_text_font() = TextFont {
            font: font.clone(),
            font_size: Self::SIZE_SMALL,
            ..default()
        };
        *game_text.get_text() = Text::new(value);
        game_text
    }

    fn medium<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        *game_text.get_text_font() = TextFont {
            font: font.clone(),
            font_size: Self::SIZE_MEDIUM,
            ..default()
        };
        *game_text.get_text() = Text::new(value);
        game_text
    }

    fn large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        *game_text.get_text_font() = TextFont {
            font: font.clone(),
            font_size: Self::SIZE_LARGE,
            ..default()
        };
        *game_text.get_text() = Text::new(value);

        game_text
    }

    fn extra_large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        *game_text.get_text_font() = TextFont {
            font: font.clone(),
            font_size: Self::SIZE_EXTRA_LARGE,
            ..default()
        };
        *game_text.get_text() = Text::new(value);

        game_text
    }

    fn primary(&mut self) -> &mut Self {
        *self.get_text_color() = TextColor(Colors::PRIMARY);
        self
    }

    fn secondary(&mut self) -> &mut Self {
        *self.get_text_color() = TextColor(Colors::SECONDARY);
        self
    }

    fn light(&mut self) -> &mut Self {
        *self.get_text_color() = TextColor(Colors::LIGHT);
        self
    }

    fn get_text_color(&mut self) -> &mut TextColor;

    fn get_text_font(&mut self) -> &mut TextFont;

    fn get_text(&mut self) -> &mut Text;

    fn spawn(self, parent: &mut ChildBuilder);
}
