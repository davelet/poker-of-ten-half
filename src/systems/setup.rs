use bevy::prelude::*;

use crate::{components::prelude::*, constants::HAN_FONT_OPTION};


pub fn pre_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("pre setup");
    // let font_handle = asset_server.load::<Font>("/System/Library/Fonts/PingFang.ttc");
    let font_handle = asset_server.load::<Font>("fonts/NotoSanSC-ExtraBold1.ttf");
    // commands.insert_resource(font_handle);
    unsafe { HAN_FONT_OPTION = Some(font_handle) };
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("setup");
    commands.spawn(Camera2dBundle::default());
    // let font_handle = asset_server.load::<Font>("/System/Library/Fonts/PingFang.ttc");
    // unsafe { HAN_FONT_OPTION = Some(font_handle) };
    // println!("LS = {:?}", asset_server.load_state(font_handle.id()));
    
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
