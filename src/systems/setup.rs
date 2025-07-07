use bevy::prelude::*;

use crate::components::prelude::*;
use crate::resources::{MatchState, MatchPlayerCount};

pub fn setup(mut commands: Commands, _asset_server: Res<AssetServer>, player_count: Res<MatchPlayerCount>) {
    commands.spawn(Camera2dBundle::default());

    create_deck(commands, player_count);
}
pub fn create_deck(mut commands: Commands, player_count: Res<MatchPlayerCount>) {
    let suits_arr = [PokerSuiteEnum::Club, PokerSuiteEnum::Diamond, PokerSuiteEnum::Heart, PokerSuiteEnum::Spade];
    for suite in suits_arr {
        for rank in 1..=13 {
            commands.spawn((PokerCard::new(rank, suite.clone()), PokerCardStatus::OnDeck));
        }
    }
    commands.spawn((PokerCard::joker(true), PokerCardStatus::OnDeck));
    commands.spawn((PokerCard::joker(false), PokerCardStatus::OnDeck));

    // 根据玩家数量决定跳过哪些玩家
    match *player_count {
        MatchPlayerCount::One => {
            // 1个对手：跳过东、西（南北对战）
            commands.spawn(SkipTurn(MatchState::EastTurn));
            commands.spawn(SkipTurn(MatchState::WestTurn));
        },
        MatchPlayerCount::Two => {
            // 2个对手：跳过西（南东北对战）
            commands.spawn(SkipTurn(MatchState::WestTurn));
        },
        MatchPlayerCount::Three => {
            // 3个对手：4人对战，不跳过任何人
        },
    }
}
