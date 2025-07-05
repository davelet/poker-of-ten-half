#![allow(dead_code)]

use std::panic;

use bevy::{asset::AssetMetaCheck, prelude::*};
use constants::PANIC_FLAG;
use plugins::prelude::*;
use resources::prelude::*;
use systems::prelude::*;

mod components;
mod systems;
mod resources;
mod plugins;
mod constants;

// Create a prelude module for commonly used types
pub mod prelude {
    pub use crate::components::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::systems::prelude::*;
    pub use bevy::prelude::*;
}

fn main() {
    {
        panic::set_hook(Box::new(move |panic_info| {
            eprintln!("应用发生了错误: {}", panic_info);
            unsafe { PANIC_FLAG = true };
        }));
    }
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins((splash_plugin, menu_plugin, game_plugin, exit_plugin))
        .insert_resource(MatchPlayerCount::One)
        .insert_resource(MatchPokerSuitCount::One)
        .insert_resource(DeckTable::default())
        // Initialize new resource-based approach for better safety
        .init_resource::<constants::FontAssets>()
        .init_resource::<constants::ImageAssets>()
        .init_resource::<constants::PanicState>()
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>() // Enable StateScoped components for better cleanup
        .add_systems(Startup, setup)
        .add_systems(Update, check_panic_and_switch_state)
        .run();
}

fn check_panic_and_switch_state(
    state: ResMut<State<AppState>>, 
    mut next_state: ResMut<NextState<AppState>>,
    panic_state: Res<constants::PanicState>,
) {
    // Check both new resource-based approach and legacy unsafe global for compatibility
    let panic_flag = panic_state.flag || unsafe { PANIC_FLAG };
    if panic_flag && *state.get() != AppState::Panic {
        println!("panic set");
        next_state.set(AppState::Panic);
    }
}
