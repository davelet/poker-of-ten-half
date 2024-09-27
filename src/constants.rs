use bevy::{asset::Handle, color::{palettes::css::{CRIMSON, LIGHT_BLUE}, Color}, text::Font, ui::BackgroundColor};



pub const MENU_TITLE: &str = "推十点半";
pub const APP_FONT: &str = "Alibaba-PuHuiTi-Regular.otf";
pub static mut HAN_FONT_OPTION: Option<Handle<Font>> = None;

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const START_BUTTON_NORMAL_COLOR: BackgroundColor = BackgroundColor(Color::Srgba(CRIMSON));
pub const START_BUTTON_HOVER_COLOR: BackgroundColor = BackgroundColor(Color::Srgba(LIGHT_BLUE));
// pub const START_BUTTON_CLICK_COLOR: BackgroundColor = BackgroundColor(Color::Srgba(CRIMSON));