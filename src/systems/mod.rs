use bevy::prelude::*;

mod setup;
mod splash;
mod menu;
mod game;
mod exit;

pub mod prelude {
    pub use super::{despawn_screen, game::*, menu::*, setup::*, splash::*};
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
