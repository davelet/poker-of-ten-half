use bevy::{app::App, prelude::*};

use crate::components::prelude::*;
use crate::resources::prelude::*;
use crate::systems::prelude::*;

pub mod prelude {
   pub use super::*;
}

pub fn splash_plugin(app: &mut App) {
    info!("Loading splash plugin");
    app.add_systems(OnEnter(GameState::Splash), (splash_setup).chain())
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

pub fn menu_plugin(app: &mut App) {
    info!("Loading menu plugin");
    app.add_systems(OnEnter(GameState::Menu), show_menu);
}

pub fn game_plugin(app: &mut App) {}
