use bevy::{app::App, prelude::*};

use crate::systems::prelude::*;
use crate::resources::prelude::*;
use crate::components::prelude::*;

pub fn splash_plugin(app: &mut App) {
   // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
   app
   // When entering the state, spawn everything needed for this screen
   .add_systems(OnEnter(GameState::Splash), splash_setup)
   // While in this state, run the `countdown` system
   .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
   // When exiting the state, despawn everything that was spawned for this screen
   .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>); 
}

pub fn menu_plugin(app: &mut App) {
    
}

pub fn game_plugin(app: &mut App) {
    
}