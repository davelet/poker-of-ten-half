#![allow(dead_code)]

use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use plugins::prelude::*;
use resources::prelude::*;
use systems::prelude::*;

mod components;
mod systems;
mod resources;
mod plugins;
mod constants;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins((splash_plugin, menu_plugin, game_plugin))
        .insert_resource(MatchPlayerCount::One)
        .insert_resource(MatchPokerSuitCount::One)
        .insert_resource(DeckTable::default())
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .run();
}
