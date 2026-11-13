//! 三语文案和菜单标签。所有界面文字都从这里取。

use crate::config::feature::{FeatureAction, FeatureDefinition};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Zh,
    En,
    Ja,
}

impl Language {
    pub const ALL: [Self; 3] = [Self::Zh, Self::En, Self::Ja];

    pub fn detect() -> Self {
        // SAFETY: GetUserDefaultUILanguage 没有前置条件，直接返回当前系统 UI 语言。
        let langid = unsafe { windows::Win32::Globalization::GetUserDefaultUILanguage() };
        // 只区分中文和日文，其他系统语言统一回退英语。
        match langid & 0x03ff {
            0x04 => Self::Zh,
            0x11 => Self::Ja,
            _ => Self::En,
        }
    }

    pub fn native_name(self) -> &'static str {
        match self {
            Self::Zh => "中文",
            Self::En => "English",
            Self::Ja => "日本語",
        }
    }
}

// 每个字段按中、英、日顺序声明，编译期生成三套静态字符串。
macro_rules! define_strings {
    ($($field:ident => [$zh:literal, $en:literal, $ja:literal]),* $(,)?) => {
        pub struct Strings {
            $(pub $field: &'static str,)*
        }

        const ZH: Strings = Strings { $($field: $zh,)* };
        const EN: Strings = Strings { $($field: $en,)* };
        const JA: Strings = Strings { $($field: $ja,)* };
    };
}

define_strings! {
    title => ["妹相随修改器", "Imouto Life Fantasy Trainer", "妹相随トレーナー"],
    subtitle => ["黑白世界的缤纷冒险", "A colorful adventure in a monochrome world", "モノクロ世界のカラフルな冒険"],
    connected => ["已连接 Game.exe", "Connected to Game.exe", "Game.exe に接続済み"],
    multi_detected => ["检测到 {} 个游戏进程", "{} game processes detected", "{} 個のゲームプロセスを検出"],
    game_found => ["已找到 Game.exe", "Game.exe found", "Game.exe を検出"],
    waiting_game => ["等待游戏", "Waiting for game", "ゲームを待機中"],
    close => ["关闭", "Close", "閉じる"],
    features => ["功能清单", "Features", "機能一覧"],
    hint_attached => ["点击行或按热键切换", "Click a row or press a hotkey", "行をクリック、またはホットキーで切替"],
    hint_multi => ["多开", "Multiple", "多重起動"],
    hint_reading => ["读取中", "Reading", "読み込み中"],
    hint_disconnected => ["未连接", "Not connected", "未接続"],
    footer_hotkeys => ["F1–F11 全局热键", "F1–F11 global hotkeys", "F1–F11 グローバルホットキー"],
    footer_multi => ["只保留一个游戏窗口", "Keep only one game window", "ゲームウィンドウを1つだけ残す"],
    footer_refreshing => ["每秒刷新", "Refreshes every second", "毎秒更新"],
    footer_waiting => ["等待游戏进程", "Waiting for game process", "ゲームプロセスを待機中"],
    language => ["语言", "Language", "言語"],
    current => ["当前", "Current", "現在"],
    date => ["日期", "Date", "日付"],
    time => ["时间", "Time", "時間"],
    schedule => ["日程", "Schedule", "スケジュール"],
    weekday => ["星期", "Weekday", "曜日"],
    edit_title => ["修改{}", "Edit {}", "{}を変更"],
    current_value => ["当前 {}", "Current {}", "現在 {}"],
    new_value => ["新数值", "New value", "新しい値"],
    integer_hint => ["只能输入整数", "Integers only", "整数のみ入力可"],
    cancel => ["取消", "Cancel", "キャンセル"],
    ok => ["确定", "OK", "OK"],
    integer_error => ["请输入整数。", "Please enter an integer.", "整数を入力してください。"],
    about => ["关于", "About", "このアプリについて"],
    tech_stack => ["第三方组件", "Third-party components", "サードパーティコンポーネント"],
    about_powered => ["Powered by Rust", "Powered by Rust", "Powered by Rust"],
    copyright => ["Copyright © 2025-2026 by Yuki", "Copyright © 2025-2026 by Yuki", "Copyright © 2025-2026 by Yuki"],
    license => ["MIT License", "MIT License", "MIT License"],
    notice => ["提示", "Notice", "お知らせ"],
    error_prefix => ["操作失败。", "Operation failed.", "操作に失敗しました。"],
    timeout_message => ["哥哥喜欢放置 Play 是么？", "Does Onii-chan enjoy idle play?", "お兄ちゃんは放置プレイが好きなの？"],
    timeout_hint => ["点击确定关闭修改器", "Click OK to close the trainer", "OKを押すと閉じます"],
    splash_praying => ["妹妹祈祷中", "Sister is praying", "妹はお祈り中"],
    disconnected_title => ["电波传达不到哦", "The signal can't reach you", "電波が届かないみたい"],
    disconnected_hint => ["请先启动 Game.exe 或 GamePro.exe", "Start Game.exe or GamePro.exe first", "先に Game.exe または GamePro.exe を起動してください"],
    waiting_title => ["正在读取游戏数据……", "Reading game data...", "ゲームデータを読み込み中……"],
    waiting_hint => ["已找到游戏，正在验证字段", "Game found; verifying fields", "ゲームを検出。項目を確認中"],
    multi_message => ["{}个妹妹同时在家，哥哥要选哪一个？", "{} sisters are home at once. Which one will you choose?", "{}人の妹が家にいます。お兄ちゃんはどっちを選ぶ？"],
    multi_hint => ["只保留一个游戏窗口，修改器会自动重新附加", "Keep only one game window; the trainer will reattach automatically", "ゲームウィンドウを1つだけ残してください。自動で再接続します"],
    hotkeys_busy => ["以下全局热键被占用：{}", "These global hotkeys are already in use: {}", "以下のグローバルホットキーは使用中です：{}"],
    hotkeys_failed => ["注册全局热键失败：{}", "Failed to register global hotkeys: {}", "グローバルホットキーの登録に失敗しました：{}"],
    missing_pointer => ["菜单缺少指针配置：{}", "Missing pointer configuration for menu: {}", "メニューのポインター設定がありません：{}"],
    scan_error => ["扫描游戏进程失败：{}", "Failed to scan game processes: {}", "ゲームプロセスのスキャンに失敗しました：{}"],
    write_error => ["写入失败。\n\n{}", "Write failed.\n\n{}", "書き込みに失敗しました。\n\n{}"],
    speed_restore_failed => ["游戏速度恢复失败，当前速度可能未确认。\n\n{}", "Failed to restore game speed; the current speed is unconfirmed.\n\n{}", "ゲーム速度の復元に失敗しました。現在の速度は未確認です。\n\n{}"],
    value_out_of_range => ["数值超出范围：{} ~ {}", "Value out of range: {} to {}", "値が範囲外です：{} ～ {}"],
}

pub fn strings(language: Language) -> &'static Strings {
    match language {
        Language::Zh => &ZH,
        Language::En => &EN,
        Language::Ja => &JA,
    }
}

