/// 一个可编辑字段。`path` 是菜单里的稳定键，`field_offset` 是相对 root 的偏移。
#[derive(Clone, Copy)]
pub struct PointerChain {
    pub path: &'static str,
    pub field_offset: u32,
}

pub const BATTLE_WINNER: PointerChain = PointerChain {
    path: "battle.winner",
    field_offset: 0x55C,
};
pub const BATTLE_SCORE: PointerChain = PointerChain {
    path: "battle.battle_score",
    field_offset: 0x74,
};
pub const TIME_DAY: PointerChain = PointerChain {
    path: "status.time.day",
    field_offset: 0x280,
};
pub const TIME_HOUR: PointerChain = PointerChain {
    path: "status.time.hour",
    field_offset: 0x10,
};
pub const TIME_MINUTE: PointerChain = PointerChain {
    path: "status.time.minute",
    field_offset: 0x14,
};
pub const TIME_WEEK: PointerChain = PointerChain {
    path: "status.time.week",
    field_offset: 0x18,
};
pub const TOOK_BATH: PointerChain = PointerChain {
    path: "status.took_bath",
    field_offset: 0x118,
};
pub const HAD_TEA: PointerChain = PointerChain {
    path: "status.had_tea",
    field_offset: 0x124,
};

// 所有字段都挂在同一个 root 下：[[global] + 0]，
// global 的地址是 module_base + CURRENT_OFFSET。
pub const CHAINS: &[PointerChain] = &[
    BATTLE_WINNER,
    BATTLE_SCORE,
    PointerChain {
        path: "item_list.drinks.green_tea",
        field_offset: 0x168,
    },
    PointerChain {
        path: "item_list.drinks.coffee",
        field_offset: 0x16C,
    },
    PointerChain {
        path: "item_list.drinks.black_tea",
        field_offset: 0x170,
    },
    PointerChain {
        path: "item_list.drinks.sleep_tea",
        field_offset: 0x174,
    },
    PointerChain {
        path: "item_list.drinks.stimulant",
        field_offset: 0x178,
    },
    PointerChain {
        path: "item_list.drinks.love_potion",
        field_offset: 0x17C,
    },
    PointerChain {
        path: "item_list.drinks.after_pill",
        field_offset: 0x180,
    },
    PointerChain {
        path: "item_list.presents.sweets",
        field_offset: 0x444,
    },
    PointerChain {
        path: "item_list.presents.present",
        field_offset: 0x448,
    },
    PointerChain {
        path: "item_list.books.adventure_book",
        field_offset: 0x44C,
    },
    PointerChain {
        path: "item_list.books.naughty_book",
        field_offset: 0x450,
    },
    PointerChain {
        path: "item_list.books.doujinshi",
        field_offset: 0x454,
    },
    PointerChain {
        path: "item_list.household_items.soft_pillow",
        field_offset: 0x458,
    },
    PointerChain {
        path: "item_list.household_items.feather_mattress",
        field_offset: 0x45C,
    },
    PointerChain {
        path: "item_list.household_items.dumbbell",
        field_offset: 0x460,
    },
    PointerChain {
        path: "item_list.household_items.iron_sandals",
        field_offset: 0x464,
    },
    PointerChain {
        path: "item_list.sex_items.condom",
        field_offset: 0x470,
    },
    PointerChain {
        path: "item_list.sex_items.vibrator",
        field_offset: 0x474,
    },
    PointerChain {
        path: "item_list.sex_items.dildo",
        field_offset: 0x478,
    },
    PointerChain {
        path: "item_list.sex_items.anal_dildo",
        field_offset: 0x47C,
    },
    PointerChain {
        path: "item_list.sex_items.tentacle",
        field_offset: 0x480,
    },
    TIME_DAY,
    TIME_HOUR,
    TIME_MINUTE,
    TIME_WEEK,
    PointerChain {
        path: "status.household_policy",
        field_offset: 0x274,
    },
    PointerChain {
        path: "status.imouto.trust",
        field_offset: 0x54,
    },
    PointerChain {
        path: "status.imouto.interest",
        field_offset: 0x5C,
    },
    PointerChain {
        path: "status.imouto.mood",
        field_offset: 0x58,
    },
    PointerChain {
        path: "status.imouto.lust",
        field_offset: 0x198,
    },
    PointerChain {
        path: "status.imouto.condition",
        field_offset: 0x50,
    },
    PointerChain {
        path: "status.oniichan.energy",
        field_offset: 0x38,
    },
    PointerChain {
        path: "status.oniichan.lust",
        field_offset: 0x48,
    },
    PointerChain {
        path: "status.oniichan.stamina",
        field_offset: 0x40,
    },
    PointerChain {
        path: "status.oniichan.money",
        field_offset: 0x1C,
    },
    TOOK_BATH,
    HAD_TEA,
];

/// 按菜单路径查找可编辑字段。
pub fn lookup(path: &str) -> Option<&'static PointerChain> {
    CHAINS.iter().find(|chain| chain.path == path)
}

/// 菜单可写入字段的安全范围，必须覆盖 probe 的字段验证范围。
pub fn bounds(path: &str) -> (i32, i32) {
    match path {
        "status.time.day" => (1, 999_999),
        "status.time.hour" => (0, 23),
        "status.time.minute" => (0, 59),
        "status.time.week" => (0, 6),
        "status.household_policy" => (1, 17),
        "status.oniichan.money" => (0, i32::MAX),
        path if path.starts_with("item_list.") => (0, 99_999),
        // 未知字段不加限制，调用方仍会先检查指针是否存在。
        _ => (i32::MIN, i32::MAX),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_stay_inside_probe_validation() {
        assert_eq!(bounds("status.time.day"), (1, 999_999));
        assert_eq!(bounds("status.time.hour"), (0, 23));
        assert_eq!(bounds("status.time.minute"), (0, 59));
        assert_eq!(bounds("status.time.week"), (0, 6));
        assert_eq!(bounds("status.oniichan.money").0, 0);
        assert_eq!(bounds("item_list.drinks.green_tea"), (0, 99_999));
    }
}
