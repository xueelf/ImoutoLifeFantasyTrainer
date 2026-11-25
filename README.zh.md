# ImoutoLifeFantasyTrainer

游戏[『妹!せいかつ～ファンタジー～』](https://www.dlsite.com/maniax/work/=/product_id/RJ338582.html)的非官方修改器，支持变速、战斗跳过，以及道具、金钱、时间和角色状态的修改，并支持全局热键。

使用其他语言阅读：[English](./README.md) | 中文 | [日本語](./README.ja.md)

## 特性

- 多语言支持：支持中文、英语和日语。
- 多平台兼容：DLsite 与 Steam 版本均可使用。
- 游戏变速：以三倍速运行游戏。
- 战斗跳过：进入战斗后直接获胜。
- 数值修改：可修改道具、金钱、时间、方针和角色状态。
- 附带地址表：可用 Cheat Engine 打开 `launch.CT` 文件。

## 下载

从 [GitHub Releases](https://github.com/xueelf/ImoutoLifeFantasyTrainer/releases/latest) 下载：

- `ImoutoLifeFantasyTrainer.exe`

仅支持 Windows 10/11 运行。界面支持中文、英语和日语，默认跟随系统语言。

如果仍在使用更旧的 Windows 版本，请下载 [v1 版本](https://github.com/xueelf/ImoutoLifeFantasyTrainer/releases/tag/v1.0.0)。

## 使用

双击 `ImoutoLifeFantasyTrainer.exe` 运行修改器，程序会自动附加游戏进程。可以使用鼠标点击菜单，也可以使用全局热键。

## 声明

本项目使用以下技术栈：

- [egui / eframe](https://github.com/emilk/egui)
- [detour](https://github.com/darfink/detour-rs)
- [iced-x86](https://github.com/icedland/iced)
- [image](https://github.com/image-rs/image)
- [windows-rs](https://github.com/microsoft/windows-rs)
- [global-hotkey](https://github.com/tauri-apps/global-hotkey)
- [anyhow](https://github.com/dtolnay/anyhow)
- [include_dir](https://github.com/Michael-F-Bryan/include_dir)
- [fastrand](https://github.com/smol-rs/fastrand)
- [absoIute/Speedhack](https://github.com/absoIute/Speedhack)
- [Cheat Engine](https://github.com/cheat-engine/cheat-engine)

修改器使用了游戏内的拆包资源（侧边图、程序图标、字体、音效等），版权归相关方所有。本项目仅供学习用途，请勿用于商业宣传或滥用。
