//! 游戏内存偏移、强类型字段定义与菜单静态结构。

pub const BATTLE_WINNER_OFFSET: u32 = 0x55C;
pub const BATTLE_SCORE_OFFSET: u32 = 0x74;
pub const TOOK_BATH_OFFSET: u32 = 0x118;
pub const HAD_TEA_OFFSET: u32 = 0x124;

/// 覆盖全部 35 个菜单字段（最大偏移 0x480 + 4 字节）的连续读取块大小。
pub const MENU_BLOCK_SIZE: usize = 0x484;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NavAction {
    Documentation,
    Repository,
    About,
}

impl NavAction {
    pub const ALL: [Self; 3] = [Self::Documentation, Self::Repository, Self::About];
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemGroup {
    Drinks,
    Presents,
    Books,
    HouseholdItems,
    SexItems,
}

impl ItemGroup {
    pub const ALL: [Self; 5] = [
        Self::Drinks,
        Self::Presents,
        Self::Books,
        Self::HouseholdItems,
        Self::SexItems,
    ];

    pub const fn fields(self) -> &'static [FieldId] {
        match self {
            Self::Drinks => &[
                FieldId::GreenTea,
                FieldId::Coffee,
                FieldId::BlackTea,
                FieldId::SleepTea,
                FieldId::Stimulant,
                FieldId::LovePotion,
                FieldId::AfterPill,
            ],
            Self::Presents => &[FieldId::Sweets, FieldId::Present],
            Self::Books => &[
                FieldId::AdventureBook,
                FieldId::NaughtyBook,
                FieldId::Doujinshi,
            ],
            Self::HouseholdItems => &[
                FieldId::SoftPillow,
                FieldId::FeatherMattress,
                FieldId::Dumbbell,
                FieldId::IronSandals,
            ],
            Self::SexItems => &[
                FieldId::Condom,
                FieldId::Vibrator,
                FieldId::Dildo,
                FieldId::AnalDildo,
                FieldId::Tentacle,
            ],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SelectOption {
    pub value: i32,
    pub read_only: bool,
}

const fn option(value: i32, read_only: bool) -> SelectOption {
    SelectOption { value, read_only }
}

pub const POLICY_OPTIONS: &[SelectOption] = &[
    option(1, true),
    option(2, false),
    option(3, false),
    option(4, false),
    option(5, false),
    option(6, false),
    option(7, false),
    option(8, false),
    option(9, false),
    option(10, false),
    option(11, false),
    option(12, false),
    option(13, false),
    option(14, false),
    option(15, true),
    option(16, true),
    option(17, false),
];

pub const INTEREST_OPTIONS: &[SelectOption] = &[
    option(10, false),
    option(8, false),
    option(6, false),
    option(4, false),
    option(2, false),
    option(0, false),
];

/// 所有菜单与状态面板涉及的游戏字段。
///
/// 所有字段都挂在同一个 root 下：`[[module_base + CURRENT_OFFSET] + 0]`。
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FieldId {
    GreenTea = 0,
    Coffee,
    BlackTea,
    SleepTea,
    Stimulant,
    LovePotion,
    AfterPill,
    Sweets,
    Present,
    AdventureBook,
    NaughtyBook,
    Doujinshi,
    SoftPillow,
    FeatherMattress,
    Dumbbell,
    IronSandals,
    Condom,
    Vibrator,
    Dildo,
    AnalDildo,
    Tentacle,
    TimeDay,
    TimeHour,
    TimeMinute,
    TimeWeek,
    HouseholdPolicy,
    ImoutoInterest,
    ImoutoTrust,
    ImoutoMood,
    ImoutoLust,
    ImoutoCondition,
    OniichanEnergy,
    OniichanLust,
    OniichanStamina,
    OniichanMoney,
}

pub const IMOUTO_GRID_FIELDS: [FieldId; 4] = [
    FieldId::ImoutoTrust,
    FieldId::ImoutoMood,
    FieldId::ImoutoLust,
    FieldId::ImoutoCondition,
];

pub const ONIICHAN_GRID_FIELDS: [FieldId; 4] = [
    FieldId::OniichanEnergy,
    FieldId::OniichanLust,
    FieldId::OniichanStamina,
    FieldId::OniichanMoney,
];

impl FieldId {
    pub const COUNT: usize = 35;

    pub const ALL: [Self; Self::COUNT] = [
        Self::GreenTea,
        Self::Coffee,
        Self::BlackTea,
        Self::SleepTea,
        Self::Stimulant,
        Self::LovePotion,
        Self::AfterPill,
        Self::Sweets,
        Self::Present,
        Self::AdventureBook,
        Self::NaughtyBook,
        Self::Doujinshi,
        Self::SoftPillow,
        Self::FeatherMattress,
        Self::Dumbbell,
        Self::IronSandals,
        Self::Condom,
        Self::Vibrator,
        Self::Dildo,
        Self::AnalDildo,
        Self::Tentacle,
        Self::TimeDay,
        Self::TimeHour,
        Self::TimeMinute,
        Self::TimeWeek,
        Self::HouseholdPolicy,
        Self::ImoutoInterest,
        Self::ImoutoTrust,
        Self::ImoutoMood,
        Self::ImoutoLust,
        Self::ImoutoCondition,
        Self::OniichanEnergy,
        Self::OniichanLust,
        Self::OniichanStamina,
        Self::OniichanMoney,
    ];

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn offset(self) -> u32 {
        match self {
            Self::GreenTea => 0x168,
            Self::Coffee => 0x16C,
            Self::BlackTea => 0x170,
            Self::SleepTea => 0x174,
            Self::Stimulant => 0x178,
            Self::LovePotion => 0x17C,
            Self::AfterPill => 0x180,
            Self::Sweets => 0x444,
            Self::Present => 0x448,
            Self::AdventureBook => 0x44C,
            Self::NaughtyBook => 0x450,
            Self::Doujinshi => 0x454,
            Self::SoftPillow => 0x458,
            Self::FeatherMattress => 0x45C,
            Self::Dumbbell => 0x460,
            Self::IronSandals => 0x464,
            Self::Condom => 0x470,
            Self::Vibrator => 0x474,
            Self::Dildo => 0x478,
            Self::AnalDildo => 0x47C,
            Self::Tentacle => 0x480,
            Self::TimeDay => 0x280,
            Self::TimeHour => 0x10,
            Self::TimeMinute => 0x14,
            Self::TimeWeek => 0x18,
            Self::HouseholdPolicy => 0x274,
            Self::ImoutoInterest => 0x5C,
            Self::ImoutoTrust => 0x54,
            Self::ImoutoMood => 0x58,
            Self::ImoutoLust => 0x198,
            Self::ImoutoCondition => 0x50,
            Self::OniichanEnergy => 0x38,
            Self::OniichanLust => 0x48,
            Self::OniichanStamina => 0x40,
            Self::OniichanMoney => 0x1C,
        }
    }

    /// 菜单可写入字段的安全范围，必须覆盖 probe 的字段验证范围。
    pub const fn bounds(self) -> (i32, i32) {
        match self {
            Self::TimeDay => (1, 999_999),
            Self::TimeHour => (0, 23),
            Self::TimeMinute => (0, 59),
            Self::TimeWeek => (0, 6),
            Self::HouseholdPolicy => (1, 17),
            Self::OniichanMoney => (0, i32::MAX),
            Self::GreenTea
            | Self::Coffee
            | Self::BlackTea
            | Self::SleepTea
            | Self::Stimulant
            | Self::LovePotion
            | Self::AfterPill
            | Self::Sweets
            | Self::Present
            | Self::AdventureBook
            | Self::NaughtyBook
            | Self::Doujinshi
            | Self::SoftPillow
            | Self::FeatherMattress
            | Self::Dumbbell
            | Self::IronSandals
            | Self::Condom
            | Self::Vibrator
            | Self::Dildo
            | Self::AnalDildo
            | Self::Tentacle => (0, 99_999),
            Self::ImoutoInterest
            | Self::ImoutoTrust
            | Self::ImoutoMood
            | Self::ImoutoLust
            | Self::ImoutoCondition
            | Self::OniichanEnergy
            | Self::OniichanLust
            | Self::OniichanStamina => (i32::MIN, i32::MAX),
        }
    }

    pub const fn is_check(self) -> bool {
        matches!(
            self,
            Self::SoftPillow
                | Self::FeatherMattress
                | Self::Dumbbell
                | Self::IronSandals
                | Self::Vibrator
                | Self::Dildo
                | Self::AnalDildo
                | Self::Tentacle
        )
    }

    pub const fn is_read_only(self) -> bool {
        matches!(self, Self::ImoutoCondition)
    }

    pub const fn options(self) -> &'static [SelectOption] {
        match self {
            Self::HouseholdPolicy => POLICY_OPTIONS,
            Self::ImoutoInterest => INTEREST_OPTIONS,
            _ => &[],
        }
    }

    pub fn format_value(self, value: i32) -> String {
        match self {
            Self::Condom => format!("x{value}"),
            _ => value.to_string(),
        }
    }
}

/// 固定容量的菜单字段快照，按 `FieldId` 索引 O(1) 存取，无堆分配。
#[derive(Clone, Copy)]
pub struct MenuValues([i32; FieldId::COUNT]);

impl Default for MenuValues {
    fn default() -> Self {
        Self([0; FieldId::COUNT])
    }
}

impl MenuValues {
    pub fn from_root_block(block: &[u8; MENU_BLOCK_SIZE]) -> Self {
        let mut values = Self::default();
        for field in FieldId::ALL {
            let offset = field.offset() as usize;
            let raw = [
                block[offset],
                block[offset + 1],
                block[offset + 2],
                block[offset + 3],
            ];
            values.set(field, i32::from_le_bytes(raw));
        }
        values
    }

