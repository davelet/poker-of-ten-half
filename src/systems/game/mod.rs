use bevy::{
    color::palettes::css::*,
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_rand::prelude::*;
use rand_core::RngCore;
use setup::prelude::*;

use crate::{components::prelude::*, resources::prelude::*};

mod setup;

pub fn shuffle_cards(mut commands: Commands, poker_query: Query<Entity, With<PokerCard>>, mut rng: ResMut<GlobalEntropy<WyRand>>) {
    for card in poker_query.iter() {
        let index = rng.next_u32();
        commands.entity(card).insert(PokerCardOrder(index));
    }

    // todo 把乱的牌放上去
}

pub fn game_setup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
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

pub fn game_button_action(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &ButtonOnGamePage), (Changed<Interaction>, With<Button>)>,
    mut keyboard_input_writer: EventWriter<KeyboardInput>,
    game_screen: Query<Entity, With<OnGameScreen>>,
) {
    let mut game_screen_entity = None;
    game_screen.iter().any(|p| {
        game_screen_entity = Some(p);
        true
    });
    for (interaction, mut color, btn_tag) in &mut interaction_query {
        match (btn_tag, *interaction) {
            (ButtonOnGamePage::BackMenuButton, Interaction::Pressed) => {
                let simulated_key_event = KeyboardInput {
                    key_code: KeyCode::KeyB,
                    logical_key: Key::Character("B".into()),
                    state: ButtonState::Pressed,
                    window: game_screen_entity.unwrap(),
                };
                keyboard_input_writer.send(simulated_key_event);
            },
            (ButtonOnGamePage::BackMenuButton, Interaction::Hovered) => {
                *color = DARK_BLUE.into();
            },
            (ButtonOnGamePage::BackMenuButton, Interaction::None) => {
                *color = LIGHT_BLUE.into();
            },
            (ButtonOnGamePage::ExitGameButton, Interaction::Hovered) => {
                *color = DARK_GREY.into();
            },
            (ButtonOnGamePage::ExitGameButton, Interaction::Pressed) => {
                let simulated_key_event = KeyboardInput {
                    key_code: KeyCode::KeyQ,
                    logical_key: Key::Character("Q".into()),
                    state: ButtonState::Pressed,
                    window: game_screen_entity.unwrap(),
                };
                keyboard_input_writer.send(simulated_key_event);
            },
            (ButtonOnGamePage::ExitGameButton, Interaction::None) => {
                *color = LIGHT_GRAY.into();
            },
            (ButtonOnGamePage::DealPokerButton, Interaction::None) => {
                *color = Color::NONE.into();
            },
            (ButtonOnGamePage::DealPokerButton, Interaction::Hovered) => {
                *color = LIGHT_SEA_GREEN.into();
            },
            (ButtonOnGamePage::DealPokerButton, Interaction::Pressed) => {
                let simulated_key_event = KeyboardInput {
                    key_code: KeyCode::KeyN,
                    logical_key: Key::Character("N".into()),
                    state: ButtonState::Pressed,
                    window: game_screen_entity.unwrap(),
                };
                keyboard_input_writer.send(simulated_key_event);
            },
            _ => {},
        }
    }
}

pub fn game_key_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    button_query: Query<(&mut BackgroundColor, &ButtonOnGamePage), With<Button>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        set_button_color(ButtonOnGamePage::BackMenuButton, DARK_BLUE.into(), button_query);
    } else if keyboard_input.just_released(KeyCode::KeyB) {
        app_state.set(AppState::Menu);
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    } else if keyboard_input.just_pressed(KeyCode::KeyN) {
        set_button_color(ButtonOnGamePage::DealPokerButton, LIGHT_SEA_GREEN.into(), button_query);
    } else if keyboard_input.just_released(KeyCode::KeyN) {
        game_state.set(GameState::Deal);
    }
}

fn set_button_color(
    target_btn: ButtonOnGamePage,
    bg_color: Color,
    mut button_query: Query<(&mut BackgroundColor, &ButtonOnGamePage), With<Button>>,
) {
    button_query.iter_mut().filter(|b| *b.1 == target_btn).any(|mut c| {
        *c.0 = bg_color.into();
        true
    });
}

pub fn deal_poker(
    mut poker_query: Query<(&PokerCard, &PokerCardStatus, &PokerCardOrder)>,
    mut deck_query: Query<&mut Text, With<DeckArea>>,
) {
    for (card, status, order) in poker_query.iter_mut() {
        println!("{:?} {:?} {}", card, status, order.0);
        for deck_text in deck_query.iter_mut() {
            // println!("{:?}", deck_text.sections[0].value);
        }
    }
}
