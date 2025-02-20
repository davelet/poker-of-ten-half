use bevy::{color::palettes::css::*, prelude::*};

use crate::{components::prelude::*, constants::*, AppState, HanTextStyle, IconLoader};

pub fn show_menu(mut commands: Commands) {
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

            place_image(parent);

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
            // TODO 第一行放两个按钮，控制人数和牌副，最多4人4副牌 TODO
            p.spawn(NodeBundle {
                style: Style {
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
            p.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: START_BUTTON_NORMAL_COLOR,
                    ..Default::default()
                },
                ButtonOnMenuPage::StartGameButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    START_BUTTON_TEXT,
                    HanTextStyle::default().with_color(TEXT_COLOR).with_font_size(40.0).get_style(),
                ));
            });

            p.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ButtonOnMenuPage::ExitGameButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    EXIT_BUTTON_TEXT,
                    HanTextStyle::default().with_color(TEXT_COLOR).with_font_size(40.0).get_style(),
                ));
            });
        });
}
fn place_image(parent: &mut ChildBuilder) {
    parent.spawn(ImageBundle {
        style: Style { width: Val::Px(350.0), ..default() },
        image: IconLoader::default().get_image(),
        ..default()
    });
}

fn place_title(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, ..default() },
            background_color: bevy::color::palettes::css::DARK_GREEN.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(MENU_TITLE, HanTextStyle::default().with_color(TEXT_COLOR).with_font_size(80.0).get_style())
                    .with_style(Style { margin: UiRect::all(Val::Px(20.0)), ..default() }),
            );
        });
}

pub fn menu_action(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &ButtonOnMenuPage), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, btn_tag) in &mut interaction_query {
        match (btn_tag, *interaction) {
            (ButtonOnMenuPage::StartGameButton, Interaction::Pressed) => {
                game_state.set(AppState::Game);
            },
            (ButtonOnMenuPage::StartGameButton, Interaction::Hovered) => {
                *color = START_BUTTON_HOVER_COLOR.into();
            },
            (ButtonOnMenuPage::ExitGameButton, Interaction::Hovered) => {
                *color = DARK_GREY.into();
            },
            (ButtonOnMenuPage::ExitGameButton, Interaction::Pressed) => {
                app_exit_events.send(AppExit::Success);
            },
            _ => {
                *color = START_BUTTON_NORMAL_COLOR;
            },
        }
    }
}

pub fn menu_key_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut button_query: Query<(&mut BackgroundColor, &ButtonOnMenuPage), With<Button>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        for (mut color, btn) in button_query.iter_mut() {
            if let ButtonOnMenuPage::StartGameButton = btn {
                *color = START_BUTTON_HOVER_COLOR.into();
                break;
            }
        }
    }
    if keyboard_input.just_released(KeyCode::KeyC) {
        game_state.set(AppState::Game);
    }

    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    }
}
