//! 启动动画：轮播 logo，最多 30 秒后自动进入主界面。

use eframe::egui::{self, Color32, TextureHandle, Ui};
use std::time::{Duration, Instant};

use crate::i18n::{self, Language};
use crate::services::assets::AssetService;

pub struct SplashState {
    pub active: bool,
    index: usize,
    ticks: usize,
    textures: Vec<TextureHandle>,
    last_tick: Instant,
}

impl SplashState {
    pub fn new(ctx: &egui::Context, assets: &AssetService) -> Self {
        let mut textures = Vec::new();
        let mut index = 0;
        while let Some(bytes) = assets.logo(index) {
            let Ok(image) = assets.load_color_image(bytes) else {
                break;
            };
            textures.push(ctx.load_texture(
                format!("splash-{index}"),
                image,
                egui::TextureOptions::LINEAR,
            ));
            index += 1;
        }

        Self {
            active: true,
            index: 0,
            ticks: 0,
            textures,
            last_tick: Instant::now(),
        }
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn tick(&mut self, allow_timeout: bool) -> bool {
        if !self.active {
            return false;
        }
        if self.last_tick.elapsed() < Duration::from_secs(1) {
            return false;
        }

        self.last_tick = Instant::now();
        self.ticks += 1;

        // 还没找到游戏时最多等 30 秒，超时后弹窗提示并退出。
        if allow_timeout && self.ticks >= 30 {
            self.active = false;
            return true;
        }

        if !self.textures.is_empty() {
            self.index = (self.index + 1) % self.textures.len();
        }
        false
    }

    pub fn draw(&self, ui: &mut Ui, language: Language) -> bool {
        let text = i18n::strings(language);
        let rect = ui.max_rect();
        ui.painter().rect_filled(rect, 0.0, Color32::BLACK);

        if let Some(texture) = self.textures.get(self.index) {
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .layout(egui::Layout::centered_and_justified(
                        egui::Direction::TopDown,
                    )),
                |ui| {
                    ui.add(
                        egui::Image::new(texture)
                            .max_size(rect.size() * 0.785)
                            .maintain_aspect_ratio(true),
                    );
                },
            );
        }

        ui.painter().text(
            egui::pos2(rect.center().x, rect.bottom() - 26.0),
            egui::Align2::CENTER_CENTER,
            text.splash_praying,
            crate::ui::theme::font(12.0),
            Color32::from_rgb(216, 212, 201),
        );

        let close_rect = egui::Rect::from_min_size(
            egui::pos2(rect.right() - 60.0, rect.top() + 10.0),
            egui::vec2(36.0, 36.0),
        );
        let close = ui.interact(
            close_rect,
            ui.scope_id().with("splash-close"),
            egui::Sense::click(),
        );
        let close_center = egui::pos2(close_rect.right() - 8.0, close_rect.center().y);
        let color = if close.hovered() {
            Color32::WHITE
        } else {
            Color32::from_rgb(216, 212, 201)
        };
        for (dx, dy) in [(-8.0, -8.0), (-8.0, 8.0)] {
            ui.painter().line_segment(
                [
                    close_center + egui::vec2(dx, dy),
                    close_center + egui::vec2(-dx, -dy),
                ],
                egui::Stroke::new(2.0, color),
            );
        }
        let close = close.on_hover_text(text.close);

        let drag = ui.interact(
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(rect.width() - 70.0, 46.0)),
            ui.scope_id().with("splash-drag"),
            egui::Sense::drag(),
        );
        if drag.drag_started() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }

        close.clicked() || ui.input(|input| input.key_pressed(egui::Key::Escape))
    }
}
