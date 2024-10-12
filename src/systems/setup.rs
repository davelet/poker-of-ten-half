use bevy::prelude::*;

use crate::components::prelude::*;

pub fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    create_deck(commands);
}
pub fn create_deck(mut commands: Commands) {
    let suits_arr = [PokerSuiteEnum::Club, PokerSuiteEnum::Diamond, PokerSuiteEnum::Heart, PokerSuiteEnum::Spade];
    for suite in suits_arr {
        for rank in 1..=13 {
            commands.spawn((PokerCard::new(rank, suite.clone()), PokerCardStatus::OnDeck));
        }
    }
    commands.spawn((PokerCard::joker(true), PokerCardStatus::OnDeck));
    commands.spawn((PokerCard::joker(false), PokerCardStatus::OnDeck));
}
