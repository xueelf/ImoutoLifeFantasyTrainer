# ImoutoLifeFantasyTrainer

ゲーム [『妹!せいかつ～ファンタジー～』](https://www.dlsite.com/maniax/work/=/product_id/RJ338582.html) の非公式トレーナーです。ゲーム速度の変更、戦闘スキップ、アイテム、所持金、時間、キャラクターステータスの編集、グローバルホットキーに対応しています。

他の言語で読む：[English](./README.md) | [中文](./README.zh.md) | 日本語

## 機能

- 多言語対応：中国語、英語、日本語に対応しています。
- マルチプラットフォーム対応：DLsite 版と Steam 版の両方で使用できます。
- ゲーム速度：ゲームを3倍速で動かします。
- 戦闘スキップ：戦闘に入るとすぐに勝利します。
- 数値編集：アイテム、所持金、時間、方針、キャラクターステータスを変更できます。
- アドレステーブル同梱：`launch.CT` ファイルは Cheat Engine で開けます。

## ダウンロード

[GitHub Releases](https://github.com/xueelf/ImoutoLifeFantasyTrainer/releases/latest) からダウンロードしてください：

- `ImoutoLifeFantasyTrainer.exe`

Windows 10/11 でのみ動作します。UI は中国語、英語、日本語に対応し、既定ではシステム言語に従います。

## 使い方

`ImoutoLifeFantasyTrainer.exe` をダブルクリックしてトレーナーを起動します。ゲームプロセスに自動的に接続されます。マウスでメニューをクリックするか、グローバルホットキーを使用できます。

## 声明

本プロジェクトは以下の技術スタックを使用しています：

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

トレーナーはゲーム内の抽出済みリソース（サイド画像、プログラムアイコン、フォント、効果音など）を使用しており、著作権はそれぞれの権利者に帰属します。本プロジェクトは学習目的のみです。商業的な宣伝や悪用はしないでください。
