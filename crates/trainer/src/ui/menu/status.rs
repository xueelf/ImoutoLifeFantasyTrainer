//! 状态面板：时间、方针、妹妹和哥哥的数值展示与编辑。

use std::collections::HashMap;

use eframe::egui::{self, Align2, Rect, Sense, Ui, Vec2};

use crate::config::nav::{NavNode, NavType};
use crate::i18n::{self, Language};
use crate::ui::theme;

use super::select::draw_select_panel;
use super::widgets::{
    dotted_underline, draw_inline_value, draw_section_title, find_node, node_value,
    panel_shell_plain,
};
use super::{MENU_WIDTH, MENU_X, MENU_Y, MenuAction, MenuState, STATUS_PANEL_HEIGHT};

// 状态面板所有分组共用：顶部 14px，标题下划线到内容 2px，组与组之间 12px。
const DATE_TITLE_Y: f32 = 14.0;
const LABEL_Y: f32 = 30.0;
const VALUE_OFFSET: f32 = 18.0;
const INNER_X: f32 = 16.0;
const COL_GAP: f32 = 16.0;
const POLICY_TITLE_Y: f32 = 82.0;
const POLICY_ROW_Y: f32 = 98.0;
const IMOUTO_TITLE_Y: f32 = 134.0;
const IMOUTO_FIRST_Y: f32 = 150.0;
const ROW_STEP: f32 = 24.0;
const ONIICHAN_TITLE_Y: f32 = 234.0;
const ONIICHAN_FIRST_Y: f32 = 250.0;
const PAIR_GAP: f32 = 12.0;
pub(super) fn draw_status(
    ui: &mut Ui,
    nav: &[NavNode],
    values: &HashMap<String, i32>,
    state: &mut MenuState,
    panels: &mut Vec<Rect>,
    language: Language,
) -> Option<MenuAction> {
    let text = i18n::strings(language);
    let panel = Rect::from_min_size(
        egui::pos2(MENU_X, MENU_Y),
        Vec2::new(MENU_WIDTH, STATUS_PANEL_HEIGHT),
    );
    panel_shell_plain(ui, panels, panel);

    let mut select_anchor = None;

    let day = find_node(nav, "status.time.day")?;
    let hour = find_node(nav, "status.time.hour")?;
    let minute = find_node(nav, "status.time.minute")?;
    let day_value = values.get(&day.path).copied().unwrap_or(0);
    let hour_value = values.get(&hour.path).copied().unwrap_or(0).clamp(0, 23);
    let minute_value = values.get(&minute.path).copied().unwrap_or(0).clamp(0, 59);
    let week_value = i18n::weekday(language, day_value);

    draw_section_title(
        ui,
        panel,
        panel.top() + DATE_TITLE_Y,
        text.schedule,
        "SCHEDULE",
    );

    let label_font = theme::font(12.0);
    let value_font = theme::font(14.0);
    let label_y = panel.top() + LABEL_Y;
    let value_y = label_y + VALUE_OFFSET;
    let inner_left = panel.left() + INNER_X;
    let col_width = (panel.width() - INNER_X * 2.0 - COL_GAP * 2.0) / 3.0;
    let day_x = inner_left;
    let time_x = day_x + col_width + COL_GAP;
    let week_x = time_x + col_width + COL_GAP;

    ui.painter().text(
        egui::pos2(day_x, label_y),
        Align2::LEFT_TOP,
        text.date,
        label_font.clone(),
        theme::MUTED,
    );
    let day_text = i18n::day_value(language, day_value);
    ui.painter().text(
        egui::pos2(day_x, value_y),
        Align2::LEFT_TOP,
        &day_text,
        value_font.clone(),
        theme::INK,
    );
    let day_width = theme::measure(ui, &day_text, value_font.clone()).x;
    dotted_underline(ui, day_x, day_x + day_width, value_y + 22.0);
    let day_rect =
        Rect::from_min_size(egui::pos2(day_x, label_y - 2.0), Vec2::new(col_width, 44.0));
    let response = ui.interact(day_rect, ui.scope_id().with("status-day"), Sense::click());
    if response.hovered() {
        theme::screentone(ui.painter(), day_rect, 6.0, 0.7);
    }
    if response.clicked() && !day.read_only {
        return Some(MenuAction::Prompt {
            path: day.path.clone(),
            caption: i18n::nav_label(language, day.caption),
        });
    }

    ui.painter().text(
        egui::pos2(time_x, label_y),
        Align2::LEFT_TOP,
        text.time,
        label_font.clone(),
        theme::MUTED,
    );
    let hour_text = format!("{hour_value:02}");
    let minute_text = format!("{minute_value:02}");
    ui.painter().text(
        egui::pos2(time_x, value_y),
        Align2::LEFT_TOP,
        &hour_text,
        value_font.clone(),
        theme::INK,
    );
    let hour_width = theme::measure(ui, &hour_text, value_font.clone()).x;
    let colon_x = time_x + hour_width + 2.0;
    ui.painter().text(
        egui::pos2(colon_x, value_y),
        Align2::LEFT_TOP,
        ":",
        value_font.clone(),
        theme::INK,
    );
    let minute_x = colon_x + 10.0;
    ui.painter().text(
        egui::pos2(minute_x, value_y),
        Align2::LEFT_TOP,
        &minute_text,
        value_font.clone(),
        theme::INK,
    );
    let minute_width = theme::measure(ui, &minute_text, value_font.clone()).x;
    dotted_underline(ui, time_x, minute_x + minute_width, value_y + 22.0);
    let hour_rect = Rect::from_min_size(
        egui::pos2(time_x, label_y - 2.0),
        Vec2::new(hour_width + 4.0, 40.0),
    );
    let response = ui.interact(hour_rect, ui.scope_id().with("status-hour"), Sense::click());
    if response.hovered() {
        theme::screentone(ui.painter(), hour_rect, 6.0, 0.7);
    }
    if response.clicked() && !hour.read_only {
        return Some(MenuAction::Prompt {
            path: hour.path.clone(),
            caption: i18n::nav_label(language, hour.caption),
        });
    }
    let minute_rect = Rect::from_min_size(
        egui::pos2(minute_x, label_y - 2.0),
        Vec2::new(minute_width + 4.0, 40.0),
    );
    let response = ui.interact(
        minute_rect,
        ui.scope_id().with("status-minute"),
        Sense::click(),
    );
    if response.hovered() {
        theme::screentone(ui.painter(), minute_rect, 6.0, 0.7);
    }
    if response.clicked() && !minute.read_only {
        return Some(MenuAction::Prompt {
            path: minute.path.clone(),
            caption: i18n::nav_label(language, minute.caption),
        });
    }

    ui.painter().text(
        egui::pos2(week_x, label_y),
        Align2::LEFT_TOP,
        text.weekday,
        label_font,
        theme::MUTED,
    );
    ui.painter().text(
        egui::pos2(week_x, value_y),
        Align2::LEFT_TOP,
        week_value,
        value_font,
        theme::INK,
    );

    draw_section_title(
        ui,
        panel,
        panel.top() + POLICY_TITLE_Y,
        i18n::nav_label(language, "方针"),
        "SELECT",
    );
    if let Some(policy) = find_node(nav, "status.household_policy") {
        let row = Rect::from_min_size(
            egui::pos2(panel.left() + INNER_X, panel.top() + POLICY_ROW_Y),
            Vec2::new(panel.width() - INNER_X * 2.0, 24.0),
        );
        let value = node_value(language, policy, values);
        draw_inline_value(
            ui,
            row.left(),
            row.center().y,
            text.current,
            &format!("{value} ▾"),
            theme::INK,
        );
        let response = ui.interact(row, ui.scope_id().with("status-policy"), Sense::click());
        if response.hovered() {
            theme::screentone(ui.painter(), row, 6.0, 0.7);
        }
        if state.open_select.as_deref() == Some(policy.path.as_str()) {
            select_anchor = Some(row);
        }
        if response.clicked() && !policy.read_only {
            state.open_select = Some(policy.path.clone());
            state.open_submenu = None;
            select_anchor = Some(row);
        }
    }

    draw_section_title(
        ui,
        panel,
        panel.top() + IMOUTO_TITLE_Y,
        i18n::nav_label(language, "妹妹"),
        "IMOUTO",
    );
    let pair_gap = PAIR_GAP;
    let pair_width = (panel.width() - INNER_X * 2.0 - pair_gap) / 2.0;
    // 心情的值最长，独占一整行，其余四项保持两列网格。
    if let Some(node) = find_node(nav, "status.imouto.interest") {
        let rect = Rect::from_min_size(
            egui::pos2(panel.left() + INNER_X, panel.top() + IMOUTO_FIRST_Y),
            Vec2::new(panel.width() - INNER_X * 2.0, 24.0),
        );
        if let Some(action) = draw_pair(ui, rect, node, values, state, &mut select_anchor, language)
        {
            return Some(action);
        }
    }
    let imouto_paths = [
        "status.imouto.trust",
        "status.imouto.mood",
        "status.imouto.lust",
        "status.imouto.condition",
    ];
    for (index, path) in imouto_paths.iter().enumerate() {
        let Some(node) = find_node(nav, path) else {
            continue;
        };
        let column = index % 2;
        let line = index / 2;
        let rect = Rect::from_min_size(
            egui::pos2(
                panel.left() + INNER_X + column as f32 * (pair_width + pair_gap),
                panel.top() + IMOUTO_FIRST_Y + (line + 1) as f32 * ROW_STEP,
            ),
            Vec2::new(pair_width, 24.0),
        );
        if let Some(action) = draw_pair(ui, rect, node, values, state, &mut select_anchor, language)
        {
            return Some(action);
        }
    }

    draw_section_title(
        ui,
        panel,
        panel.top() + ONIICHAN_TITLE_Y,
        i18n::nav_label(language, "哥哥"),
        "ONIICHAN",
    );
    let oniichan_paths = [
        "status.oniichan.energy",
        "status.oniichan.lust",
        "status.oniichan.stamina",
        "status.oniichan.money",
    ];
    for (index, path) in oniichan_paths.iter().enumerate() {
        let Some(node) = find_node(nav, path) else {
            continue;
        };
        let column = index % 2;
        let line = index / 2;
        let rect = Rect::from_min_size(
            egui::pos2(
                panel.left() + INNER_X + column as f32 * (pair_width + pair_gap),
                panel.top() + ONIICHAN_FIRST_Y + line as f32 * ROW_STEP,
            ),
            Vec2::new(pair_width, 24.0),
        );
        if let Some(action) = draw_pair(ui, rect, node, values, state, &mut select_anchor, language)
        {
            return Some(action);
        }
    }

    if let Some(anchor) = select_anchor
        && let Some(node) = state
            .open_select
            .as_deref()
            .and_then(|path| find_node(nav, path))
        && matches!(node.kind, NavType::Select)
    {
        return draw_select_panel(ui, node, values, anchor, panels, language);
    }

    None
}

