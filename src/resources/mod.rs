use bevy::{prelude::*, scene::ron::de};

use crate::components::prelude::PokerCard;

pub mod prelude {
    pub use super::*;
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MatchState {
    #[default]
    // Idle, // 等待发牌
    // Deal,      // 发牌
    SouthTurn, // 自己的轮次
    DealingSouth, // 给自己发牌
    EastTurn,     // 右手边轮次
    NorthTurn,    // 对面的轮次
    WestTurn,     // 左手边轮次
    Ended,        // 游戏结束
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DealPokerInMatch {
    Deal,
    #[default]
    End,
}

#[derive(Component)]
pub struct DealingPokerRecord;

#[derive(Resource, PartialEq, Eq)]
pub enum MatchPlayerCount {
    One = 1,
    Two = 2,
    Three = 3,
}

#[derive(Resource)]
pub enum MatchPokerSuitCount {
    One = 1,
    Two = 2,
    Three = 3,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct DeckTable {
    pub on_table: Vec<PokerCard>,
    pub off_table: Vec<PokerCard>,
}

impl Default for DeckTable {
    fn default() -> Self {
        Self { on_table: Default::default(), off_table: Default::default() }
    }
}

pub struct HanTextStyle(pub TextStyle);

impl Default for HanTextStyle {
    fn default() -> Self {
        if let Some(hf) = unsafe { crate::constants::HAN_FONT_OPTION.clone() } {
            return Self(TextStyle { font: hf, ..Default::default() });
        }
        Self(TextStyle { ..Default::default() })
    }
}

impl HanTextStyle {
    pub fn with_font_size(&mut self, font_size: f32) -> &mut Self {
        self.0.font_size = font_size;
        self
    }

    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.0.color = color;
        self
    }

    pub fn get_style(&self) -> TextStyle {
        self.0.clone()
    }
}

pub struct IconLoader(pub UiImage);

impl Default for IconLoader {
    fn default() -> Self {
        if let Some(hf) = unsafe { crate::constants::APP_ICON_IMAGE.clone() } {
            return Self(UiImage::new(hf));
        }
        Self(Default::default())
    }
}

impl IconLoader {
    pub fn get_image(&self) -> UiImage {
        self.0.clone()
    }
}
