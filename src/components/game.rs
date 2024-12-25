use bevy::prelude::*;

use crate::MatchState;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component, PartialEq, Eq)]
pub enum ButtonOnGamePage {
    BackMenuButton,    // 返回前页(B)
    ExitGameButton,    // 退出游戏(Q)
    RenewGameButton,   // 重新开始游戏(R)
    DealPokerButton,   // 给我发牌(N)
    StopDealingButton, // 跳过我（J)
}
/// 牌数，A 2,3,4,5,6,7,8,9, 10，J Q K
#[derive(Component, Debug, Clone, Copy)]
pub struct CardRank {
    pub rank: i8,
}

// 卡牌点数
#[derive(Component, Debug, Clone, Copy)]
pub struct CardPoint {
    pub point_type: PokerReducedPoint,
}

#[derive(Component, Debug, Clone, Copy)]
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

impl PokerReducedPoint {
    pub fn point(&self) -> f32 {
        match self {
            PokerReducedPoint::NaturalPoint(point) => *point as f32,
            PokerReducedPoint::HalfPoint => 0.5,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]

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

#[derive(Component, Debug, PartialEq, Eq)]
pub enum PokerCardStatus {
    OnDeck,  // 初始化尚未洗牌
    OnTable, // 牌在牌堆上
    OnHand,  // 牌在手牌上
    Used,    // 牌被使用
}

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

#[derive(Component, PartialEq, Eq)]
pub enum DeckArea {
    AVAIL,
    USED,
}

#[derive(Component, Debug)]
pub struct PlayerPointShown(pub MatchState);

// 记录需要跳过的玩家，直接进入其他玩家轮次
#[derive(Component, Debug, Clone, Copy)]
pub struct SkipTurn(pub MatchState);

#[derive(Component, Debug)]
pub struct PokerCardTypeSlotWithIndex(pub u8);

#[derive(Component, Debug)]
pub struct PokerCardRankSlotWithIndex(pub u8);

#[derive(Component, Debug)]
pub struct SinglePokerAreaSlot(pub MatchState);
