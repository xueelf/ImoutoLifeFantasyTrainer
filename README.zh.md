# ImoutoLifeFantasyTrainer

游戏[『妹!せいかつ～ファンタジー～』](https://www.dlsite.com/maniax/work/=/product_id/RJ338582.html)的非官方修改器，可为游戏提供变速、战斗跳过，以及提供道具、金钱、时间和角色状态的修改，支持全局热键。

使用其他语言阅读：[English](./README.md) | 中文 | [日本語](./README.ja.md)

## 特性

- 多语言支持：支持中文、英语和日语。
- 多平台兼容：DLsite 与 Steam 平台均可使用。
- 游戏变速：程序三倍速快进。
- 战斗跳过：游戏进入战斗阶段直接获得胜利。
- 数值修改：道具、金钱、时间、方针、角色状态均可自定义。
- 附带地址表：`launch.CT` 可供 Cheat Engine 查看。

## 下载

从 [GitHub Releases](https://github.com/xueelf/ImoutoLifeFantasyTrainer/releases/latest) 下载：

- `ImoutoLifeFantasyTrainer.exe`

要求 Windows 10/11，程序界面支持中文、英语、日语，默认跟随 Windows 系统语言。

## 使用

双击 `ImoutoLifeFantasyTrainer.exe` 运行修改器，程序会自动附加游戏进程，可使用鼠标点击菜单或全局热键

## 声明

本项目使用到了以下技术栈：

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

修改器使用到了游戏内的拆包资源（侧边图、程序图标、字体、音效等），版权归相关方所属，本项目仅供学习用途，请勿肆意宣传滥用。
