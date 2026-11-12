//! 帮助菜单：语言切换、文档、源码和关于。

use eframe::egui::{self, Rect, Ui, Vec2};

use crate::config::nav::{NavNode, NavType};
use crate::i18n::{self, Language};
use crate::ui::theme;

use super::widgets::{
    RowKind, find_node, menu_row, option_row, panel_height, panel_shell_plain, popup_panel,
};
use super::{
    MENU_ROW_HEIGHT, MENU_WIDTH, MENU_X, MENU_Y, MenuAction, MenuState, OPTION_INSET, ROW_INSET,
};
pub(super) fn draw_help(
    ui: &mut Ui,
    nav: &[NavNode],
    state: &mut MenuState,
    panels: &mut Vec<Rect>,
    language: Language,
) -> Option<MenuAction> {
    let text = i18n::strings(language);
    let help = find_node(nav, "help")?;
    let rows = help.children.len() + 1;
    let height = panel_height(rows);
    let panel = Rect::from_min_size(egui::pos2(MENU_X, MENU_Y), Vec2::new(MENU_WIDTH, height));
    panel_shell_plain(ui, panels, panel);

    let mut y = panel.top();
    // 语言行固定放在最上面，和下面的文档/源码/关于分开。
    let language_rect = Rect::from_min_size(
        egui::pos2(panel.left() + ROW_INSET, y),
        Vec2::new(panel.width() - ROW_INSET * 2.0, MENU_ROW_HEIGHT),
    );
    let response = menu_row(
        ui,
        language_rect,
        text.language,
        None,
        RowKind::Group,
        true,
        ("language", 0),
    );
    if response.hovered() {
        theme::screentone(ui.painter(), language_rect, 6.0, 0.7);
    }
    theme::list_separator(ui.painter(), language_rect);
    if response.clicked() {
        if state.open_select.as_deref() == Some("language") {
            state.open_select = None;
        } else {
            state.open_select = Some("language".to_string());
        }
    }
    y += MENU_ROW_HEIGHT;

    for (index, node) in help.children.iter().enumerate() {
        let rect = Rect::from_min_size(
            egui::pos2(panel.left() + ROW_INSET, y),
            Vec2::new(panel.width() - ROW_INSET * 2.0, MENU_ROW_HEIGHT),
        );
        let is_action = matches!(node.kind, NavType::Action(_));
        let response = menu_row(
            ui,
            rect,
            i18n::nav_label(language, node.caption),
            None,
            if is_action {
                RowKind::Action
            } else {
                RowKind::Group
            },
            true,
            ("help", index),
        );
        if response.hovered() {
            theme::screentone(ui.painter(), rect, 6.0, 0.7);
        }
        if index + 1 < help.children.len() {
            theme::list_separator(ui.painter(), rect);
        }
        if response.clicked()
            && let NavType::Action(command) = node.kind
        {
            return Some(MenuAction::Command(command));
        }
        y += MENU_ROW_HEIGHT;
    }

    if state.open_select.as_deref() == Some("language") {
        return draw_language_panel(ui, panels, language_rect, language);
    }
    None
}

fn draw_language_panel(
    ui: &mut Ui,
    panels: &mut Vec<Rect>,
    anchor: Rect,
    language: Language,
) -> Option<MenuAction> {
    let panel = popup_panel(ui, panels, anchor, Language::ALL.len());

    for (index, option) in Language::ALL.iter().enumerate() {
        let rect = Rect::from_min_size(
            egui::pos2(
                panel.left() + OPTION_INSET,
                panel.top() + index as f32 * MENU_ROW_HEIGHT,
            ),
            Vec2::new(panel.width() - OPTION_INSET * 2.0, MENU_ROW_HEIGHT),
        );
        let response = option_row(
            ui,
            rect,
            option.native_name(),
            *option == language,
            true,
            ("language-option", index),
        );
        if response.clicked() {
            return Some(MenuAction::Language(*option));
        }
    }
    None
}
