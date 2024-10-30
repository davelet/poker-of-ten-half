use bevy::{
    color::palettes::css::*,
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use rand::seq::SliceRandom;
use setup::prelude::*;

use crate::{components::prelude::*, resources::prelude::*};

mod setup;

pub fn shuffle_cards(mut commands: Commands, poker_query: Query<(Entity, &PokerCard)>) {
    let mut shuffled_cards: Vec<PokerCard> = poker_query.iter().map(|p| (*p.1).clone()).collect();
    let mut rng = rand::thread_rng();
    shuffled_cards.shuffle(&mut rng);
    for (entity, _) in poker_query.iter() {
        commands.entity(entity).despawn();
    }
    for c in shuffled_cards {
        commands.spawn((c, PokerCardStatus::OnTable));
    }
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

pub fn update_stage(poker_query: Query<(&PokerCard, &PokerCardStatus)>, mut deck_query: Query<&mut Text, With<DeckArea>>) {
    let mut avail_cards = vec![];
    let mut used_cards = vec![];

    for (card, status) in poker_query.iter() {
        match *status {
            PokerCardStatus::OnTable => avail_cards.push(card),
            PokerCardStatus::OnHand => used_cards.push(card),
            _ => {},
        }
    }
    for (idx, mut deck_text) in deck_query.iter_mut().enumerate() {
        let text = &deck_text.sections[0].value;
        let new_text: String;
        if idx == 0 {
            new_text = format!("{} {}", text, avail_cards.len());
        } else {
            new_text = format!("{} {}", text, used_cards.len());
        }
        deck_text.sections[0].value = new_text;
    }
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
            (ButtonOnGamePage::StopDealingButton, Interaction::Pressed) => {
                let simulated_key_event = KeyboardInput {
                    key_code: KeyCode::KeyJ,
                    logical_key: Key::Character("J".into()),
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
    mut commands: Commands,
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
        game_state.set(GameState::DealingSouth);
    } else if keyboard_input.just_released(KeyCode::KeyN) {
        set_button_color(ButtonOnGamePage::DealPokerButton, Color::NONE.into(), button_query);
    } else if keyboard_input.just_pressed(KeyCode::KeyJ) {
        commands.spawn(SkipTurn(GameState::SouthTurn));
        game_state.set(GameState::EastTurn);
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

pub fn deal_south(
    mut poker_query: Query<(&PokerCard, &mut PokerCardStatus)>,
    mut deck_query: Query<&mut Text, With<DeckArea>>,
    mut game_state: ResMut<NextState<GameState>>,
    skip_turn_query: Query<&SkipTurn>,
) {
    for (card, mut status) in poker_query.iter_mut() {
        if *status != PokerCardStatus::OnTable {
            continue;
        }

        *status = PokerCardStatus::OnHand;        
        for deck_text in deck_query.iter_mut() {
            // println!("{:?}", deck_text.sections[0].value);
        }
        game_state.set(GameState::EastTurn);
        return;
    }
    game_state.set(GameState::Ended);
}

pub fn waiting_deal_south(
    mut poker_query: Query<(&PokerCard, &PokerCardStatus)>,
    mut deck_query: Query<&mut Text, With<DeckArea>>,
    mut game_state: ResMut<NextState<GameState>>,
    skip_turn_query: Query<&SkipTurn>,
) {
    if skip_turn_query.iter().any(|t| t.0 == GameState::SouthTurn) {
        println!("skip south turn");
        game_state.set(GameState::EastTurn);
    }
    // else {
    //     println!("dealed south done poker");
    //
    // }
    // println!("dealed south done poker");
    // game_state.set(GameState::EastTurn);
}

pub fn deal_east(
    mut poker_query: Query<(&PokerCard, &PokerCardStatus)>,
    mut deck_query: Query<&mut Text, With<DeckArea>>,
    skip_turn_query: Query<&SkipTurn>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // for (card, status) in poker_query.iter() {
    //     println!("{:?} {:?} ", card, status);
    //     for deck_text in deck_query.iter_mut() {
    //         // println!("{:?}", deck_text.sections[0].value);
    //     }
    // }
    let mut skip = false;
    for turn in skip_turn_query.iter() {
        println!("{:?}", turn.0);
        if turn.0 == GameState::EastTurn {
            skip = true;
            break;
        }
    }
    if !skip {
        // println!("skip east turn");
    }

    println!("dealed east done poker");
    game_state.set(GameState::NorthTurn);
}

pub fn deal_north(
    mut poker_query: Query<(&PokerCard, &PokerCardStatus)>,
    mut deck_query: Query<&mut Text, With<DeckArea>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    println!("dealed north done poker");
    game_state.set(GameState::WestTurn);
}

pub fn deal_west(
    mut poker_query: Query<(&PokerCard, &PokerCardStatus)>,
    mut deck_query: Query<&mut Text, With<DeckArea>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    println!("dealed west done poker");
    game_state.set(GameState::SouthTurn);
}