    /// 用几组已知范围做弱校验。字段还没初始化时返回 false，调用方会等待重试。
    pub fn is_valid_game_state(&self) -> bool {
        (0..=23).contains(&self.get(FieldId::TimeHour))
            && (0..=59).contains(&self.get(FieldId::TimeMinute))
            && (0..=6).contains(&self.get(FieldId::TimeWeek))
            && (0..1_000_000).contains(&self.get(FieldId::TimeDay))
            && (0..100_000).contains(&self.get(FieldId::GreenTea))
            && (0..=17).contains(&self.get(FieldId::HouseholdPolicy))
            && self.get(FieldId::OniichanMoney) >= 0
    }

    pub const fn get(&self, field: FieldId) -> i32 {
        self.0[field.index()]
    }

    pub fn set(&mut self, field: FieldId, value: i32) {
        self.0[field.index()] = value;
    }

    pub fn clear(&mut self) {
        self.0 = [0; FieldId::COUNT];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn field_indices_and_offsets_are_consistent() {
        let mut offsets = HashSet::new();
        for (index, field) in FieldId::ALL.into_iter().enumerate() {
            assert_eq!(field.index(), index);
            assert!((field.offset() as usize) + 4 <= MENU_BLOCK_SIZE);
            assert!(
                offsets.insert(field.offset()),
                "duplicate offset for {field:?}"
            );
        }
    }

    #[test]
    fn bounds_stay_inside_probe_validation() {
        let mut base = MenuValues::default();
        base.set(FieldId::TimeDay, 1);
        base.set(FieldId::HouseholdPolicy, 1);
        assert!(base.is_valid_game_state());

        for field in FieldId::ALL {
            let (min, max) = field.bounds();
            let mut low = base;
            low.set(field, min);
            assert!(low.is_valid_game_state(), "min bound invalid for {field:?}");

            let mut high = base;
            high.set(field, max);
            assert!(
                high.is_valid_game_state(),
                "max bound invalid for {field:?}"
            );
        }
    }
}
