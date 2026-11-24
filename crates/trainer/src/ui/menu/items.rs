//! 道具菜单：一级分组和二级编辑面板。

use eframe::egui::{self, Rect, Ui, Vec2};

use crate::config::pointer::{ItemGroup, MenuValues};
use crate::i18n::{self, Language};
use crate::ui::theme;

use super::widgets::{RowKind, field_value, main_panel, menu_row, popup_panel};
use super::{MENU_ROW_HEIGHT, MenuAction, MenuState, ROW_INSET};

pub(super) fn draw_items(
    ui: &mut Ui,
    values: &MenuValues,
    state: &mut MenuState,
    panels: &mut Vec<Rect>,
    language: Language,
) -> Option<MenuAction> {
    let panel = main_panel(ui, panels, ItemGroup::ALL.len());

    let mut y = panel.top();
    let mut submenu_anchor = None;
    for (index, group) in ItemGroup::ALL.into_iter().enumerate() {
        let rect = Rect::from_min_size(
            egui::pos2(panel.left() + ROW_INSET, y),
            Vec2::new(panel.width() - ROW_INSET * 2.0, MENU_ROW_HEIGHT),
        );
        let response = menu_row(
            ui,
            rect,
            i18n::item_group_label(language, group),
            None,
            RowKind::Group,
            true,
            ("item-group", index),
        );
        if response.hovered() {
            theme::screentone(ui.painter(), rect, 6.0, 0.7);
        }
        if index + 1 < ItemGroup::ALL.len() {
            theme::list_separator(ui.painter(), rect);
        }
        if state.open_submenu == Some(group) {
            // 子菜单锚在触发它的分组行上，弹出面板会自动避让窗口底部。
            submenu_anchor = Some(rect);
        }
        if response.clicked() {
            if state.open_submenu == Some(group) {
                state.open_submenu = None;
            } else {
                state.open_submenu = Some(group);
                state.open_select = None;
            }
        }
        y += MENU_ROW_HEIGHT;
    }

    if let (Some(anchor), Some(group)) = (submenu_anchor, state.open_submenu) {
        let fields = group.fields();
        let sub = popup_panel(ui, panels, anchor, fields.len());

        let mut row_y = sub.top();
        for (index, &field) in fields.iter().enumerate() {
            let rect = Rect::from_min_size(
                egui::pos2(sub.left() + ROW_INSET, row_y),
                Vec2::new(sub.width() - ROW_INSET * 2.0, MENU_ROW_HEIGHT),
            );
            let current = values.get(field);
            let is_check = field.is_check();
            // 复选框用“值是否大于 0”表示开关，点击后取反。
            let value = (!is_check).then(|| field_value(language, field, values));
            let row_id = if is_check {
                ("item-check", index)
            } else {
                ("item-prompt", index)
            };
            let enabled = !field.is_read_only();
            let response = menu_row(
                ui,
                rect,
                i18n::field_label(language, field),
                value.as_deref(),
                if is_check {
                    RowKind::Check(current > 0)
                } else {
                    RowKind::Prompt
                },
                enabled,
                row_id,
            );
            if response.hovered() {
                theme::screentone(ui.painter(), rect, 6.0, 0.7);
            }
            if index + 1 < fields.len() {
                theme::list_separator(ui.painter(), rect);
            }
            if response.clicked() && enabled {
                return Some(if is_check {
                    MenuAction::Check {
                        field,
                        checked: current <= 0,
                    }
                } else {
                    MenuAction::Prompt(field)
                });
            }
            row_y += MENU_ROW_HEIGHT;
        }
    }

    None
}
