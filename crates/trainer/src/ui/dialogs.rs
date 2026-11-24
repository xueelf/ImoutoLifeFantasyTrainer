//! 输入、关于、错误和超时弹窗。

use eframe::egui::{
    self, Align2, Color32, Pos2, Rect, Sense, Stroke, StrokeKind, TextureHandle, Ui, Vec2,
};

use crate::config::pointer::FieldId;
use crate::config::{self, VERSION};
use crate::i18n::{self, Language};
use crate::services::win32;
use crate::ui::theme;

pub struct PromptState {
    pub field: FieldId,
    pub value: String,
    pub error: Option<String>,
}

pub enum PromptResult {
    None,
    Submit(i32),
    Cancel,
}

pub fn draw_prompt(
    ui: &mut Ui,
    state: &mut PromptState,
    current: i32,
    language: Language,
) -> PromptResult {
    let text = i18n::strings(language);
    draw_scrim(ui, "prompt-scrim");
    let card = Rect::from_min_size(Pos2::new(100.0, 124.0), Vec2::new(360.0, 190.0));
    draw_card(ui, card);
    let caption = i18n::field_label(language, state.field);
    let close = draw_title(
        ui,
        card,
        &text.edit_title.replace("{}", caption),
        "prompt-close",
    );
    let hint = if let Some(error) = &state.error {
        error.as_str()
    } else {
        text.integer_hint
    };
    ui.painter().text(
        Pos2::new(card.left() + 18.0, card.top() + 48.0),
        Align2::LEFT_TOP,
        text.new_value,
        theme::font(12.0),
        theme::INK,
    );

    let field = Rect::from_min_size(
        Pos2::new(card.left() + 18.0, card.top() + 66.0),
        Vec2::new(card.width() - 36.0, 36.0),
    );
    ui.painter().rect(
        field,
        egui::CornerRadius::from(4.0),
        Color32::WHITE,
        Stroke::new(1.5, theme::INK),
        StrokeKind::Inside,
    );
    let response = ui.put(
        field.shrink2(Vec2::new(6.0, 2.0)),
        egui::TextEdit::singleline(&mut state.value)
            .font(theme::mono(16.0))
            .text_color(theme::INK)
            .frame(egui::Frame::NONE)
            .align(Align2::LEFT_CENTER)
            .desired_width(f32::INFINITY),
    );
    // 每次打开输入框都抢焦点，Enter 提交、Esc 取消。
    if !response.has_focus() {
        response.request_focus();
    }

    ui.painter().text(
        Pos2::new(card.left() + 18.0, card.top() + 112.0),
        Align2::LEFT_TOP,
        text.current_value.replace("{}", &current.to_string()),
        theme::font(10.0),
        theme::MUTED,
    );
    ui.painter().text(
        Pos2::new(card.right() - 18.0, card.top() + 112.0),
        Align2::RIGHT_TOP,
        hint,
        theme::font(10.0),
        if state.error.is_some() {
            theme::INK
        } else {
            theme::MUTED
        },
    );

    let cancel = theme::button(
        ui,
        Rect::from_min_size(
            Pos2::new(card.right() - 182.0, card.top() + 142.0),
            Vec2::new(76.0, 30.0),
        ),
        text.cancel,
        false,
        ui.scope_id().with("prompt-cancel"),
    );
    let submit = theme::button(
        ui,
        Rect::from_min_size(
            Pos2::new(card.right() - 94.0, card.top() + 142.0),
            Vec2::new(76.0, 30.0),
        ),
        text.ok,
        true,
        ui.scope_id().with("prompt-submit"),
    );

    let enter = ui.input(|input| input.key_pressed(egui::Key::Enter));
    let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
    if cancel.clicked() || escape || close {
        return PromptResult::Cancel;
    }
    if submit.clicked() || enter {
        match state.value.trim().parse::<i32>() {
            Ok(value) => return PromptResult::Submit(value),
            Err(_) => state.error = Some(text.integer_error.to_string()),
        }
    }

    PromptResult::None
}

