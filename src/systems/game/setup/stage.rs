use bevy::{color::palettes::css::*, prelude::*};

pub fn place_stage(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: LIGHT_CYAN.into(),
            ..default()
        })
        .with_children(|parent| {
            // 三行布局：对面、中、自己。其中中间的包括左边、中桌、右边
            place_north_spot(parent);
            place_center_line(parent);
            place_south_spot(parent);
        });
}

fn place_north_spot(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: PURPLE.into(),
                    ..default()
                });
        });
}
// 中间的包括左边、中桌、右边
fn place_center_line(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(40.0),
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                // flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: GREEN_YELLOW.into(),
                    ..default()
                });
                
                parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: WHITE.into(),
                    ..default()
                });
                
                parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: PINK.into(),
                    ..default()
                });
        });
}

fn place_south_spot(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            // background_color: DARK_ORANGE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: LIGHT_CORAL.into(),
                    ..default()
                });
        });
}
