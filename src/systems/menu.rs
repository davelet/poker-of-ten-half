use bevy::{color::palettes::css::*, prelude::*};

use crate::{components::prelude::*, constants::*, GameState, HanTextStyle};

pub fn show_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            place_title(parent);

            place_image(asset_server, parent);

            // let button_style = Style {
            //     width: Val::Px(250.0),
            //     height: Val::Px(65.0),
            //     margin: UiRect::all(Val::Px(20.0)),
            //     justify_content: JustifyContent::Center,
            //     align_items: AlignItems::Center,
            //     ..default()
            // };
            // let button_icon_style = Style {
            //     width: Val::Px(30.0),
            //     // This takes the icons out of the flexbox flow, to be positioned exactly
            //     position_type: PositionType::Absolute,
            //     // The icon will be close to the left border of the button
            //     left: Val::Px(10.0),
            //     ..default()
            // };
            // let button_text_style = TextStyle {
            //     font_size: 40.0,
            //     color: TEXT_COLOR,
            //     ..default()
            // };
            place_buttons(parent);
        });
}

fn place_buttons(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            // 第一行放两个按钮
            p.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            // .with_children(|parent: &mut ChildBuilder<'_>| {
                // parent
                //     .spawn(TextBundle {
                //         text: Text::from_section(
                //             "1", // initial text
                //             HanTextStyle::default()
                //                 .with_color(TEXT_COLOR)
                //                 .with_font_size(40.0)
                //                 .get_style(),
                //         ),
                //         ..Default::default()
                //     });
            // })
            ;

            // 第二行放开始
            p.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: START_BUTTON_NORMAL_COLOR,
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "SG",
                    HanTextStyle::default()
                        .with_color(TEXT_COLOR)
                        .with_font_size(40.0)
                        .get_style(),
                ));
            });
        });
}
fn place_image(asset_server: Res<AssetServer>, parent: &mut ChildBuilder) {
    let icon = asset_server.load("poker-title.png");
    parent.spawn(ImageBundle {
        style: Style {
            width: Val::Px(350.0),
            ..default()
        },
        image: UiImage::new(icon),
        ..default()
    });
}

fn place_title(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: bevy::color::palettes::css::DARK_GREEN.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    MENU_TITLE,
                    HanTextStyle::default()
                        .with_color(TEXT_COLOR)
                        .with_font_size(80.0)
                        .get_style(),
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
        });
}

pub fn menu_action(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            *color = DARK_GREY.into();
            app_exit_events.send(AppExit::Success);
        } else if *interaction == Interaction::Hovered {
            *color = START_BUTTON_HOVER_COLOR.into(); 
        } else {
            *color = START_BUTTON_NORMAL_COLOR;
        }
    }
}
