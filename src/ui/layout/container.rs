use bevy::prelude::*;

use crate::resources::prelude::*;

pub struct Container {
    node: Node,
    background_color: BackgroundColor,
}

impl Default for Container {
    fn default() -> Container {
        Container {
            node: Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Colors::TRANSPARENT),
        }
    }
}

impl Container {
    pub fn size(width: f32, height: f32) -> Container {
        let mut housing = Self::default();
        housing.node.width = Val::Px(width);
        housing.node.height = Val::Px(height);
        housing
    }

    pub fn size_percentage(width: f32, height: f32) -> Container {
        let mut housing = Self::default();
        housing.node.width = Val::Percent(width);
        housing.node.height = Val::Percent(height);
        housing
    }

    pub fn auto_height() -> Container {
        let mut housing = Self::default();
        housing.node.height = Val::Auto;
        housing
    }

    pub fn auto_height_with_width(width: f32) -> Container {
        let mut housing = Self::default();
        housing.node.width = Val::Px(width);
        housing.node.height = Val::Auto;
        housing
    }

    pub fn auto() -> Container {
        let mut housing = Self::default();
        housing.node.width = Val::Auto;
        housing.node.height = Val::Auto;
        housing
    }

    pub fn half() -> Container {
        let mut housing = Self::default();
        housing.node.width = Val::Percent(50.0);
        housing
    }

    pub fn height(height: f32) -> Container {
        let mut housing = Self::default();
        housing.node.height = Val::Px(height);
        housing
    }

    pub fn absolute(&mut self) -> &mut Container {
        self.node.position_type = PositionType::Absolute;
        self
    }

    pub fn row(&mut self) -> &mut Container {
        self.node.flex_direction = FlexDirection::Row;
        self
    }

    pub fn justify_between(&mut self) -> &mut Container {
        self.node.justify_content = JustifyContent::SpaceBetween;
        self
    }

    pub fn justify_around(&mut self) -> &mut Container {
        self.node.justify_content = JustifyContent::SpaceAround;
        self
    }

    pub fn justify_start(&mut self) -> &mut Container {
        self.node.justify_content = JustifyContent::FlexStart;
        self
    }

    pub fn justify_end(&mut self) -> &mut Container {
        self.node.justify_content = JustifyContent::FlexEnd;
        self
    }

    pub fn items_start(&mut self) -> &mut Container {
        self.node.align_items = AlignItems::FlexStart;
        self
    }

    pub fn content_start(&mut self) -> &mut Container {
        self.node.align_content = AlignContent::FlexStart;
        self
    }

    pub fn wrap(&mut self) -> &mut Container {
        self.node.flex_wrap = FlexWrap::Wrap;
        self
    }

    pub fn align_start(&mut self) -> &mut Container {
        self.node.align_items = AlignItems::FlexStart;
        self
    }

    pub fn align_end(&mut self) -> &mut Container {
        self.node.align_items = AlignItems::FlexEnd;
        self
    }

    pub fn margin_bottom(&mut self, bottom: f32) -> &mut Container {
        self.node.margin.bottom = Val::Px(bottom);
        self
    }

    pub fn spawn(self, parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
        parent
            .spawn((self.node, self.background_color))
            .with_children(children);
    }
}
