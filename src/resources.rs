use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource)]
pub enum MatchPlayerCount {
    One = 1,
    Two = 2,
    Three = 3,
}