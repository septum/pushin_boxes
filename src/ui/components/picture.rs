use bevy::prelude::*;

pub struct Picture {
    bundle: NodeBundle,
}

impl Default for Picture {
    fn default() -> Picture {
        Picture {
            bundle: NodeBundle { ..default() },
        }
    }
}

impl Picture {
    pub fn new(width: f32, height: f32, image: &Handle<Image>) -> Picture {
        let mut picture = Self::default();
        picture.bundle.style.size.width = Val::Px(width);
        picture.bundle.style.size.height = Val::Px(height);
        picture.bundle.image = image.clone().into();
        picture
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.bundle);
    }
}
