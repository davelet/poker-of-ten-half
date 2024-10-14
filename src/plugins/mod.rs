use bevy::{app::App, prelude::*};

use crate::{components::prelude::*, resources::prelude::*, systems::prelude::*};

pub mod prelude {
    pub use super::*;
}

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(AppState::Splash)))
        .add_systems(OnExit(AppState::Splash), despawn_screen::<OnSplashScreen>);
}

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), show_menu)
        .add_systems(Update, (menu_action, menu_key_input_system).run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), despawn_screen::<OnMenuScreen>);
}

pub fn game_plugin(app: &mut App) {
    app.init_state::<GameState>()
        .add_systems(OnEnter(AppState::Game), (shuffle_cards, game_setup, update_stage).chain())
        .add_systems(Update, (game_button_action, game_key_input).run_if(in_state(AppState::Game)))
        .add_systems(OnEnter(GameState::Deal), deal_poker)
        .add_systems(OnExit(AppState::Game), despawn_screen::<OnGameScreen>);
}
