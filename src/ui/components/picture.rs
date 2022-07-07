use bevy::prelude::*;

pub struct Picture {
    pub bundle: NodeBundle,
}

impl Default for Picture {
    fn default() -> Picture {
        Picture {
            bundle: NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

impl Picture {
    #[must_use]
    pub fn new(width: Val, height: Val, image: &Handle<Image>) -> Picture {
        let mut picture = Picture::default();
        picture.bundle.style.size.width = width;
        picture.bundle.style.size.height = height;
        picture.bundle.image = image.clone().into();
        picture
    }

    #[must_use]
    pub fn percent(width: f32, height: f32, image: &Handle<Image>) -> Picture {
        let mut picture = Picture::default();
        picture.bundle.style.size.width = Val::Percent(width);
        picture.bundle.style.size.height = Val::Percent(height);
        picture.bundle.image = image.clone().into();
        picture
    }

    #[must_use]
    pub fn px(width: f32, height: f32, image: &Handle<Image>) -> Picture {
        let mut picture = Picture::default();
        picture.bundle.style.size.width = Val::Px(width);
        picture.bundle.style.size.height = Val::Px(height);
        picture.bundle.image = image.clone().into();
        picture
    }

    #[must_use]
    pub fn full(image: &Handle<Image>) -> Picture {
        let mut picture = Picture::default();
        picture.bundle.image = image.clone().into();
        picture
    }

    pub fn width(&mut self, width: Val) -> &mut Picture {
        self.bundle.style.size.width = width;
        self
    }

    pub fn height(&mut self, height: Val) -> &mut Picture {
        self.bundle.style.size.height = height;
        self
    }

    pub fn position_type(&mut self, position_type: PositionType) -> &mut Picture {
        self.bundle.style.position_type = position_type;
        self
    }

    pub fn left_position(&mut self, position: Val) -> &mut Picture {
        self.bundle.style.position.left = position;
        self
    }

    pub fn right_position(&mut self, position: Val) -> &mut Picture {
        self.bundle.style.position.right = position;
        self
    }

    pub fn image(&mut self, image: &Handle<Image>) -> &mut Picture {
        self.bundle.image = image.clone().into();
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn_bundle(self.bundle);
    }
}
