//! 菜单行、选项行和面板的共用绘制组件。

use std::collections::HashMap;

use eframe::egui::{
    self, Align2, Color32, CornerRadius, Rect, Sense, Stroke, StrokeKind, Ui, Vec2,
};

use crate::config::nav::{self, NavNode, NavType};
use crate::i18n::{self, Language};
use crate::ui::theme;

use super::{
    BODY_BOTTOM, LABEL_VALUE_GAP, MENU_ROW_HEIGHT, MENU_WIDTH, MENU_X, MENU_Y, OPTION_FONT_SIZE,
    OPTION_LABEL_X, OPTION_RADIO_X, ROW_TEXT_INSET, SIDE_X,
};
pub(crate) enum RowKind {
    Group,
    Prompt,
    Check(bool),
    Action,
}

pub(crate) fn menu_row(
    ui: &mut Ui,
    rect: Rect,
    label: &str,
    value: Option<&str>,
    kind: RowKind,
    enabled: bool,
    id: impl std::hash::Hash + std::fmt::Debug,
) -> egui::Response {
    // 禁用行只保留 hover，避免误触。
    let response = ui.interact(
        rect,
        ui.scope_id().with(("menu-row", id)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let color = if enabled { theme::INK } else { theme::MUTED };
    let label_font = theme::font(14.0);
    let mut label_x = rect.left() + ROW_TEXT_INSET;
    if let RowKind::Check(checked) = kind {
        let box_rect = Rect::from_min_size(
            egui::pos2(rect.left() + ROW_TEXT_INSET, rect.center().y - 8.0),
            Vec2::splat(16.0),
        );
        ui.painter().rect(
            box_rect,
            CornerRadius::from(2.0),
            if checked { theme::INK } else { Color32::WHITE },
            Stroke::new(1.5, theme::INK),
            StrokeKind::Inside,
        );
        if checked {
            ui.painter().line_segment(
                [
                    egui::pos2(box_rect.left() + 2.0, box_rect.center().y),
                    egui::pos2(box_rect.left() + 4.5, box_rect.bottom() - 2.5),
                ],
                Stroke::new(1.2, theme::PAPER),
            );
            ui.painter().line_segment(
                [
                    egui::pos2(box_rect.left() + 4.5, box_rect.bottom() - 2.5),
                    egui::pos2(box_rect.right() - 2.0, box_rect.top() + 2.5),
                ],
                Stroke::new(1.2, theme::PAPER),
            );
        }
        label_x += 22.0;
    }
    ui.painter().text(
        egui::pos2(label_x, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        label_font.clone(),
        color,
    );

    if matches!(kind, RowKind::Prompt)
        && let Some(value) = value
    {
        let label_width = theme::measure(ui, label, label_font).x;
        let value_font = theme::font(14.0);
        let value_width = theme::measure(ui, value, value_font.clone()).x;
        let value_x = (label_x + label_width + LABEL_VALUE_GAP)
            .min(rect.right() - ROW_TEXT_INSET - value_width);
        ui.painter().text(
            egui::pos2(value_x, rect.center().y),
            Align2::LEFT_CENTER,
            value,
            value_font,
            color,
        );
    }

    match kind {
        RowKind::Group => {
            ui.painter().text(
                egui::pos2(rect.right() - ROW_TEXT_INSET, rect.center().y),
                Align2::RIGHT_CENTER,
                "›",
                theme::mono(12.0),
                theme::MUTED,
            );
        }
        RowKind::Action => {
            ui.painter().text(
                egui::pos2(rect.right() - ROW_TEXT_INSET, rect.center().y),
                Align2::RIGHT_CENTER,
                "↗",
                theme::mono(12.0),
                theme::MUTED,
            );
        }
        RowKind::Prompt | RowKind::Check(_) => {}
    }

    response.on_hover_cursor(if enabled {
        egui::CursorIcon::PointingHand
    } else {
        egui::CursorIcon::Default
    })
}

pub(crate) fn option_row(
    ui: &mut Ui,
    rect: Rect,
    label: &str,
    selected: bool,
    enabled: bool,
    id: impl std::hash::Hash + std::fmt::Debug,
) -> egui::Response {
    let response = ui.interact(
        rect,
        ui.scope_id().with(("menu-option", id)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    circle(
        ui.painter(),
        egui::pos2(rect.left() + OPTION_RADIO_X, rect.center().y),
        5.5,
        selected,
        enabled,
    );
    ui.painter().text(
        egui::pos2(rect.left() + OPTION_LABEL_X, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        theme::font(OPTION_FONT_SIZE),
        if enabled { theme::INK } else { theme::MUTED },
    );
    if response.hovered() && enabled {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        theme::screentone(ui.painter(), rect, 6.0, 0.7);
    }
    response
}

pub(crate) fn panel_height(rows: usize) -> f32 {
    rows as f32 * MENU_ROW_HEIGHT
}

pub(crate) fn panel_shell_plain(ui: &mut Ui, panels: &mut Vec<Rect>, rect: Rect) -> Rect {
    theme::panel(ui.painter(), rect, 12.0, Vec2::new(4.0, 4.0));
    panels.push(rect);
    rect
}

pub(crate) fn main_panel(ui: &mut Ui, panels: &mut Vec<Rect>, rows: usize) -> Rect {
    let rect = Rect::from_min_size(
        egui::pos2(MENU_X, MENU_Y),
        Vec2::new(MENU_WIDTH, panel_height(rows)),
    );
    panel_shell_plain(ui, panels, rect)
}

pub(crate) fn popup_panel(ui: &mut Ui, panels: &mut Vec<Rect>, anchor: Rect, rows: usize) -> Rect {
    let height = panel_height(rows);
    // 下拉面板贴在右侧，并按底部剩余空间向上收，避免超出内容区。
    let max_y = BODY_BOTTOM - height - 2.0;
    let y = (anchor.top() - 10.0).clamp(MENU_Y, max_y.max(MENU_Y));
    let rect = Rect::from_min_size(egui::pos2(SIDE_X, y), Vec2::new(MENU_WIDTH, height));
    panel_shell_plain(ui, panels, rect)
}

pub(crate) fn draw_inline_value(
    ui: &Ui,
    x: f32,
    y: f32,
    label: &str,
    value: &str,
    value_color: Color32,
) {
    let label_font = theme::font(12.0);
    ui.painter().text(
        egui::pos2(x, y),
        Align2::LEFT_CENTER,
        label,
        label_font.clone(),
        theme::MUTED,
    );
    let label_width = theme::measure(ui, label, label_font).x;
    ui.painter().text(
        egui::pos2(x + label_width + LABEL_VALUE_GAP, y),
        Align2::LEFT_CENTER,
        value,
        theme::font(14.0),
        value_color,
    );
}

pub(crate) fn draw_section_title(ui: &mut Ui, panel: Rect, y: f32, title: &str, meta: &str) {
    ui.painter().text(
        egui::pos2(panel.left() + 16.0, y + 4.0),
        Align2::LEFT_CENTER,
        title,
        theme::font(14.0),
        theme::INK,
    );
    ui.painter().text(
        egui::pos2(panel.right() - 10.0, y + 4.0),
        Align2::RIGHT_CENTER,
        meta,
        theme::mono(10.0),
        theme::MUTED,
    );
    ui.painter().line_segment(
        [
            egui::pos2(panel.left() + 16.0, y + 14.0),
            egui::pos2(panel.right() - 10.0, y + 14.0),
        ],
        Stroke::new(1.0, theme::LINE),
    );
}

pub(crate) fn dotted_underline(ui: &Ui, x0: f32, x1: f32, y: f32) {
    let mut x = x0;
    while x < x1 {
        let end = (x + 4.0).min(x1);
        ui.painter().line_segment(
            [egui::pos2(x, y), egui::pos2(end, y)],
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(27, 27, 27, 80)),
        );
        x += 6.0;
    }
}

pub(crate) fn circle(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    filled: bool,
    enabled: bool,
) {
    let color = if enabled { theme::INK } else { theme::MUTED };
    painter.circle_stroke(center, radius, Stroke::new(1.1, color));
    if filled {
        painter.circle_filled(center, radius - 1.5, color);
    }
}

pub(crate) fn node_value(
    language: Language,
    node: &NavNode,
    values: &HashMap<String, i32>,
) -> String {
    let value = values.get(&node.path).copied().unwrap_or(0);
    match node.kind {
        NavType::Select => node
            .options
            .iter()
            .find(|option| option.value == value)
            .map(|option| i18n::nav_label(language, option.caption).to_string())
            .unwrap_or_else(|| value.to_string()),
        _ => nav::format_count(node.format, value),
    }
}

pub(crate) fn find_node<'a>(nodes: &'a [NavNode], path: &str) -> Option<&'a NavNode> {
    for node in nodes {
        if node.path == path {
            return Some(node);
        }
        if let Some(found) = find_node(&node.children, path) {
            return Some(found);
        }
    }
    None
}
