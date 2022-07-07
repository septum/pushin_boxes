use bevy::prelude::*;

pub fn update_dynamic_text(text: &mut Text, value: String) {
    text.sections[1].value = value;
}
