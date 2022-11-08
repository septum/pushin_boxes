mod dynamic;
mod embossed;
mod simple;

pub use dynamic::{DynamicText, DynamicTextMarker};
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

        game_text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_SMALL;
        });

        game_text
    }

    fn medium<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        game_text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_MEDIUM;
        });

        game_text
    }

    fn large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        game_text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_LARGE;
        });

        game_text
    }

    fn extra_large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        game_text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_EXTRA_LARGE;
        });

        game_text
    }

    fn for_each_section(&mut self, f: impl FnMut(&mut TextSection)) {
        self.text_bundle().text.sections.iter_mut().for_each(f);
    }

    fn primary(&mut self) -> &mut Self {
        self.for_each_section(|section| section.style.color = Colors::PRIMARY);
        self
    }

    fn secondary(&mut self) -> &mut Self {
        self.for_each_section(|section| section.style.color = Colors::SECONDARY);
        self
    }

    fn light(&mut self) -> &mut Self {
        self.for_each_section(|section| section.style.color = Colors::LIGHT);
        self
    }

    fn text_bundle(&mut self) -> &mut TextBundle;

    fn spawn(self, parent: &mut ChildBuilder);
}
