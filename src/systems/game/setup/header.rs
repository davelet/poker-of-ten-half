use bevy::{color::palettes::css::*, prelude::*};

use crate::{constants::*, HanTextStyle};
use crate::components::prelude::*;


pub fn place_header(parent: &mut ChildBuilder) {
    let style = HanTextStyle::default()
        .with_color(bevy::prelude::Color::Srgba(BLACK))
        .with_font_size(30.0)
        .get_style();
    parent
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                ..default()
            },
            background_color: DARK_ORANGE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(20.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: LIGHT_BLUE.into(),
                        ..Default::default()
                    },
                    ButtonOnGamePage::BackMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        RETURN_TO_MENU_BUTTON_TEXT,
                        style.clone(),
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(60.0),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: YELLOW.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section("双人对局", style.clone()),));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(20.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            // margin: UiRect::top(Val::Px(20.0)),
                            ..Default::default()
                        },
                        background_color: LIGHT_GRAY.into(),
                        ..Default::default()
                    },
                    ButtonOnGamePage::ExitGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(EXIT_BUTTON_TEXT, style.clone()));
                });
        });
}
