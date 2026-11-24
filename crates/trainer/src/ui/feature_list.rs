//! 已连接状态下的功能清单。

use eframe::egui::{self, Align2, Rect, Sense, Stroke, Ui};

use crate::config::feature::{FeatureAction, FeatureDefinition};
use crate::i18n::{self, Language};
use crate::ui::theme;

pub struct FeatureView {
    pub definition: &'static FeatureDefinition,
    pub is_on: bool,
}

pub fn draw(
    ui: &mut Ui,
    rect: Rect,
    views: &[FeatureView],
    interactive: bool,
    language: Language,
) -> Option<FeatureAction> {
    let mut clicked = None;
    let count = views.len().max(1) as f32;
    // 按行数均分高度并向下取偶，避免边框落在半像素上。
    let row_height = ((rect.height() / count) / 2.0).floor() * 2.0;
    let list_top = rect.top() + ((rect.height() - row_height * count) / 2.0).floor();
    let key_font = theme::mono(12.0);

    for (index, view) in views.iter().enumerate() {
        let (effect, description) = i18n::feature_text(language, view.definition);
        let row = Rect::from_min_size(
            egui::pos2(rect.left(), list_top + index as f32 * row_height),
            egui::vec2(rect.width(), row_height),
        );
        let response = if interactive {
            ui.interact(row, ui.scope_id().with(("feature", index)), Sense::click())
                .on_hover_cursor(egui::CursorIcon::PointingHand)
        } else {
            ui.interact(row, ui.scope_id().with(("feature", index)), Sense::hover())
        };

        if view.is_on {
            theme::screentone(ui.painter(), row.shrink2(egui::vec2(0.0, 2.0)));
        } else if response.hovered() && interactive {
            ui.painter().rect_filled(
                row.shrink2(egui::vec2(0.0, 2.0)),
                0.0,
                egui::Color32::from_rgba_unmultiplied(27, 27, 27, 7),
            );
        }

        if index + 1 < views.len() {
            ui.painter().line_segment(
                [row.left_bottom(), row.right_bottom()],
                Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(215, 211, 200, 150),
                ),
            );
        }

        let name_font = theme::font(14.0);
        let desc_font = theme::font(12.0);
        let name_pos = row.left_top() + egui::vec2(0.0, 4.0);
        let desc_pos = row.left_top() + egui::vec2(0.0, 24.0);
        ui.painter().text(
            name_pos,
            Align2::LEFT_TOP,
            effect,
            name_font.clone(),
            theme::INK,
        );
        ui.painter().text(
            desc_pos,
            Align2::LEFT_TOP,
            description,
            desc_font,
            if view.is_on { theme::INK } else { theme::MUTED },
        );

        if view.is_on {
            // 开启状态用网点底纹和波浪下划线标记，替代常规开关。
            let width = theme::measure(ui, effect, name_font).x;
            theme::wavy_line(
                ui.painter(),
                name_pos.x,
                name_pos.x + width,
                row.top() + 22.0,
                theme::INK,
            );
        }

        let key_rect = Rect::from_min_size(
            egui::pos2(row.right() - 46.0, row.center().y - 14.0),
            egui::vec2(46.0, 28.0),
        );
        theme::keycap(
            ui.painter(),
            key_rect,
            view.definition.hotkey.label(),
            view.is_on,
            key_font.clone(),
        );

        if response.clicked() {
            clicked = Some(view.definition.action);
        }
    }

    clicked
}
