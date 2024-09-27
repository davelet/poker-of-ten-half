use bevy::prelude::Component;

#[derive(Component)]
pub struct OnMenuScreen;

#[derive(Component)]
pub enum ButtonOnMenuPage {
    StartGameButton,
    ExitGameButton,
}

