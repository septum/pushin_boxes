mod dynamic;
mod embossed;
mod simple;

pub use dynamic::{DynamicText, DynamicTextData};
pub use embossed::EmbossedText;
pub use simple::SimpleText;

use std::default::Default;

use bevy::{
    prelude::*,
    text::{FontSmoothing, LineHeight},
};

use crate::Colors;

pub trait GameText: Default {
    const SIZE_SMALL: f32 = 20.0;
    const SIZE_MEDIUM: f32 = 40.0;
    const SIZE_LARGE: f32 = 100.0;
    const SIZE_EXTRA_LARGE: f32 = 120.0;

    fn small<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        *game_text.get_text_font() = TextFont {
            font: font.clone(),
            font_size: Self::SIZE_SMALL,
            line_height: LineHeight::RelativeToFont(1.),
            font_smoothing: FontSmoothing::None,
        };
        *game_text.get_text() = Text::new(value);
        game_text
    }

    fn medium<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        *game_text.get_text_font() = TextFont {
            font: font.clone(),
            font_size: Self::SIZE_MEDIUM,
            line_height: LineHeight::RelativeToFont(1.),
            font_smoothing: FontSmoothing::None,
        };
        *game_text.get_text() = Text::new(value);
        game_text
    }

    fn large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        *game_text.get_text_font() = TextFont {
            font: font.clone(),
            font_size: Self::SIZE_LARGE,
            line_height: LineHeight::RelativeToFont(1.),
            font_smoothing: FontSmoothing::None,
        };
        *game_text.get_text() = Text::new(value);

        game_text
    }

    fn extra_large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        *game_text.get_text_font() = TextFont {
            font: font.clone(),
            font_size: Self::SIZE_EXTRA_LARGE,
            line_height: LineHeight::RelativeToFont(1.),
            font_smoothing: FontSmoothing::None,
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

    fn spawn(self, parent: &mut ChildSpawnerCommands);
}
