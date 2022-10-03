use bevy::prelude::*;

use crate::resources::prelude::*;

pub struct Container {
    bundle: NodeBundle,
}

impl Default for Container {
    fn default() -> Container {
        Container {
            bundle: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Colors::TRANSPARENT.into(),
                ..default()
            },
        }
    }
}

impl Container {
    pub fn size(width: f32, height: f32) -> Container {
        let mut housing = Self::default();
        housing.bundle.style.size.width = Val::Px(width);
        housing.bundle.style.size.height = Val::Px(height);
        housing
    }

    pub fn size_percentage(width: f32, height: f32) -> Container {
        let mut housing = Self::default();
        housing.bundle.style.size.width = Val::Percent(width);
        housing.bundle.style.size.height = Val::Percent(height);
        housing
    }

    pub fn auto_height() -> Container {
        let mut housing = Self::default();
        housing.bundle.style.size.height = Val::Auto;
        housing
    }

    pub fn auto_height_with_width(width: f32) -> Container {
        let mut housing = Self::default();
        housing.bundle.style.size.width = Val::Px(width);
        housing.bundle.style.size.height = Val::Auto;
        housing
    }

    pub fn auto() -> Container {
        let mut housing = Self::default();
        housing.bundle.style.size.width = Val::Auto;
        housing.bundle.style.size.height = Val::Auto;
        housing
    }

    pub fn half() -> Container {
        let mut housing = Self::default();
        housing.bundle.style.size.width = Val::Percent(50.0);
        housing
    }

    pub fn height(height: f32) -> Container {
        let mut housing = Self::default();
        housing.bundle.style.size.height = Val::Px(height);
        housing
    }

    pub fn absolute(&mut self) -> &mut Container {
        self.bundle.style.position_type = PositionType::Absolute;
        self
    }

    pub fn row(&mut self) -> &mut Container {
        self.bundle.style.flex_direction = FlexDirection::Row;
        self
    }

    pub fn justify_between(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::SpaceBetween;
        self
    }

    pub fn justify_around(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::SpaceAround;
        self
    }

    pub fn justify_start(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::FlexStart;
        self
    }

    pub fn justify_end(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::FlexEnd;
        self
    }

    pub fn items_start(&mut self) -> &mut Container {
        self.bundle.style.align_items = AlignItems::FlexStart;
        self
    }

    pub fn content_start(&mut self) -> &mut Container {
        self.bundle.style.align_content = AlignContent::FlexStart;
        self
    }

    pub fn wrap_reverse(&mut self) -> &mut Container {
        self.bundle.style.flex_wrap = FlexWrap::WrapReverse;
        self
    }

    pub fn align_start(&mut self) -> &mut Container {
        self.bundle.style.align_items = AlignItems::FlexStart;
        self
    }

    pub fn align_end(&mut self) -> &mut Container {
        self.bundle.style.align_items = AlignItems::FlexEnd;
        self
    }

    pub fn margin_bottom(&mut self, bottom: f32) -> &mut Container {
        self.bundle.style.margin.bottom = Val::Px(bottom);
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
        parent.spawn_bundle(self.bundle).with_children(children);
    }
}
