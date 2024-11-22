use core::panic;

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

use crate::{components::prelude::*, constants::*, resources::prelude::*};

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
    mut game_state: ResMut<NextState<MatchState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        set_button_color(ButtonOnGamePage::BackMenuButton, DARK_BLUE.into(), button_query);
    } else if keyboard_input.just_released(KeyCode::KeyB) {
        app_state.set(AppState::Menu);
    } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    } else if keyboard_input.just_pressed(KeyCode::KeyN) {
        set_button_color(ButtonOnGamePage::DealPokerButton, LIGHT_SEA_GREEN.into(), button_query);
        game_state.set(MatchState::DealingSouth);
    } else if keyboard_input.just_released(KeyCode::KeyN) {
        set_button_color(ButtonOnGamePage::DealPokerButton, Color::NONE.into(), button_query);
    } else if keyboard_input.just_pressed(KeyCode::KeyJ) {
        commands.spawn(SkipTurn(MatchState::SouthTurn));
        game_state.set(MatchState::EastTurn);
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

fn show_poker_for_player(
    mut commands: Commands,
    player: MatchState,
    card: &PokerCard,
    mut deal_state: ResMut<NextState<DealPokerInMatch>>,
) {
    match player {
        MatchState::DealingSouth | MatchState::EastTurn | MatchState::NorthTurn | MatchState::WestTurn => {
            deal_state.set(DealPokerInMatch::Deal);
            commands.spawn((card.clone(), DealingPokerRecord));
        },
        _ => panic!("error input in `show_poker_for_player`"),
    }
}

pub fn display_pokers(
    mut commands: Commands,
    mut deal_state: ResMut<NextState<DealPokerInMatch>>,
    game_state: ResMut<State<MatchState>>,
    mut dealing_query: Query<(Entity, &PokerCard, &DealingPokerRecord)>,
    mut type_text_query: Query<(&mut Text, &SinglePokerAreaSlot, &PokerCardTypeSlotWithIndex), Without<PokerCardRankSlotWithIndex>>,
    mut rank_text_query: Query<(&mut Text, &SinglePokerAreaSlot, &PokerCardRankSlotWithIndex), Without<PokerCardTypeSlotWithIndex>>,
) {
    let state = game_state.get();
    let mut card = None;
    for (e, p, _) in dealing_query.iter_mut() {
        card = Some(p.clone());
        commands.entity(e).despawn();
        break;
    }
    let card = card.unwrap();
    println!("card: {:?}", card);
    for (mut node, slot, type_flag) in type_text_query.iter_mut() {
        if slot.0 == *state && node.sections[0].value == POKER_EMPTY_SLOT_TEXT {
            println!("==={:?}card: {:?} > {:?}", node, slot, type_flag);
            node.sections[0].value = generate_type_text(&card.suite);
            break;
        }
    }
    for (mut node, slot, rank) in rank_text_query.iter_mut() {
        if slot.0 == *state && node.sections[0].value == BLANK_STRING {
            println!("+++{:?}card: {:?} > {:?}", node, slot, rank);
            node.sections[0].value = card.rank.rank.to_string();
            break;
        }
    }

    // match *state {
    //     MatchState::SouthTurn => {},
    //     MatchState::EastTurn => {},
    //     MatchState::NorthTurn => {},
    //     MatchState::WestTurn => {},
    //     _ => {},
    // }
    deal_state.set(DealPokerInMatch::End); // 结束发牌
}

fn generate_type_text(suite: &CardType) -> String {
    match suite.suite {
        PokerSuiteEnum::Spade => "黑桃".to_string(),
        PokerSuiteEnum::Heart => "红桃".to_string(),
        PokerSuiteEnum::Diamond => "方片".to_string(),
        PokerSuiteEnum::Club => "梅花".to_string(),
        PokerSuiteEnum::Joker => "王牌".to_string(),
    }
}
pub fn next_player(current_state: Res<State<MatchState>>, mut game_state: ResMut<NextState<MatchState>>) {
    let state = current_state.get();
    match *state {
        MatchState::DealingSouth => {
            game_state.set(MatchState::EastTurn);
        },
        MatchState::EastTurn => game_state.set(MatchState::NorthTurn),
        MatchState::NorthTurn => game_state.set(MatchState::WestTurn),
        MatchState::WestTurn => game_state.set(MatchState::SouthTurn),
        _ => {},
    }
}

pub fn deal_south(
    commands: Commands,
    poker_query: Query<(&PokerCard, &mut PokerCardStatus)>,
    deck_query: Query<(&mut Text, &DeckArea)>,
    game_state: ResMut<NextState<MatchState>>,
    deal_state: ResMut<NextState<DealPokerInMatch>>,
) {
    deal_single_poker(commands, poker_query, deck_query, deal_state, game_state, MatchState::DealingSouth);
}

fn deal_single_poker(
    commands: Commands,
    mut poker_query: Query<(&PokerCard, &mut PokerCardStatus)>,
    deck_query: Query<(&mut Text, &DeckArea)>,
    deal_state: ResMut<NextState<DealPokerInMatch>>,
    mut game_state: ResMut<NextState<MatchState>>,
    player: MatchState,
) {
    for (card, mut status) in poker_query.iter_mut() {
        if *status != PokerCardStatus::OnTable {
            continue;
        }

        *status = PokerCardStatus::OnHand;
        update_deck_area(deck_query, true, -1);
        show_poker_for_player(commands, player, card, deal_state);

        return;
    }
    game_state.set(MatchState::Ended);
}

pub fn waiting_deal_south(mut game_state: ResMut<NextState<MatchState>>, skip_turn_query: Query<&SkipTurn>) {
    if skip_turn_query.iter().any(|t| t.0 == MatchState::SouthTurn) {
        game_state.set(MatchState::EastTurn);
    }
}

pub fn deal_east(
    commands: Commands,
    skip_turn_query: Query<&SkipTurn>,
    mut game_state: ResMut<NextState<MatchState>>,
    deal_state: ResMut<NextState<DealPokerInMatch>>,
    poker_query: Query<(&PokerCard, &mut PokerCardStatus)>,
    deck_query: Query<(&mut Text, &DeckArea)>,
    player_count: Res<MatchPlayerCount>,
) {
    let mut skip = false;
    for turn in skip_turn_query.iter() {
        println!("{:?}", turn.0);
        if turn.0 == MatchState::EastTurn {
            skip = true;
            break;
        }
    }
    if !skip && *player_count.into_inner() != MatchPlayerCount::One {
        deal_single_poker(commands, poker_query, deck_query, deal_state, game_state, MatchState::EastTurn);
        return;
    }

    println!("dealed east done poker");
    game_state.set(MatchState::NorthTurn);
}

pub fn deal_north(
    commands: Commands,
    poker_query: Query<(&PokerCard, &mut PokerCardStatus)>,
    deck_query: Query<(&mut Text, &DeckArea)>,
    mut game_state: ResMut<NextState<MatchState>>,
    deal_state: ResMut<NextState<DealPokerInMatch>>,
    skip_turn_query: Query<&SkipTurn>,
) {
    let mut skip = false;
    for turn in skip_turn_query.iter() {
        println!("{:?}", turn.0);
        if turn.0 == MatchState::NorthTurn {
            skip = true;
            break;
        }
    }
    if !skip {
        deal_single_poker(commands, poker_query, deck_query, deal_state, game_state, MatchState::NorthTurn);
        return;
    }

    println!("dealed north done poker");
    game_state.set(MatchState::WestTurn);
}

pub fn deal_west(
    commands: Commands,
    poker_query: Query<(&PokerCard, &mut PokerCardStatus)>,
    deck_query: Query<(&mut Text, &DeckArea)>,
    mut game_state: ResMut<NextState<MatchState>>,
    deal_state: ResMut<NextState<DealPokerInMatch>>,
    skip_turn_query: Query<&SkipTurn>,
    player_count: Res<MatchPlayerCount>,
) {
    let mut skip = false;
    for turn in skip_turn_query.iter() {
        println!("{:?}", turn.0);
        if turn.0 == MatchState::WestTurn {
            skip = true;
            break;
        }
    }
    if !skip && *player_count.into_inner() == MatchPlayerCount::Three {
        deal_single_poker(commands, poker_query, deck_query, deal_state, game_state, MatchState::WestTurn);
        return;
    }

    println!("dealed west done poker");
    game_state.set(MatchState::SouthTurn);
}

pub fn match_eneded(mut poker_query: Query<(&PokerCard, &mut PokerCardStatus)>, mut deck_query: Query<(&mut Text, &DeckArea)>) {
    let mut to_unsed = 0;
    for (card, mut status) in poker_query.iter_mut() {
        if *status == PokerCardStatus::OnTable {
            break;
        }
        if *status == PokerCardStatus::OnHand {
            *status = PokerCardStatus::Used;
            to_unsed += 1;
        }
    }
    if to_unsed > 0 {
        update_deck_area(deck_query, false, to_unsed);
    }
}

fn update_deck_area(mut deck_query: Query<(&mut Text, &DeckArea)>, deck_flag: bool, adder: i32) {
    for (mut deck_text, area_flag) in deck_query.iter_mut() {
        if (*area_flag == DeckArea::USED) == deck_flag {
            continue;
        }
        let text = &deck_text.sections[0].value;
        let current_count: i32 = text.split_whitespace().last().unwrap_or("0").parse().unwrap_or(0);
        let new_count = current_count + adder;
        deck_text.sections[0].value = format!("{} {}", text.trim_end_matches(char::is_numeric).trim(), new_count);

        break;
    }
}
