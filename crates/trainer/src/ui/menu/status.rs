//! 状态面板：时间、方针、妹妹和哥哥的数值展示与编辑。

use eframe::egui::{self, Align2, Rect, Sense, Ui, Vec2};

use crate::config::pointer::{FieldId, IMOUTO_GRID_FIELDS, MenuValues, ONIICHAN_GRID_FIELDS};
use crate::i18n::{self, Language};
use crate::ui::theme;

use super::widgets::{
    dotted_underline, draw_inline_value, draw_section_title, field_value, option_row,
    panel_shell_plain, popup_panel,
};
use super::{
    MENU_ROW_HEIGHT, MENU_WIDTH, MENU_X, MENU_Y, MenuAction, MenuState, OPTION_GAP, OPTION_INSET,
    STATUS_PANEL_HEIGHT, SelectTarget,
};

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
const PAIR_WIDTH: f32 = (MENU_WIDTH - INNER_X * 2.0 - PAIR_GAP) / 2.0;

pub(super) fn draw_status(
    ui: &mut Ui,
    values: &MenuValues,
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

    let day_value = values.get(FieldId::TimeDay);
    let hour_value = values.get(FieldId::TimeHour).clamp(0, 23);
    let minute_value = values.get(FieldId::TimeMinute).clamp(0, 59);
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
    if let Some(action) = draw_time_trigger(ui, day_rect, "status-day", FieldId::TimeDay, state) {
        return Some(action);
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
    if let Some(action) = draw_time_trigger(ui, hour_rect, "status-hour", FieldId::TimeHour, state)
    {
        return Some(action);
    }

    let minute_rect = Rect::from_min_size(
        egui::pos2(minute_x, label_y - 2.0),
        Vec2::new(minute_width + 4.0, 40.0),
    );
    if let Some(action) =
        draw_time_trigger(ui, minute_rect, "status-minute", FieldId::TimeMinute, state)
    {
        return Some(action);
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
        i18n::field_label(language, FieldId::HouseholdPolicy),
        "SELECT",
    );
    let policy_row = Rect::from_min_size(
        egui::pos2(panel.left() + INNER_X, panel.top() + POLICY_ROW_Y),
        Vec2::new(panel.width() - INNER_X * 2.0, 24.0),
    );
    let policy_value = field_value(language, FieldId::HouseholdPolicy, values);
    draw_inline_value(
        ui,
        policy_row.left(),
        policy_row.center().y,
        text.current,
        &format!("{policy_value} ▾"),
        theme::INK,
    );
    let response = ui
        .interact(
            policy_row,
            ui.scope_id().with("status-policy"),
            Sense::click(),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    if response.hovered() {
        theme::screentone(ui.painter(), policy_row, 6.0, 0.7);
    }
    let policy_target = SelectTarget::Field(FieldId::HouseholdPolicy);
    if response.clicked() {
        if state.open_select == Some(policy_target) {
            state.open_select = None;
        } else {
            state.open_select = Some(policy_target);
            state.open_submenu = None;
        }
    }
    if state.open_select == Some(policy_target) {
        select_anchor = Some(policy_row);
    }

    draw_section_title(
        ui,
        panel,
        panel.top() + IMOUTO_TITLE_Y,
        text.imouto,
        "IMOUTO",
    );
    // 心情的值最长，独占一整行，其余四项保持两列网格。
    let interest_rect = Rect::from_min_size(
        egui::pos2(panel.left() + INNER_X, panel.top() + IMOUTO_FIRST_Y),
        Vec2::new(panel.width() - INNER_X * 2.0, 24.0),
    );
    if let Some(action) = draw_pair(
        ui,
        interest_rect,
        FieldId::ImoutoInterest,
        values,
        state,
        &mut select_anchor,
        language,
    ) {
        return Some(action);
    }
    if let Some(action) = draw_pair_grid(
        ui,
        panel.top() + IMOUTO_FIRST_Y + ROW_STEP,
        &IMOUTO_GRID_FIELDS,
        values,
        state,
        &mut select_anchor,
        language,
    ) {
        return Some(action);
    }

    draw_section_title(
        ui,
        panel,
        panel.top() + ONIICHAN_TITLE_Y,
        text.oniichan,
        "ONIICHAN",
    );
    if let Some(action) = draw_pair_grid(
        ui,
        panel.top() + ONIICHAN_FIRST_Y,
        &ONIICHAN_GRID_FIELDS,
        values,
        state,
        &mut select_anchor,
        language,
    ) {
        return Some(action);
    }

    if let (Some(anchor), Some(SelectTarget::Field(field))) = (select_anchor, state.open_select) {
        return draw_select_panel(ui, field, values, anchor, panels, language);
    }

    None
}

fn draw_time_trigger(
    ui: &mut Ui,
    rect: Rect,
    id: &'static str,
    field: FieldId,
    state: &mut MenuState,
) -> Option<MenuAction> {
    let response = ui
        .interact(rect, ui.scope_id().with(id), Sense::click())
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    if response.hovered() {
        theme::screentone(ui.painter(), rect, 6.0, 0.7);
    }
    if response.clicked() && !field.is_read_only() {
        state.open_select = None;
        return Some(MenuAction::Prompt(field));
    }
    None
}

fn draw_pair_grid(
    ui: &mut Ui,
    first_y: f32,
    fields: &[FieldId; 4],
    values: &MenuValues,
    state: &mut MenuState,
    select_anchor: &mut Option<Rect>,
    language: Language,
) -> Option<MenuAction> {
    for (index, &field) in fields.iter().enumerate() {
        let column = index % 2;
        let line = index / 2;
        let rect = Rect::from_min_size(
            egui::pos2(
                MENU_X + INNER_X + column as f32 * (PAIR_WIDTH + PAIR_GAP),
                first_y + line as f32 * ROW_STEP,
            ),
            Vec2::new(PAIR_WIDTH, 24.0),
        );
        if let Some(action) = draw_pair(ui, rect, field, values, state, select_anchor, language) {
            return Some(action);
        }
    }
    None
}

fn draw_pair(
    ui: &mut Ui,
    rect: Rect,
    field: FieldId,
    values: &MenuValues,
    state: &mut MenuState,
    select_anchor: &mut Option<Rect>,
    language: Language,
) -> Option<MenuAction> {
    let value = field_value(language, field, values);
    let is_select = !field.options().is_empty();
    let enabled = !field.is_read_only();
    let display_value = if is_select {
        format!("{value} ▾")
    } else {
        value
    };
    let response = ui.interact(
        rect,
        ui.scope_id().with(("status-pair", field)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    ui.painter().text(
        egui::pos2(rect.left(), rect.center().y),
        Align2::LEFT_CENTER,
        i18n::field_label(language, field),
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
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        theme::screentone(ui.painter(), rect, 6.0, 0.7);
    }
    let target = SelectTarget::Field(field);
    if response.clicked() && enabled {
        if is_select {
            if state.open_select == Some(target) {
                state.open_select = None;
            } else {
                state.open_select = Some(target);
                state.open_submenu = None;
            }
        } else {
            state.open_select = None;
            return Some(MenuAction::Prompt(field));
        }
    }
    if state.open_select == Some(target) {
        *select_anchor = Some(rect);
    }
    None
}

fn draw_select_panel(
    ui: &mut Ui,
    field: FieldId,
    values: &MenuValues,
    anchor: Rect,
    panels: &mut Vec<Rect>,
    language: Language,
) -> Option<MenuAction> {
    let options = field.options();
    if options.is_empty() {
        return None;
    }
    // 选项超过 10 个时分两列，避免面板超出窗口。
    let columns = if options.len() > 10 { 2 } else { 1 };
    let rows = options.len().div_ceil(columns);
    let panel = popup_panel(ui, panels, anchor, rows);

    let current = values.get(field);
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
        let label = i18n::option_label(language, field, option.value).unwrap_or("");
        let response = option_row(
            ui,
            rect,
            label,
            selected,
            enabled,
            ("select-option", field, index),
        );
        if response.clicked() && enabled {
            return Some(MenuAction::Select {
                field,
                value: option.value,
            });
        }
    }
    None
}
