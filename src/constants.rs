use bevy::{
    asset::Handle,
    color::{palettes::css::*, Color},
    prelude::Image,
    text::Font,
    ui::BackgroundColor,
};

pub const MENU_TITLE: &str = "推十点半";
pub const APP_FONT: &str = "Alibaba-PuHuiTi-Regular.otf";
pub const APP_ICON: &str = "poker-title.png";
pub static mut HAN_FONT_OPTION: Option<Handle<Font>> = None;
pub static mut APP_ICON_IMAGE: Option<Handle<Image>> = None;
pub static mut PANIC_FLAG: bool = false;

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const PLAYER_PLOT_TEXT: &str = "玩家：";
pub const BLANK_STRING: &str = "";
pub const START_BUTTON_TEXT: &str = "来一局(C)";
pub const EXIT_BUTTON_TEXT: &str = "退出程序(Q)";
pub const RETURN_TO_MENU_BUTTON_TEXT: &str = "返回前页(B)";
pub const RENEW_GAME_BUTTON_TEXT: &str = "重新开始(R)";
pub const DEAL_POKER_BUTTON_TEXT: &str = "发一张牌(N)";
pub const STOP_DEALING_BUTTON_TEXT: &str = "我牌够了(J)";
pub const POKER_EMPTY_SLOT_TEXT: &str = "空位";
pub const TOTAL_POINT_TEXT: &str = "总点数：";
pub const POKER_DECK_AVAILABLE_TEXT: &str = "牌堆";
pub const POKER_USED_TEXT: &str = "已弃牌";
pub const START_BUTTON_NORMAL_COLOR: BackgroundColor = BackgroundColor(Color::Srgba(CRIMSON));
pub const START_BUTTON_HOVER_COLOR: BackgroundColor = BackgroundColor(Color::Srgba(GREEN));
// pub const START_BUTTON_CLICK_COLOR: BackgroundColor = BackgroundColor(Color::Srgba(CRIMSON));
