use bevy::prelude::*;

use crate::resources::prelude::*;

pub struct Housing {
    pub bundle: NodeBundle,
}

impl Default for Housing {
    fn default() -> Housing {
        Housing {
            bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Colors::TRANSPARENT.into(),
                ..Default::default()
            },
        }
    }
}

impl Housing {
    #[must_use]
    pub fn new(width: Val, height: Val) -> Housing {
        let mut housing = Housing::default();
        housing.bundle.style.size.width = width;
        housing.bundle.style.size.height = height;
        housing
    }

    #[must_use]
    pub fn full() -> Housing {
        Housing::default()
    }

    #[must_use]
    pub fn percent(width: f32, height: f32) -> Housing {
        let mut housing = Housing::default();
        housing.bundle.style.size.width = Val::Percent(width);
        housing.bundle.style.size.height = Val::Percent(height);
        housing
    }

    #[must_use]
    pub fn px(width: f32, height: f32) -> Housing {
        let mut housing = Housing::default();
        housing.bundle.style.size.width = Val::Px(width);
        housing.bundle.style.size.height = Val::Px(height);
        housing
    }

    pub fn width(&mut self, width: Val) -> &mut Housing {
        self.bundle.style.size.width = width;
        self
    }

    pub fn height(&mut self, height: Val) -> &mut Housing {
        self.bundle.style.size.height = height;
        self
    }

    pub fn position_type(&mut self, position_type: PositionType) -> &mut Housing {
        self.bundle.style.position_type = position_type;
        self
    }

    pub fn left_position(&mut self, position: Val) -> &mut Housing {
        self.bundle.style.position.left = position;
        self
    }

    pub fn image(&mut self, image: &Handle<Image>) -> &mut Housing {
        self.bundle.image = image.clone().into();
        self
    }

    pub fn flex_direction(&mut self, flex_direction: FlexDirection) -> &mut Housing {
        self.bundle.style.flex_direction = flex_direction;
        self
    }

    pub fn justify_content(&mut self, justify_content: JustifyContent) -> &mut Housing {
        self.bundle.style.justify_content = justify_content;
        self
    }

    pub fn align_items(&mut self, align_items: AlignItems) -> &mut Housing {
        self.bundle.style.align_items = align_items;
        self
    }

    pub fn left_padding(&mut self, padding: Val) -> &mut Housing {
        self.bundle.style.padding.left = padding;
        self
    }

    pub fn right_padding(&mut self, padding: Val) -> &mut Housing {
        self.bundle.style.padding.right = padding;
        self
    }

    pub fn flex_wrap(&mut self, flex_wrap: FlexWrap) -> &mut Housing {
        self.bundle.style.flex_wrap = flex_wrap;
        self
    }

    pub fn align_content(&mut self, align_content: AlignContent) -> &mut Housing {
        self.bundle.style.align_content = align_content;
        self
    }

    pub fn top(&mut self, top: f32) -> &mut Housing {
        self.bundle.style.position.top = Val::Px(top);
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
        parent.spawn_bundle(self.bundle).with_children(children);
    }
}
