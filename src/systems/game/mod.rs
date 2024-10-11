use crate::{components::prelude::*, GameState};
use bevy::{color::palettes::css::*, prelude::*};
use setup::prelude::*;

mod setup;

pub fn game_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    // justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            place_header(parent);

            place_stage(parent);
        });
}

pub fn game_update(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &ButtonOnGamePage), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, btn_tag) in &mut interaction_query {
        match (btn_tag, *interaction) {
            (ButtonOnGamePage::BackMenuButton, Interaction::Pressed) => {
                game_state.set(GameState::Menu);
            },
            (ButtonOnGamePage::BackMenuButton, Interaction::Hovered) => {
                *color = DARK_BLUE.into();
            },
            (ButtonOnGamePage::ExitGameButton, Interaction::Hovered) => {
                *color = DARK_GREY.into();
            },
            (ButtonOnGamePage::ExitGameButton, Interaction::Pressed) => {
                app_exit_events.send(AppExit::Success);
            },
            (ButtonOnGamePage::ExitGameButton, Interaction::None) => {
                *color = LIGHT_GRAY.into();
            },
            (ButtonOnGamePage::BackMenuButton, Interaction::None) => {
                *color = LIGHT_BLUE.into();
            },
            _ => {},
        }
    }
}

pub fn game_key_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut button_query: Query<(&mut BackgroundColor, &ButtonOnGamePage), With<Button>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        for (mut color, btn) in button_query.iter_mut() {
            if let ButtonOnGamePage::BackMenuButton = btn {
                *color = DARK_BLUE.into();
                break;
            }
        }
    }

    if keyboard_input.just_released(KeyCode::KeyB) {
        game_state.set(GameState::Menu);
    }

    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    }
}
