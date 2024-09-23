use bevy::prelude::*;

mod splash_screen;
pub mod game;

pub mod prelude {

    pub use super::despawn_screen;
    pub use super::splash_screen::*;
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}