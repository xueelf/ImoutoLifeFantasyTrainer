//! 应用常量、游戏进程信息和静态菜单数据。

pub mod feature;
pub mod pointer;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const VERSION_LABEL: &str = concat!("v", env!("CARGO_PKG_VERSION"));

// 窗口与内容区域尺寸。
pub const WINDOW_WIDTH: f32 = 560.0;
pub const WINDOW_HEIGHT: f32 = 440.0;
pub const HEADER_HEIGHT: f32 = 64.0;
pub const MENU_HEIGHT: f32 = 40.0;
pub const BODY_HEIGHT: f32 = 308.0;
pub const FOOTER_HEIGHT: f32 = 28.0;
pub const COVER_WIDTH: f32 = 204.0;
pub const COVER_HEIGHT: f32 = 272.0;
pub const COVER_PLACEHOLDER: &str = "card0001.jpg";

// 游戏进程、基址和注入 DLL。
pub const GAME_PROCESS_NAMES: &[&str] = &["Game.exe", "GamePro.exe"];

// Steam 中文版和 DLsite 日文版共用这个基址偏移，
// 最后再由 probe 读字段确认。
pub const CURRENT_OFFSET: u32 = 0x00DA_D67C;

// 两个已知版本里 WOLF 单实例检查（FindWindow）的 RVA。
// 用它识别游戏 exe。即使进程没有启动成功、字段还没有初始化，也能识别版本。
pub const SINGLE_INSTANCE_PATTERN_OFFSET: u32 = 0x0012_97D6;

pub const SPEEDHACK_DLL_NAME: &str = "Speedhack-x86.dll";

// 关于窗口链接。
pub const DOCUMENTATION_URL: &str = "https://blog.yuki.sh/posts/05064242ec86/";
pub const REPOSITORY_URL: &str = "https://github.com/xueelf/ImoutoLifeFantasyTrainer/";

pub struct TechStackItem {
    pub name: &'static str,
    pub copyright: &'static str,
    pub license: &'static str,
    pub url: &'static str,
}

pub const TECH_STACK: &[TechStackItem] = &[
    // 关于窗口和 README 共用的第三方组件列表。
    TechStackItem {
        name: "egui / eframe",
        copyright: "Copyright © Emil Ernerfeldt and contributors",
        license: "MIT OR Apache-2.0",
        url: "https://github.com/emilk/egui",
    },
    TechStackItem {
        name: "detour",
        copyright: "Copyright © 2017 Elliott Linder",
        license: "BSD-2-Clause",
        url: "https://github.com/darfink/detour-rs",
    },
    TechStackItem {
        name: "iced-x86",
        copyright: "Copyright © 2018-present iced project and contributors",
        license: "MIT",
        url: "https://github.com/icedland/iced",
    },
    TechStackItem {
        name: "image",
        copyright: "Copyright © image-rs contributors",
        license: "MIT OR Apache-2.0",
        url: "https://github.com/image-rs/image",
    },
    TechStackItem {
        name: "windows-rs",
        copyright: "Copyright © Microsoft Corporation",
        license: "MIT OR Apache-2.0",
        url: "https://github.com/microsoft/windows-rs",
    },
    TechStackItem {
        name: "global-hotkey",
        copyright: "Copyright © tauri-apps contributors",
        license: "Apache-2.0 OR MIT",
        url: "https://github.com/tauri-apps/global-hotkey",
    },
    TechStackItem {
        name: "anyhow",
        copyright: "Copyright © David Tolnay",
        license: "MIT OR Apache-2.0",
        url: "https://github.com/dtolnay/anyhow",
    },
    TechStackItem {
        name: "include_dir",
        copyright: "Copyright © Michael-F-Bryan",
        license: "MIT",
        url: "https://github.com/Michael-F-Bryan/include_dir",
    },
    TechStackItem {
        name: "fastrand",
        copyright: "Copyright © smol-rs contributors",
        license: "Apache-2.0 OR MIT",
        url: "https://github.com/smol-rs/fastrand",
    },
    TechStackItem {
        name: "absoIute/Speedhack",
        copyright: "Copyright © absolute-gamer",
        license: "MIT",
        url: "https://github.com/absoIute/Speedhack",
    },
];