// 菜单树里存中文 caption，这里按语言查表。查不到就保留中文。
macro_rules! nav_labels {
    ($($zh:literal => [$en:literal, $ja:literal]),* $(,)?) => {
        pub fn nav_label(language: Language, zh: &'static str) -> &'static str {
            match language {
                Language::Zh => zh,
                Language::En => match zh {
                    $($zh => $en,)*
                    _ => zh,
                },
                Language::Ja => match zh {
                    $($zh => $ja,)*
                    _ => zh,
                },
            }
        }
    };
}

nav_labels! {
    "道具" => ["Items", "アイテム"],
    "状态" => ["Status", "ステータス"],
    "帮助" => ["Help", "ヘルプ"],
    "茶水类" => ["Drinks", "お茶"],
    "礼物类" => ["Gifts", "プレゼント"],
    "书籍类" => ["Books", "本"],
    "家具类" => ["Furniture", "家具"],
    "H类" => ["H Items", "H系"],
    "绿茶" => ["Green Tea", "緑茶"],
    "咖啡" => ["Coffee", "コーヒー"],
    "红茶" => ["Black Tea", "紅茶"],
    "安眠茶" => ["Sleep Tea", "安眠茶"],
    "壮阳茶" => ["Stimulant", "精力剤"],
    "媚药" => ["Love Potion", "媚薬"],
    "避孕药" => ["After Pill", "アフターピル"],
    "零食" => ["Snack", "お菓子"],
    "礼物" => ["Gift", "プレゼント"],
    "冒险的书" => ["Adventure Book", "冒険の書"],
    "色色的书" => ["Naughty Book", "えっちな本"],
    "小薄本" => ["Thin Book", "薄いブック"],
    "安眠枕" => ["Soft Pillow", "安眠枕"],
    "羽毛被" => ["Feather Mattress", "羽毛布団"],
    "哑铃" => ["Dumbbell", "ダンベル"],
    "铁木屐" => ["Iron Geta", "鉄下駄"],
    "安全套" => ["Condom", "コンドーム"],
    "跳蛋" => ["Vibrator", "バイブ"],
    "按摩棒" => ["Dildo", "電マ"],
    "菊穴按摩棒" => ["Anal Dildo", "アナルバイブ"],
    "触手" => ["Tentacle", "触手"],
    "时间" => ["Time", "時間"],
    "日" => ["Day", "日"],
    "时" => ["Hour", "時"],
    "分" => ["Minute", "分"],
    "周" => ["Week", "週"],
    "周一" => ["Mon", "月"],
    "周二" => ["Tue", "火"],
    "周三" => ["Wed", "水"],
    "周四" => ["Thu", "木"],
    "周五" => ["Fri", "金"],
    "周六" => ["Sat", "土"],
    "周日" => ["Sun", "日"],
    "方针" => ["Policy", "方針"],
    "妹妹" => ["Sister", "妹"],
    "哥哥" => ["Brother", "兄"],
    "心情" => ["Mood", "機嫌"],
    "信赖" => ["Trust", "信頼"],
    "性趣" => ["Interest", "性興味"],
    "性欲" => ["Lust", "性欲"],
    "健康" => ["Health", "健康"],
    "行动力" => ["Energy", "行動力"],
    "精力" => ["Stamina", "精力"],
    "金钱" => ["Money", "所持金"],
    "开始之时" => ["Beginning", "開始時"],
    "往常的生活" => ["Usual Life", "いつもの生活"],
    "打工生活" => ["Part-time", "バイト生活"],
    "便当生活" => ["Bento Life", "お弁当生活"],
    "节约生活" => ["Frugal Life", "節約生活"],
    "读书周" => ["Reading", "読書週間"],
    "肉体改造" => ["Training", "肉体改造"],
    "大人的阶段" => ["Adult Stage", "大人の階段"],
    "健康的每一天" => ["Healthy", "健康的な毎日"],
    "奢侈期" => ["Luxury", "贅沢期"],
    "禁欲生活" => ["Abstinence", "禁欲生活"],
    "糜烂生活" => ["Decadent", "糜爛生活"],
    "烦恼全开" => ["Worries", "悩み全開"],
    "妹妹生活" => ["Sister Life", "妹生活"],
    "暑假！" => ["Summer!", "夏休み！"],
    "合宿！" => ["Camp!", "合宿！"],
    "妹妹旅行" => ["Sister Trip", "妹旅行"],
    "哥哥LOVE" => ["Brother LOVE", "お兄ちゃんLOVE"],
    "很开心" => ["Very Happy", "とても嬉しい"],
    "普通" => ["Normal", "普通"],
    "不开心" => ["Unhappy", "不機嫌"],
    "即将爆炸" => ["About to Burst", "爆発寸前"],
    "爆炸" => ["Burst", "爆発"],
    "文档" => ["Documentation", "ドキュメント"],
    "源码" => ["Source", "ソース"],
    "关于" => ["About", "このアプリ"],
}

pub fn day_value(language: Language, day: i32) -> String {
    match language {
        Language::Zh | Language::Ja => format!("{day}日"),
        Language::En => format!("Day {day}"),
    }
}

pub fn weekday(language: Language, day: i32) -> &'static str {
    const ZH: [&str; 7] = ["周一", "周二", "周三", "周四", "周五", "周六", "周日"];
    const EN: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const JA: [&str; 7] = ["月", "火", "水", "木", "金", "土", "日"];
    // 游戏天数从 1 开始，按 7 天循环。非法值先钳到 1。
    let index = (day.max(1) - 1).rem_euclid(7) as usize;
    match language {
        Language::Zh => ZH[index],
        Language::En => EN[index],
        Language::Ja => JA[index],
    }
}