pub fn draw_about(ui: &mut Ui, icon: Option<&TextureHandle>, language: Language) -> bool {
    let text = i18n::strings(language);
    draw_scrim(ui, "about-scrim");
    let card = Rect::from_min_size(Pos2::new(70.0, 40.0), Vec2::new(420.0, 360.0));
    draw_card(ui, card);
    let close = draw_title(ui, card, text.about, "about-close");

    let icon_rect = Rect::from_min_size(
        Pos2::new(card.left() + 18.0, card.top() + 48.0),
        Vec2::splat(36.0),
    );
    if let Some(icon) = icon {
        ui.put(
            icon_rect,
            egui::Image::new(icon).fit_to_exact_size(Vec2::splat(36.0)),
        );
    } else {
        ui.painter().rect(
            icon_rect,
            egui::CornerRadius::from(4.0),
            Color32::WHITE,
            Stroke::new(1.0, theme::INK),
            StrokeKind::Inside,
        );
    }
    ui.painter().text(
        Pos2::new(icon_rect.right() + 10.0, icon_rect.top() + 2.0),
        Align2::LEFT_TOP,
        format!("{} {VERSION}", text.title),
        theme::font(14.0),
        theme::INK,
    );
    ui.painter().text(
        Pos2::new(icon_rect.right() + 12.0, icon_rect.top() + 20.0),
        Align2::LEFT_TOP,
        format!("{} · {}", text.about_powered, text.license),
        theme::font(10.0),
        theme::MUTED,
    );

    ui.painter().text(
        Pos2::new(card.left() + 18.0, card.top() + 96.0),
        Align2::LEFT_TOP,
        text.tech_stack,
        theme::font(10.0),
        theme::MUTED,
    );

    let list_rect = Rect::from_min_max(
        Pos2::new(card.left() + 18.0, card.top() + 112.0),
        Pos2::new(card.right() - 18.0, card.top() + 280.0),
    );
    ui.painter().rect_stroke(
        list_rect,
        egui::CornerRadius::from(0.0),
        Stroke::new(1.0, theme::LINE),
        StrokeKind::Inside,
    );
    let list_inner = list_rect.shrink(6.0);
    // 技术栈列表可滚动，点击任意一行用系统浏览器打开项目主页。
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(list_inner)
            .layout(egui::Layout::top_down(egui::Align::LEFT)),
        |ui| {
            egui::ScrollArea::vertical()
                .id_salt("about-tech-scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for item in config::TECH_STACK {
                        let (rect, response) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), 26.0),
                            Sense::click(),
                        );
                        if response.hovered() {
                            ui.painter().rect_filled(rect, 0.0, theme::SCREEN_TONE);
                        }
                        let font = theme::font(10.0);
                        ui.painter().text(
                            Pos2::new(rect.left() + 2.0, rect.top() + 2.0),
                            Align2::LEFT_TOP,
                            item.name,
                            font.clone(),
                            theme::INK,
                        );
                        ui.painter().text(
                            Pos2::new(rect.right() - 2.0, rect.top() + 2.0),
                            Align2::RIGHT_TOP,
                            item.license,
                            font.clone(),
                            theme::MUTED,
                        );
                        ui.painter().text(
                            Pos2::new(rect.left() + 2.0, rect.top() + 14.0),
                            Align2::LEFT_TOP,
                            item.copyright,
                            font.clone(),
                            theme::MUTED,
                        );
                        if response.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if response.clicked() {
                            win32::open_url(item.url);
                        }
                    }
                });
        },
    );

    ui.painter().text(
        Pos2::new(card.left() + 18.0, card.top() + 292.0),
        Align2::LEFT_TOP,
        text.copyright,
        theme::font(10.0),
        theme::MUTED,
    );

    let button = theme::button(
        ui,
        Rect::from_min_size(
            Pos2::new(card.right() - 94.0, card.top() + 312.0),
            Vec2::new(76.0, 30.0),
        ),
        text.close,
        true,
        ui.scope_id().with("about-confirm"),
    );
    let dismiss = ui
        .input(|input| input.key_pressed(egui::Key::Escape) || input.key_pressed(egui::Key::Enter));
    close || button.clicked() || dismiss
}

pub fn draw_error(ui: &mut Ui, message: &str, language: Language) -> bool {
    let text = i18n::strings(language);
    draw_notice_shell(
        ui,
        language,
        "error",
        text.error_prefix,
        |ui, icon_center| {
            ui.painter().text(
                icon_center,
                Align2::CENTER_CENTER,
                "!",
                theme::mono(16.0),
                theme::INK,
            );
        },
        |ui, card| {
            ui.put(
                Rect::from_min_max(
                    Pos2::new(card.left() + 62.0, card.top() + 74.0),
                    Pos2::new(card.right() - 20.0, card.top() + 116.0),
                ),
                egui::Label::new(
                    egui::RichText::new(message)
                        .font(theme::font(10.0))
                        .color(theme::MUTED),
                )
                .wrap(),
            );
        },
    )
}

