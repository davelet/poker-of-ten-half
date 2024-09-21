use bevy::prelude::*;

use crate::components::*;

pub fn setup_game(mut commands: Commands) {
    create_deck(commands);
}
pub fn create_deck(mut commands: Commands) {
    let suits_arr = [
        PokerSuiteEnum::Club,
        PokerSuiteEnum::Diamond,
        PokerSuiteEnum::Heart,
        PokerSuiteEnum::Spade,
    ];
    for suite in suits_arr {
        for rank in 1..=13 {
            commands.spawn((
                // PokerCard,
                CardRank { rank },
                CardType { suite: suite.clone() },
                CardPoint {
                    point_type: match rank {
                        1..=10 => PokerReducedPoint::NaturalPoint(rank as u8),
                        _ => PokerReducedPoint::HalfPoint,
                    },
                },
            ));
        }
    }
    commands.spawn((
        // PokerCard,
        CardRank { rank: -1 },
        CardType {
            suite: PokerSuiteEnum::Joker,
        },
        CardPoint {
            point_type: PokerReducedPoint::HalfPoint,
        },
    ));
    commands.spawn((
        // PokerCard,
        CardRank { rank: -2 },
        CardType {
            suite: PokerSuiteEnum::Joker,
        },
        CardPoint {
            point_type: PokerReducedPoint::HalfPoint,
        },
    ));
}