pub fn feature_text(
    language: Language,
    definition: &FeatureDefinition,
) -> (&'static str, &'static str) {
    match language {
        Language::Zh => (definition.effect, definition.description),
        Language::En => match definition.action {
            FeatureAction::InstantWin => ("Battle Ends in 5 Seconds", "Win the battle instantly"),
            FeatureAction::Invincibility => ("AT Field", "Immune to battle damage"),
            FeatureAction::TripleSpeed => ("Three Times Faster", "Triple game speed"),
            FeatureAction::TimeStop => ("The World", "Stop game time"),
            FeatureAction::UnlimitedBaths => ("Unlimited Baths", "Unlimited baths"),
            FeatureAction::UnlimitedTea => ("Unlimited Tea", "Unlimited tea"),
        },
        Language::Ja => match definition.action {
            FeatureAction::InstantWin => ("出会って5秒で決着", "戦闘に即勝利"),
            FeatureAction::Invincibility => ("ATフィールド", "戦闘ダメージ無効"),
            FeatureAction::TripleSpeed => ("3倍速", "ゲーム速度3倍"),
            FeatureAction::TimeStop => ("ザ・ワールド", "ゲーム時間を停止"),
            FeatureAction::UnlimitedBaths => ("入浴し放題", "入浴回数無制限"),
            FeatureAction::UnlimitedTea => ("お茶し放題", "お茶回数無制限"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weekdays_follow_day_and_wrap() {
        assert_eq!(weekday(Language::Zh, 1), "周一");
        assert_eq!(weekday(Language::Zh, 7), "周日");
        assert_eq!(weekday(Language::Zh, 8), "周一");
        assert_eq!(weekday(Language::En, 2), "Tue");
        assert_eq!(weekday(Language::Ja, 5), "金");
    }

    #[test]
    fn day_value_is_localized() {
        assert_eq!(day_value(Language::Zh, 46), "46日");
        assert_eq!(day_value(Language::En, 46), "Day 46");
        assert_eq!(day_value(Language::Ja, 46), "46日");
    }

    #[test]
    fn feature_names_keep_source_references() {
        fn find(action: FeatureAction) -> &'static FeatureDefinition {
            crate::config::feature::FEATURES
                .iter()
                .find(|feature| feature.action == action)
                .expect("feature definition")
        }

        let winner = find(FeatureAction::InstantWin);
        assert_eq!(
            feature_text(Language::En, winner).0,
            "Battle Ends in 5 Seconds"
        );
        assert_eq!(feature_text(Language::Ja, winner).0, "出会って5秒で決着");

        let speed = find(FeatureAction::TripleSpeed);
        assert_eq!(feature_text(Language::En, speed).0, "Three Times Faster");

        let time_stop = find(FeatureAction::TimeStop);
        assert_eq!(feature_text(Language::En, time_stop).0, "The World");
        assert_eq!(feature_text(Language::Ja, time_stop).0, "ザ・ワールド");
    }

    #[test]
    fn nav_labels_are_translated() {
        assert_eq!(nav_label(Language::Zh, "道具"), "道具");
        assert_eq!(nav_label(Language::En, "道具"), "Items");
        assert_eq!(nav_label(Language::Ja, "道具"), "アイテム");
        assert_eq!(nav_label(Language::En, "未知"), "未知");
        assert_eq!(nav_label(Language::En, "媚药"), "Love Potion");
        assert_eq!(nav_label(Language::En, "小薄本"), "Thin Book");
        assert_eq!(nav_label(Language::En, "羽毛被"), "Feather Mattress");
        assert_eq!(nav_label(Language::En, "安眠枕"), "Soft Pillow");
        assert_eq!(nav_label(Language::Ja, "壮阳茶"), "精力剤");
        assert_eq!(nav_label(Language::Ja, "按摩棒"), "電マ");
        assert_eq!(nav_label(Language::En, "性趣"), "Interest");
        assert_eq!(nav_label(Language::Ja, "心情"), "機嫌");
        assert_eq!(nav_label(Language::Ja, "性趣"), "性興味");
    }
}
