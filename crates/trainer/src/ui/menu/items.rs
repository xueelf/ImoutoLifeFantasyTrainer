//! 道具菜单：一级分组和二级编辑面板。

use std::collections::HashMap;

use eframe::egui::{self, Rect, Ui, Vec2};

use crate::config::nav::{NavNode, NavType};
use crate::i18n::{self, Language};
use crate::ui::theme;

use super::widgets::{RowKind, find_node, main_panel, menu_row, node_value, popup_panel};
use super::{MENU_ROW_HEIGHT, MenuAction, MenuState, ROW_INSET};
pub(super) fn draw_items(
    ui: &mut Ui,
    nav: &[NavNode],
    values: &HashMap<String, i32>,
    state: &mut MenuState,
    panels: &mut Vec<Rect>,
    language: Language,
) -> Option<MenuAction> {
    let items = find_node(nav, "item_list")?;
    let panel = main_panel(ui, panels, items.children.len());

    let mut y = panel.top();
    let mut submenu_anchor = None;
    for (index, node) in items.children.iter().enumerate() {
        let rect = Rect::from_min_size(
            egui::pos2(panel.left() + ROW_INSET, y),
            Vec2::new(panel.width() - ROW_INSET * 2.0, MENU_ROW_HEIGHT),
        );
        let response = menu_row(
            ui,
            rect,
            i18n::nav_label(language, node.caption),
            None,
            RowKind::Group,
            true,
            ("item-group", index),
        );
        if response.hovered() {
            theme::screentone(ui.painter(), rect, 6.0, 0.7);
        }
        if index + 1 < items.children.len() {
            theme::list_separator(ui.painter(), rect);
        }
        if state.open_submenu.as_deref() == Some(node.path.as_str()) {
            // 子菜单锚在触发它的分组行上，弹出面板会自动避让窗口底部。
            submenu_anchor = Some(rect);
        }
        if response.clicked() {
            if state.open_submenu.as_deref() == Some(node.path.as_str()) {
                state.open_submenu = None;
            } else {
                state.open_submenu = Some(node.path.clone());
                state.open_select = None;
            }
        }
        y += MENU_ROW_HEIGHT;
    }

    if let Some(anchor) = submenu_anchor
        && let Some(group) = state
            .open_submenu
            .as_deref()
            .and_then(|path| find_node(nav, path))
    {
        let sub = popup_panel(ui, panels, anchor, group.children.len());

        let mut row_y = sub.top();
        for (index, child) in group.children.iter().enumerate() {
            let rect = Rect::from_min_size(
                egui::pos2(sub.left() + ROW_INSET, row_y),
                Vec2::new(sub.width() - ROW_INSET * 2.0, MENU_ROW_HEIGHT),
            );
            let current = values.get(&child.path).copied().unwrap_or(0);
            let is_check = matches!(child.kind, NavType::Check);
            // 复选框用“值是否大于 0”表示开关，点击后取反。
            let value = (!is_check).then(|| node_value(language, child, values));
            let row_id = if is_check {
                ("item-check", index)
            } else {
                ("item-prompt", index)
            };
            let response = menu_row(
                ui,
                rect,
                i18n::nav_label(language, child.caption),
                value.as_deref(),
                if is_check {
                    RowKind::Check(current > 0)
                } else {
                    RowKind::Prompt
                },
                !child.read_only,
                row_id,
            );
            if response.hovered() {
                theme::screentone(ui.painter(), rect, 6.0, 0.7);
            }
            if index + 1 < group.children.len() {
                theme::list_separator(ui.painter(), rect);
            }
            if response.clicked() && !child.read_only {
                return Some(if is_check {
                    MenuAction::Check {
                        path: child.path.clone(),
                        checked: current <= 0,
                    }
                } else {
                    MenuAction::Prompt {
                        path: child.path.clone(),
                        caption: i18n::nav_label(language, child.caption),
                    }
                });
            }
            row_y += MENU_ROW_HEIGHT;
        }
    }

    None
}
