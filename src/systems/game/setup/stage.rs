use bevy::{color::palettes::css::*, math::bool, prelude::*, scene::ron::de};

use crate::{components::prelude::*, constants::*, HanTextStyle, MatchState};

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
            spawn_player(parent, MEDIUM_PURPLE, FlexDirection::Column, MatchState::NorthTurn, true);
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
            spawn_player(parent, GREEN_YELLOW, FlexDirection::ColumnReverse, MatchState::WestTurn, false);

            setup_deck_stage(parent);

            spawn_player(parent, PINK, FlexDirection::ColumnReverse, MatchState::EastTurn, false);
        });
}

// 中间拍桌，未发的和已弃的
fn setup_deck_stage(parent: &mut ChildBuilder) {
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
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(90.0),
                        height: Val::Percent(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: LIGHT_SEA_GREEN.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            POKER_DECK_AVAILABLE_TEXT,
                            HanTextStyle::default().with_color(bevy::prelude::Color::Srgba(BLACK)).with_font_size(60.0).get_style(),
                        ))
                        .insert(DeckArea::AVAIL);
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(60.0),
                        height: Val::Percent(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: LIGHT_SKY_BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            POKER_USED_TEXT,
                            HanTextStyle::default().with_color(bevy::prelude::Color::Srgba(BLACK)).with_font_size(30.0).get_style(),
                        ))
                        .insert(DeckArea::USED);
                });
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
            let style = HanTextStyle::default().with_color(bevy::prelude::Color::Srgba(BLACK)).with_font_size(30.0).get_style();
            spawn_game_button(parent, FlexDirection::RowReverse, style.clone(), RENEW_GAME_BUTTON_TEXT, ButtonOnGamePage::RenewGameButton);

            spawn_player(parent, LIGHT_CORAL, FlexDirection::ColumnReverse, MatchState::DealingSouth, true);

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::ColumnReverse,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_game_button(parent, FlexDirection::Row, style.clone(), DEAL_POKER_BUTTON_TEXT, ButtonOnGamePage::DealPokerButton);
                    spawn_game_button(
                        parent,
                        FlexDirection::Row,
                        style.clone(),
                        STOP_DEALING_BUTTON_TEXT,
                        ButtonOnGamePage::StopDealingButton,
                    );
                });
        });
}

fn spawn_player(parent: &mut ChildBuilder, bg_color: Srgba, flex_direction: FlexDirection, turn: MatchState, has_content: bool) {
    let side_position = turn == MatchState::EastTurn || turn == MatchState::WestTurn;
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
        .with_children(|parent| {
            let style = HanTextStyle::default().with_color(bevy::prelude::Color::Srgba(BLACK)).with_font_size(20.0).get_style();
            if has_content {
                parent.spawn(TextBundle::from_section(format!("{}{}", PLAYER_PLOT_TEXT, generate_player_name(&turn)), style.clone()));
                parent.spawn(TextBundle::from_section("豆子100", style.clone()));
                spawn_cards(parent, &turn);
            } else {
                parent.spawn(TextBundle::from_section("Nobody", style));
            }
        });
}

fn generate_player_name(turn: &MatchState) -> String {
    match *turn {
        MatchState::DealingSouth => "南".to_string(),
        MatchState::WestTurn => "西".to_string(),
        MatchState::NorthTurn => "北".to_string(),
        MatchState::EastTurn => "东".to_string(),
        _ => panic!("Invalid turn"),
    }
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
            style: Style { width: Val::Percent(30.0), flex_direction, margin: UiRect::all(Val::Px(10.0)), ..default() },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style { height: Val::Percent(100.0), flex_direction: FlexDirection::ColumnReverse, ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((ButtonBundle::default(), button_on_game_page)).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(text, style.clone()));
                    });
                });
        });
}

fn spawn_cards(parent: &mut ChildBuilder, turn: &MatchState) {
    let style = HanTextStyle::default().with_color(bevy::prelude::Color::Srgba(BLACK)).with_font_size(26.0).get_style();
    parent
        .spawn(NodeBundle {
            style: Style { width: Val::Percent(80.0), height: Val::Percent(50.0), ..default() },
            background_color: WHITE.into(),
            ..default()
        })
        .with_children(|parent| {
            for idx in 1..=5 {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(20.0),
                            margin: UiRect::all(Val::Percent(4.0)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        border_color: BLACK.into(),
                        background_color: DARK_OLIVEGREEN.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(NodeBundle { ..default() }).with_children(|parent| {
                            parent.spawn((
                                TextBundle { text: Text::from_section(POKER_EMPTY_SLOT_TEXT, style.clone()), ..default() },
                                PokerCardTypeSlotWithIndex(idx),
                                SinglePokerAreaSlot(turn.clone()),
                            ));
                            parent.spawn((
                                TextBundle { text: Text::from_section(BLANK_STRING, style.clone()), ..default() },
                                PokerCardRankSlotWithIndex(idx),
                                SinglePokerAreaSlot(turn.clone()),
                            ));
                        });
                    });
            }
        });
    parent
        .spawn(NodeBundle {
            style: Style { width: Val::Percent(80.0), height: Val::Percent(30.0), align_items: AlignItems::Center, ..default() },
            background_color: LIGHT_CYAN.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style { width: Val::Percent(60.0), margin: UiRect::horizontal(Val::Percent(25.0)), ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    let style = TextStyle { font_size: 30.0, ..style };
                    parent.spawn(TextBundle::from_section(TOTAL_POINT_TEXT, style.clone()));
                    parent.spawn((TextBundle::from_section(BLANK_STRING, style.clone()), PlayerPointShown));
                });
        });
}