fn draw_pair(
    ui: &mut Ui,
    rect: Rect,
    node: &NavNode,
    values: &HashMap<String, i32>,
    state: &mut MenuState,
    select_anchor: &mut Option<Rect>,
    language: Language,
) -> Option<MenuAction> {
    let value = node_value(language, node, values);
    let is_select = matches!(node.kind, NavType::Select);
    let enabled = !node.read_only;
    let display_value = if is_select {
        format!("{value} ▾")
    } else {
        value
    };
    let response = ui.interact(
        rect,
        ui.scope_id().with(("status-pair", &node.path)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    ui.painter().text(
        egui::pos2(rect.left(), rect.center().y),
        Align2::LEFT_CENTER,
        i18n::nav_label(language, node.caption),
        theme::font(12.0),
        theme::MUTED,
    );
    ui.painter().text(
        egui::pos2(rect.right(), rect.center().y),
        Align2::RIGHT_CENTER,
        &display_value,
        theme::font(14.0),
        if enabled { theme::INK } else { theme::MUTED },
    );
    if response.hovered() && enabled {
        theme::screentone(ui.painter(), rect, 6.0, 0.7);
    }
    if state.open_select.as_deref() == Some(node.path.as_str()) {
        *select_anchor = Some(rect);
    }
    if response.clicked() && enabled {
        if is_select {
            state.open_select = Some(node.path.clone());
            state.open_submenu = None;
            *select_anchor = Some(rect);
        } else {
            return Some(MenuAction::Prompt {
                path: node.path.clone(),
                caption: i18n::nav_label(language, node.caption),
            });
        }
    }
    None
}
