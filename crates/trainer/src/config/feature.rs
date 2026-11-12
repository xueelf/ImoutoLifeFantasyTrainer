//! 功能开关定义：热键、效果名和说明。

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FeatureAction {
    InstantWin,
    Invincibility,
    TripleSpeed,
    TimeStop,
    UnlimitedBaths,
    UnlimitedTea,
}

#[derive(Clone, Copy)]
pub enum Hotkey {
    F1,
    F2,
    F3,
    F9,
    F10,
    F11,
}

impl Hotkey {
    pub fn label(self) -> &'static str {
        match self {
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
        }
    }
}

#[derive(Clone, Copy)]
pub struct FeatureDefinition {
    pub action: FeatureAction,
    pub hotkey: Hotkey,
    pub effect: &'static str,
    pub description: &'static str,
}

pub const FEATURES: &[FeatureDefinition] = &[
    // 顺序决定功能列表和热键注册顺序。
    FeatureDefinition {
        action: FeatureAction::InstantWin,
        hotkey: Hotkey::F1,
        effect: "见面5秒结束战斗",
        description: "战斗直接胜利",
    },
    FeatureDefinition {
        action: FeatureAction::Invincibility,
        hotkey: Hotkey::F2,
        effect: "AT力场",
        description: "免疫战斗伤害",
    },
    FeatureDefinition {
        action: FeatureAction::TripleSpeed,
        hotkey: Hotkey::F3,
        effect: "三倍速",
        description: "游戏速度三倍",
    },
    FeatureDefinition {
        action: FeatureAction::TimeStop,
        hotkey: Hotkey::F9,
        effect: "砸瓦鲁多",
        description: "停止游戏时间",
    },
    FeatureDefinition {
        action: FeatureAction::UnlimitedBaths,
        hotkey: Hotkey::F10,
        effect: "无限洗澡",
        description: "洗澡不限次数",
    },
    FeatureDefinition {
        action: FeatureAction::UnlimitedTea,
        hotkey: Hotkey::F11,
        effect: "无限泡茶",
        description: "泡茶不限次数",
    },
];
