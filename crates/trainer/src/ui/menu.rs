//! 顶部菜单、二级面板和菜单交互状态。

use eframe::egui::{self, Align2, Color32, Rect, Sense, Stroke, Ui, Vec2};

use crate::config::pointer::{FieldId, ItemGroup, MenuValues, NavAction};
use crate::config::{BODY_HEIGHT, HEADER_HEIGHT, MENU_HEIGHT, WINDOW_WIDTH};
use crate::i18n::{self, Language};
use crate::ui::theme;

const MENU_X: f32 = 24.0;
const MENU_Y: f32 = 108.0;
// 一级与二级菜单共用同一宽度，宽度只由内容决定，不随窗口拉伸。
const MENU_WIDTH: f32 = 208.0;
const MENU_GAP: f32 = 12.0;
const SIDE_X: f32 = MENU_X + MENU_WIDTH + MENU_GAP;
const MASK_INSET: f32 = 10.0;

const BODY_TOP: f32 = HEADER_HEIGHT + MENU_HEIGHT;
const BODY_BOTTOM: f32 = BODY_TOP + BODY_HEIGHT;
const STATUS_PANEL_HEIGHT: f32 = BODY_BOTTOM - MENU_Y;

// 菜单不允许上下留白：面板高度完全由行数决定，所有菜单行等高。
const MENU_ROW_HEIGHT: f32 = 32.0;

const ROW_INSET: f32 = 4.0;
const ROW_TEXT_INSET: f32 = 6.0;
const LABEL_VALUE_GAP: f32 = 10.0;

// 下拉菜单统一排布：左右各 8，选项文字 12，列间距 12。
const OPTION_INSET: f32 = 8.0;
const OPTION_GAP: f32 = 12.0;
const OPTION_RADIO_X: f32 = 6.0;
const OPTION_LABEL_X: f32 = 20.0;
const OPTION_FONT_SIZE: f32 = 12.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuTab {
    Items,
    Status,
    Help,
}

impl MenuTab {
    const fn index(self) -> usize {
        match self {
            Self::Items => 0,
            Self::Status => 1,
            Self::Help => 2,
        }
    }

    const fn label(self, language: Language) -> &'static str {
        let text = i18n::strings(language);
        match self {
            Self::Items => text.tab_items,
            Self::Status => text.tab_status,
            Self::Help => text.tab_help,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectTarget {
    Language,
    Field(FieldId),
}

pub struct MenuState {
    pub open: Option<MenuTab>,
    pub open_submenu: Option<ItemGroup>,
    pub open_select: Option<SelectTarget>,
    pub tab_rects: [Rect; 3],
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            open: None,
            open_submenu: None,
            open_select: None,
            tab_rects: [Rect::NOTHING; 3],
        }
    }
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    Language(Language),
    Prompt(FieldId),
    Check { field: FieldId, checked: bool },
    Select { field: FieldId, value: i32 },
    Command(NavAction),
}

mod help;
mod items;
mod status;
mod widgets;

