use bevy::{color::palettes::css::CRIMSON, prelude::*};

use crate::{components::prelude::*, constants::{self, *}, HanTextStyle};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub fn show_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("show_menu");

    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    if let Some(han) = unsafe { constants::HAN_FONT_OPTION.clone() } {
                        info!("Using Han font {:?}", asset_server.load_state(han.id()));
                    }
                    parent.spawn(
                        TextBundle::from_section(
                            MENU_TITLE,
                            HanTextStyle::default()
                                .with_color(TEXT_COLOR)
                                .with_font_size(80.0)
                                .get_style(),
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                });
        });
}
