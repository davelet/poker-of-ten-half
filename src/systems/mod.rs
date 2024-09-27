use bevy::prelude::*;

mod setup;
mod splash_screen;
mod menu;
mod game;

pub mod prelude {
    pub use super::setup::*;
    pub use super::despawn_screen;
    pub use super::splash_screen::*;
    pub use super::menu::*;
    pub use super::game::*;
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}