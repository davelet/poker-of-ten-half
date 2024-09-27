#![allow(dead_code)]

use bevy::prelude::*;
use resources::prelude::*;
use systems::prelude::*;
use plugins::prelude::*;

mod components;
mod systems;
mod resources;
mod plugins;
mod constants;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MatchPlayerCount::One)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash_plugin, menu_plugin, game_plugin))
        .run();
}

