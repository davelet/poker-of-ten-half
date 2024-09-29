use bevy::{color::palettes::css::*, math::bool, prelude::*};

use crate::{components::prelude::*, constants::*, HanTextStyle};

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
            place_north_line(parent);
            place_center_line(parent);
            place_south_line(parent);
        });
}

fn place_north_line(parent: &mut ChildBuilder) {
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
            spawn_player(parent, PURPLE, FlexDirection::Column, false);
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
            spawn_player(parent, GREEN_YELLOW, FlexDirection::ColumnReverse, true);

            parent.spawn(NodeBundle {
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

            spawn_player(parent, PINK, FlexDirection::ColumnReverse, true);
        });
}

fn place_south_line(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let style = HanTextStyle::default()
                .with_color(bevy::prelude::Color::Srgba(BLACK))
                .with_font_size(30.0)
                .get_style();
            spawn_game_button(
                parent,
                FlexDirection::RowReverse,
                style.clone(),
                RENEW_GAME_BUTTON_TEXT,
                ButtonOnGamePage::RenewGameButton,
            );

            spawn_player(parent, LIGHT_CORAL, FlexDirection::ColumnReverse, false);

            spawn_game_button(
                parent,
                FlexDirection::Row,
                style.clone(),
                DEAL_POKER_BUTTON_TEXT,
                ButtonOnGamePage::DealPokerButton,
            );
        });
}

fn spawn_player(
    parent: &mut ChildBuilder,
    bg_color: Srgba,
    flex_direction: FlexDirection,
    side_position: bool,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(if side_position { 30.0 } else { 40.0 }),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction,
                ..default()
            },
            background_color: bg_color.into(),
            ..default()
        })
        .with_children(|parent| {});
}

fn spawn_game_button(
    parent: &mut ChildBuilder,
    flex_direction: FlexDirection,
    style: TextStyle,
    text: &str,
    button_on_game_page: ButtonOnGamePage,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((ButtonBundle::default(), button_on_game_page))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(text, style.clone()));
                        });
                });
        });
}