pub fn draw_timeout(ui: &mut Ui, language: Language) -> bool {
    let text = i18n::strings(language);
    draw_notice_shell(
        ui,
        language,
        "timeout",
        text.timeout_message,
        |ui, icon_center| {
            ui.painter().line_segment(
                [icon_center, icon_center + Vec2::new(0.0, -12.0)],
                Stroke::new(2.0, theme::INK),
            );
            ui.painter().line_segment(
                [icon_center, icon_center + Vec2::new(8.0, 4.0)],
                Stroke::new(2.0, theme::INK),
            );
        },
        |ui, card| {
            ui.painter().text(
                Pos2::new(card.left() + 62.0, card.top() + 80.0),
                Align2::LEFT_TOP,
                text.timeout_hint,
                theme::font(10.0),
                theme::MUTED,
            );
        },
    )
}

fn draw_notice_shell(
    ui: &mut Ui,
    language: Language,
    id_prefix: &str,
    heading: &str,
    draw_icon: impl FnOnce(&mut Ui, Pos2),
    draw_body: impl FnOnce(&mut Ui, Rect),
) -> bool {
    let text = i18n::strings(language);
    draw_scrim(ui, &format!("{id_prefix}-scrim"));
    let card = Rect::from_min_size(Pos2::new(100.0, 124.0), Vec2::new(360.0, 190.0));
    draw_card(ui, card);
    let close = draw_title(ui, card, text.notice, &format!("{id_prefix}-close"));

    let icon_center = Pos2::new(card.left() + 34.0, card.top() + 80.0);
    ui.painter()
        .circle_stroke(icon_center, 18.0, Stroke::new(2.0, theme::INK));
    draw_icon(ui, icon_center);

    ui.painter().text(
        Pos2::new(card.left() + 62.0, card.top() + 56.0),
        Align2::LEFT_TOP,
        heading,
        theme::font(12.0),
        theme::INK,
    );
    draw_body(ui, card);

    let button = theme::button(
        ui,
        Rect::from_min_size(
            Pos2::new(card.right() - 94.0, card.top() + 142.0),
            Vec2::new(76.0, 30.0),
        ),
        text.ok,
        true,
        ui.scope_id().with((id_prefix, "confirm")),
    );
    let dismiss = ui
        .input(|input| input.key_pressed(egui::Key::Escape) || input.key_pressed(egui::Key::Enter));
    close || button.clicked() || dismiss
}

fn draw_scrim(ui: &mut Ui, id: &str) {
    let full = ui.max_rect();
    ui.painter().rect_filled(
        full,
        0.0,
        Color32::from_rgba_unmultiplied(253, 252, 248, 184),
    );
    ui.interact(full, ui.scope_id().with(id), Sense::click());
}

fn draw_card(ui: &mut Ui, rect: Rect) {
    ui.painter().rect_filled(
        rect.translate(Vec2::new(4.0, 4.0)),
        egui::CornerRadius::from(12.0),
        Color32::from_rgba_unmultiplied(27, 27, 27, 35),
    );
    ui.painter().rect(
        rect,
        egui::CornerRadius::from(12.0),
        theme::MENU_PAPER,
        Stroke::new(2.0, theme::INK),
        StrokeKind::Inside,
    );
    theme::tape(
        ui.painter(),
        Pos2::new(rect.center().x, rect.top()),
        Vec2::new(58.0, 16.0),
        -2.0_f32.to_radians(),
    );
}

fn draw_title(ui: &mut Ui, card: Rect, title: &str, id: &str) -> bool {
    ui.painter().text(
        Pos2::new(card.left() + 18.0, card.top() + 14.0),
        Align2::LEFT_TOP,
        title,
        theme::font(18.0),
        theme::INK,
    );
    let close_rect = Rect::from_min_size(
        Pos2::new(card.right() - 34.0, card.top() + 10.0),
        Vec2::splat(18.0),
    );
    let response = ui.interact(close_rect, ui.scope_id().with(id), Sense::click());
    ui.painter().text(
        close_rect.center(),
        Align2::CENTER_CENTER,
        "×",
        theme::mono(12.0),
        theme::MUTED,
    );
    if response.hovered() {
        ui.painter()
            .circle_stroke(close_rect.center(), 10.0, Stroke::new(1.0, theme::LINE));
    }
    response.clicked()
}
