use bevy::prelude::*;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub enum ButtonOnGamePage {
    BackMenuButton,
    ExitGameButton,
    RenewGameButton,
    DealPokerButton,
}
/// 牌数，A 2,3,4,5,6,7,8,9, 10，J Q K
#[derive(Component)]
pub struct CardRank {
    pub rank: i8,
}

// 卡牌点数
#[derive(Component)]
pub struct CardPoint {
    pub point_type: PokerReducedPoint,
}

#[derive(Component)]
pub struct CardType {
    pub suite: PokerSuiteEnum,
}

/// 红桃（Hearts）
/// 黑桃（Spades）
/// 方块（Diamonds）
/// 梅花（Clubs）
#[derive(Clone, Copy, Debug)]
pub enum PokerSuiteEnum {
    Spade,
    Heart,
    Diamond,
    Club,
    Joker,
}

#[derive(Clone, Copy, Debug)]
pub enum PokerReducedPoint {
    NaturalPoint(u8),
    HalfPoint, // 1/2
}

impl std::fmt::Display for PokerReducedPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PokerReducedPoint::NaturalPoint(point) => write!(f, "{}", point),
            PokerReducedPoint::HalfPoint => write!(f, "0.5"),
        }
    }
}


//
//

/// 比赛豆子
#[derive(Component)]
pub struct MatchCoin;