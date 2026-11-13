# ImoutoLifeFantasyTrainer

An unofficial trainer for [妹!せいかつ～ファンタジー～](https://www.dlsite.com/maniax/work/=/product_id/RJ338582.html) that offers game speed control, battle skip, and editing of items, money, time, and character stats, with global hotkey support.

Read this in other languages: English | [中文](./README.zh.md) | [日本語](./README.ja.md)

## Features

- Multilingual support: Chinese, English, and Japanese.
- Multi-platform support: works with both DLsite and Steam.
- Game speed: fast-forwards the game at 3× speed.
- Battle skip: immediately win when a battle starts.
- Value editing: customize items, money, time, household policy, and character stats.
- Included address table: `launch.CT` can be opened in Cheat Engine.

## Download

Download from [GitHub Releases](https://github.com/xueelf/ImoutoLifeFantasyTrainer/releases/latest):

- `ImoutoLifeFantasyTrainer.exe`

Windows 10/11 is required. The UI supports Chinese, English, and Japanese, and follows the Windows system language by default.

## Usage

Double-click `ImoutoLifeFantasyTrainer.exe` to run the trainer. It attaches to the game automatically; use the menu with the mouse or the global hotkeys.

## Notice

This project uses the following tech stack:

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

The trainer uses extracted game assets (side images, program icon, fonts, sounds, etc.), and all rights belong to their respective owners. This project is for learning purposes only; please do not promote or abuse it.
