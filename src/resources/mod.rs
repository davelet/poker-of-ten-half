use bevy::prelude::*;

pub mod prelude {
    pub use super::*;
}

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

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer {
    pub timer: Timer,
}