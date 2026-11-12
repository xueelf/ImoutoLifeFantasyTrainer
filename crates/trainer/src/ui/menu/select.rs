//! 下拉选择面板，选项较多时自动分两列。

use std::collections::HashMap;

use eframe::egui::{self, Rect, Ui, Vec2};

use crate::config::nav::NavNode;
use crate::i18n::{self, Language};

use super::widgets::{option_row, popup_panel};
use super::{MENU_ROW_HEIGHT, MenuAction, OPTION_GAP, OPTION_INSET};
pub(super) fn draw_select_panel(
    ui: &mut Ui,
    node: &NavNode,
    values: &HashMap<String, i32>,
    anchor: Rect,
    panels: &mut Vec<Rect>,
    language: Language,
) -> Option<MenuAction> {
    let options = &node.options;
    if options.is_empty() {
        return None;
    }
    // 选项超过 10 个时分两列，避免面板超出窗口。
    let columns = if options.len() > 10 { 2 } else { 1 };
    let rows = options.len().div_ceil(columns);
    let panel = popup_panel(ui, panels, anchor, rows);

    let current = values.get(&node.path).copied().unwrap_or(0);
    let column_width =
        (panel.width() - OPTION_INSET * 2.0 - OPTION_GAP * (columns.saturating_sub(1)) as f32)
            / columns as f32;
    for (index, option) in options.iter().enumerate() {
        let column = index % columns;
        let line = index / columns;
        let rect = Rect::from_min_size(
            egui::pos2(
                panel.left() + OPTION_INSET + column as f32 * (column_width + OPTION_GAP),
                panel.top() + line as f32 * MENU_ROW_HEIGHT,
            ),
            Vec2::new(column_width, MENU_ROW_HEIGHT),
        );
        let selected = option.value == current;
        let enabled = !option.read_only;
        let response = option_row(
            ui,
            rect,
            i18n::nav_label(language, option.caption),
            selected,
            enabled,
            ("select-option", &node.path, index),
        );
        if response.clicked() && enabled {
            return Some(MenuAction::Select {
                path: node.path.clone(),
                value: option.value,
            });
        }
    }
    None
}
