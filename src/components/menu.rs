use bevy::prelude::Component;

#[derive(Component)]
pub struct OnMenuScreen;

#[derive(Component, Debug)]
pub enum ButtonOnMenuPage {
    StartGameButton,
    ExitGameButton,
}
