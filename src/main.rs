use bevy::prelude::*;
use resources::*;
use systems::game::*;
use plugins::*;

mod components;
mod systems;
mod resources;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MatchPlayerCount::One)
        .init_state::<GameState>()
        .add_systems(Startup, setup_game)
        .add_plugins((splash_plugin, menu_plugin, game_plugin))
        .run();
}

