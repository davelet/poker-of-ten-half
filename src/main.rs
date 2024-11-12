#![allow(dead_code)]

use std::panic;

use bevy::prelude::*;
use constants::PANIC_FLAG;
use plugins::prelude::*;
use resources::prelude::*;
use systems::prelude::*;

mod components;
mod systems;
mod resources;
mod plugins;
mod constants;

fn main() {
    {
        panic::set_hook(Box::new(move |panic_info| {
            eprintln!("应用发生了错误: {}", panic_info);
            unsafe { PANIC_FLAG = true };
        }));
    }
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((splash_plugin, menu_plugin, game_plugin, exit_plugin))
        .insert_resource(MatchPlayerCount::One)
        .insert_resource(MatchPokerSuitCount::One)
        .insert_resource(DeckTable::default())
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, check_panic_and_switch_state)
        .run();
}

fn check_panic_and_switch_state(state: ResMut<State<AppState>>, mut next_state: ResMut<NextState<AppState>>) {
    println!("check_panic_and_switch_state");
    if unsafe { PANIC_FLAG } && *state.get() != AppState::Panic {
        println!("panic set");
        next_state.set(AppState::Panic);
    }
}
