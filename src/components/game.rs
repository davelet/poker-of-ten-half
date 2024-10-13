use bevy::prelude::*;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component, PartialEq, Eq)]
pub enum ButtonOnGamePage {
    BackMenuButton,
    ExitGameButton,
    RenewGameButton,
    DealPokerButton,
}
/// 牌数，A 2,3,4,5,6,7,8,9, 10，J Q K
#[derive(Component, Debug, Clone, Copy)]
struct CardRank {
    pub rank: i8,
}

// 卡牌点数
#[derive(Component, Debug, Clone, Copy)]
struct CardPoint {
    pub point_type: PokerReducedPoint,
}

#[derive(Component, Debug, Clone, Copy)]
struct CardType {
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

#[derive(Component, Debug, Clone)]

pub struct PokerCard {
    pub rank: CardRank,
    pub point: CardPoint,
    pub suite: CardType,
}

impl PokerCard {
    pub fn new(rank: i8, suite: PokerSuiteEnum) -> Self {
        PokerCard {
            rank: CardRank { rank },
            suite: CardType { suite },
            point: CardPoint {
                point_type: match rank {
                    1..=10 => PokerReducedPoint::NaturalPoint(rank as u8),
                    _ => PokerReducedPoint::HalfPoint,
                },
            },
        }
    }

    pub fn joker(big: bool) -> Self {
        Self {
            rank: CardRank { rank: if big { -1 } else { -2 } },
            suite: CardType { suite: PokerSuiteEnum::Joker },
            point: CardPoint { point_type: PokerReducedPoint::HalfPoint },
        }
    }
}

#[derive(Component, Debug)]
pub enum PokerCardStatus {
    OnDeck,  // 初始化尚未洗牌
    OnTable, // 牌在牌堆上
    OnHand,  // 牌在手牌上
    Used,    // 牌被使用
}

// #[derive(Component)]
// pub struct PokerCardOrder(pub u32);

// impl Default for PokerCardOrder {
//     fn default() -> Self {
//         Self(0)
//     }
// }

impl std::fmt::Display for PokerReducedPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PokerReducedPoint::NaturalPoint(point) => write!(f, "{}", point),
            PokerReducedPoint::HalfPoint => write!(f, "0.5"),
        }
    }
}

/// 比赛豆子
#[derive(Component)]
pub struct MatchCoin;

#[derive(Component)]
pub enum DeckArea {
    AVAIL,
    USED,
}