pub fn draw_tabs(
    ui: &mut Ui,
    state: &mut MenuState,
    attached: bool,
    interactive: bool,
    language: Language,
) {
    let labels = [MenuTab::Items, MenuTab::Status, MenuTab::Help];
    let font = theme::font(if language == Language::Ja { 12.0 } else { 14.0 });
    let widths = labels.map(|tab| theme::measure(ui, tab.label(language), font.clone()).x);
    let text_total: f32 = widths.iter().sum();
    // 三个标签整体收进菜单宽度内，气泡箭头始终能对准当前标签。
    let unit = 24.0;
    let slot_units = (labels.len() * 2 - 1) as f32;
    let scale = ((MENU_WIDTH - text_total) / (unit * slot_units)).clamp(0.0, 1.0);
    let pad = unit * scale;
    let mut x = MENU_X;
    let y = HEADER_HEIGHT;

    for (index, tab) in labels.into_iter().enumerate() {
        let label = tab.label(language);
        let width = widths[index] + pad;
        let rect = Rect::from_min_size(egui::pos2(x, y), Vec2::new(width, MENU_ROW_HEIGHT));
        state.tab_rects[index] = rect;
        let enabled = attached || tab == MenuTab::Help;
        let can_interact = enabled && interactive;
        let response = ui.interact(
            rect,
            ui.scope_id().with(("menu-tab", index)),
            if can_interact {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        let hovered = response.hovered() && can_interact;
        if hovered {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        let active = state.open == Some(tab);
        let color = if !enabled {
            theme::DISABLED
        } else if active || hovered {
            theme::INK
        } else {
            theme::MUTED
        };

        if hovered && !active {
            ui.painter().line_segment(
                [rect.left_bottom(), rect.right_bottom()],
                Stroke::new(1.0, theme::LINE),
            );
        }
        if active {
            theme::wobbly_oval(
                ui.painter(),
                rect,
                Stroke::new(1.5, theme::INK),
                index as f32 * 1.7,
            );
        }
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            label,
            font.clone(),
            color,
        );

        if response.clicked() && can_interact {
            if state.open == Some(tab) {
                state.open = None;
            } else {
                state.open = Some(tab);
            }
            state.open_submenu = None;
            state.open_select = None;
        }
        x = rect.right() + pad;
    }
}

pub fn draw_panels(
    ui: &mut Ui,
    values: &MenuValues,
    state: &mut MenuState,
    language: Language,
) -> Option<MenuAction> {
    let open = state.open?;
    let body = Rect::from_min_max(
        egui::pos2(MASK_INSET, BODY_TOP),
        egui::pos2(WINDOW_WIDTH - MASK_INSET, BODY_BOTTOM),
    );
    ui.painter().rect_filled(
        body,
        0.0,
        Color32::from_rgba_unmultiplied(253, 252, 248, 220),
    );

    let mut panels = Vec::new();
    let action = match open {
        MenuTab::Items => items::draw_items(ui, values, state, &mut panels, language),
        MenuTab::Status => status::draw_status(ui, values, state, &mut panels, language),
        MenuTab::Help => help::draw_help(ui, state, &mut panels, language),
    };

    if let Some(panel) = panels.first() {
        let tab_rect = state.tab_rects[open.index()];
        if tab_rect != Rect::NOTHING {
            let cx = tab_rect
                .center()
                .x
                .clamp(panel.left() + 16.0, panel.right() - 16.0);
            // 顶边切口要盖住 2px 描边。奇数偏移可避免折线落到描边下方。
            let border_y = panel.top() + 1.0;
            let patch = 14.0;
            let base_half = 6.0;
            let apex_y = panel.top() - 6.0;
            // 先盖住顶边，再用一条折线同时描出顶边和指针。
            ui.painter().rect_filled(
                Rect::from_min_max(
                    egui::pos2(cx - patch, panel.top() - 0.5),
                    egui::pos2(cx + patch, panel.top() + 3.0),
                ),
                0.0,
                theme::MENU_PAPER,
            );
            ui.painter().add(egui::Shape::convex_polygon(
                vec![
                    egui::pos2(cx - base_half, panel.top() + 3.0),
                    egui::pos2(cx - base_half, border_y),
                    egui::pos2(cx, apex_y),
                    egui::pos2(cx + base_half, border_y),
                    egui::pos2(cx + base_half, panel.top() + 3.0),
                ],
                theme::MENU_PAPER,
                Stroke::NONE,
            ));
            ui.painter().add(egui::Shape::line(
                vec![
                    egui::pos2(cx - patch, border_y),
                    egui::pos2(cx - base_half, border_y),
                    egui::pos2(cx, apex_y),
                    egui::pos2(cx + base_half, border_y),
                    egui::pos2(cx + patch, border_y),
                ],
                Stroke::new(2.0, theme::INK),
            ));
        }
    }

    if let Some(action) = action {
        match action {
            MenuAction::Select { .. } | MenuAction::Language(_) => {
                // 选完下拉项只收起下拉，状态面板继续留着。
                state.open_select = None;
            }
            MenuAction::Command(_) => {
                state.open = None;
                state.open_submenu = None;
                state.open_select = None;
            }
            // 输入/勾选后保持当前菜单和子菜单，方便连续修改。
            MenuAction::Prompt(_) | MenuAction::Check { .. } => {}
        }
        return Some(action);
    }

    let click = ui.input(|input| {
        if input.pointer.any_click() {
            input.pointer.interact_pos()
        } else {
            None
        }
    });
    if let Some(click) = click {
        // 点击落在面板和标签之外时关闭菜单。
        let inside = panels.iter().any(|panel| panel.contains(click))
            || state.tab_rects.iter().any(|rect| rect.contains(click));
        if !inside {
            state.open = None;
            state.open_submenu = None;
            state.open_select = None;
        }
    }

    None
}
