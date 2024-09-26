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

pub struct HanTextStyle(pub TextStyle);

impl Default for HanTextStyle {
    fn default() -> Self {
        if let Some(hf) = unsafe { crate::constants::HAN_FONT_OPTION.clone() } {
            return Self(TextStyle {
                font: hf,
                ..Default::default()
            });
        }
        Self(TextStyle {
            ..Default::default()
        })
    }
}

impl HanTextStyle {
    pub fn with_font_size(&self, font_size: f32) -> Self {
        let mut default = HanTextStyle::default();

        default.0.font_size = font_size;
        default
    }

    pub fn with_color(&self, color: Color) -> Self {
        let mut default = HanTextStyle::default();

        default.0.color = color;
        default
    }

    pub fn get_style(&self) -> TextStyle {
        self.0.clone()
    }
}
