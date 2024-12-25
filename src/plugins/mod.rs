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
    app.init_state::<MatchState>()
        .init_state::<DealPokerInMatch>()
        .add_systems(OnEnter(AppState::Game), (shuffle_cards, game_setup, update_stage).chain())
        .add_systems(Update, (game_button_action, game_key_input).run_if(in_state(AppState::Game)))
        .add_systems(OnEnter(MatchState::SouthTurn), waiting_deal_south)
        .add_systems(OnEnter(MatchState::DealingSouth), deal_south)
        .add_systems(OnEnter(MatchState::EastTurn), deal_east)
        .add_systems(OnEnter(MatchState::NorthTurn), deal_north)
        .add_systems(OnEnter(MatchState::WestTurn), deal_west)
        .add_systems(OnEnter(MatchState::Ended), match_ended)
        .add_systems(OnEnter(MatchState::Idle), match_cleanup)
        .add_systems(OnEnter(DealPokerInMatch::Deal), display_pokers)
        .add_systems(OnEnter(DealPokerInMatch::End), next_player)
        .add_systems(OnExit(AppState::Game), despawn_screen::<OnGameScreen>);
}

pub fn exit_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Panic), exit_screen);
}

fn exit_screen(mut commands: Commands) {
    // commands.spawn((
    //     NodeBundle
    // ))
    eprintln!("123-----------");
}
