//! 程序入口：单实例检查、窗口配置和 eframe 启动。

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::undocumented_unsafe_blocks)]

#[cfg(not(all(target_os = "windows", target_pointer_width = "32")))]
compile_error!("ImoutoLifeFantasyTrainer only supports 32-bit Windows targets");

mod app;
mod config;
mod i18n;
mod services;
mod ui;

use eframe::egui;
use std::sync::Arc;

use crate::app::TrainerApp;
use crate::i18n::Language;
use crate::services::assets::AssetService;
use crate::services::single_instance::SingleInstance;
use crate::services::win32;

fn load_window_icon(assets: &AssetService) -> Option<egui::IconData> {
    let bytes = assets.icon()?;
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    Some(egui::IconData {
        rgba: image.as_raw().clone(),
        width: image.width(),
        height: image.height(),
    })
}

fn main() -> eframe::Result {
    // 单实例锁同时保护全局热键和游戏内存写入，避免两个修改器争用资源。
    let _instance = match SingleInstance::acquire() {
        Ok(Some(instance)) => instance,
        Ok(None) => {
            win32::message_box("提示", "修改器已经在运行。");
            return Ok(());
        }
        Err(error) => {
            win32::message_box("错误", &format!("创建单实例锁失败：{error}"));
            return Ok(());
        }
    };

    let assets = AssetService::new();
    let language = Language::detect();
    let title = i18n::strings(language).title;
    // 固定尺寸、无系统标题栏，窗口装饰全部由 egui 绘制。
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([config::WINDOW_WIDTH, config::WINDOW_HEIGHT])
        .with_resizable(false)
        .with_decorations(false);

    if let Some(icon) = load_window_icon(&assets) {
        viewport = viewport.with_icon(Arc::new(icon));
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let result = eframe::run_native(
        title,
        options,
        Box::new(move |cc| Ok(Box::new(TrainerApp::new(cc, assets, language)))),
    );

    // 没有控制台窗口时，启动错误只能靠消息框告诉用户。
    if let Err(error) = &result {
        win32::message_box("错误", &format!("修改器启动失败：{error}"));
    }

    result
}
