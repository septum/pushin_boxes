use bevy::prelude::*;

#[derive(Default)]
pub struct Picture {
    node: ImageNode,
}

impl Picture {
    pub fn new(image: &Handle<Image>) -> Picture {
        let mut picture = Self::default();
        picture.node.image = image.clone().into();
        picture
    }

    pub fn spawn(self, parent: &mut ChildSpawnerCommands) {
        parent.spawn(self.node);
    }
}
