//! 未连接、等待和多开状态的内容区。

use eframe::egui::{Color32, Pos2, Rect, TextureHandle, Ui};

use crate::i18n::{self, Language};
use crate::ui::theme;

const ART_CENTER_Y: f32 = 86.0;
const ART_SIZE: f32 = 120.0;
const ART_MAX_SIZE: f32 = 96.0;
const ART_GAP: f32 = 16.0;
const TITLE_Y: f32 = 166.0;
const HINT_Y: f32 = 210.0;
const STATE_TITLE_SIZE: f32 = 20.0;
const MULTI_TITLE_SIZE: f32 = 14.0;
const HINT_SIZE: f32 = 12.0;

#[derive(Clone, Copy)]
pub enum ConnectionState {
    Disconnected,
    Waiting,
    Multiple(usize),
}

pub fn draw(
    ui: &mut Ui,
    rect: Rect,
    kind: ConnectionState,
    multi_images: &[TextureHandle],
    multi_selection: &[usize],
    radio_image: Option<&TextureHandle>,
    language: Language,
) {
    match kind {
        ConnectionState::Disconnected => draw_radio(ui, rect, false, radio_image, language),
        ConnectionState::Waiting => draw_radio(ui, rect, true, radio_image, language),
        ConnectionState::Multiple(count) => {
            draw_multi(ui, rect, count, multi_images, multi_selection, language)
        }
    }
}

fn draw_radio(
    ui: &mut Ui,
    rect: Rect,
    waiting: bool,
    image: Option<&TextureHandle>,
    language: Language,
) {
    let text = i18n::strings(language);
    draw_art(ui, rect, image);

    let (title, hint) = if waiting {
        (text.waiting_title, text.waiting_hint)
    } else {
        (text.disconnected_title, text.disconnected_hint)
    };
    draw_centered_text(ui, rect, TITLE_Y, title, STATE_TITLE_SIZE, theme::INK);
    draw_centered_text(ui, rect, HINT_Y, hint, HINT_SIZE, theme::MUTED);

    if waiting {
        // 三个点依次闪烁，提示还在读取游戏数据。
        ui.ctx().request_repaint();
        let time = ui.input(|input| input.time);
        for index in 0..3 {
            let alpha = (0.25
                + 0.75 * ((time * 2.2 - index as f64 * 0.5).sin() * 0.5 + 0.5) as f32)
                .clamp(0.2, 1.0);
            ui.painter().circle_filled(
                Pos2::new(
                    rect.center().x - 12.0 + index as f32 * 12.0,
                    rect.top() + 240.0,
                ),
                4.0,
                theme::INK.gamma_multiply(alpha),
            );
        }
    }
}

fn draw_multi(
    ui: &mut Ui,
    rect: Rect,
    count: usize,
    images: &[TextureHandle],
    selection: &[usize],
    language: Language,
) {
    let text = i18n::strings(language);
    let selected: Vec<&TextureHandle> = selection
        .iter()
        .filter_map(|index| images.get(*index))
        .collect();
    draw_multi_art(ui, rect, &selected);

    let count_text = if language == Language::Zh {
        chinese_count(count)
    } else {
        count.to_string()
    };
    let message = text.multi_message.replace("{}", &count_text);
    draw_centered_text(ui, rect, TITLE_Y, &message, MULTI_TITLE_SIZE, theme::INK);
    draw_centered_text(ui, rect, HINT_Y, text.multi_hint, HINT_SIZE, theme::MUTED);
}

fn draw_art(ui: &Ui, rect: Rect, texture: Option<&TextureHandle>) {
    let Some(texture) = texture else {
        return;
    };
    let center = Pos2::new(rect.center().x, rect.top() + ART_CENTER_Y);
    draw_texture_fit(ui, texture, center, ART_SIZE);
}

fn draw_multi_art(ui: &Ui, rect: Rect, images: &[&TextureHandle]) {
    let count = images.len();
    if count == 0 {
        return;
    }
    // 按窗口宽度均分单元格，最多 96px，并保持偶数宽度。
    let total_gap = ART_GAP * (count.saturating_sub(1)) as f32;
    let cell = ((rect.width() - total_gap) / count as f32).clamp(2.0, ART_MAX_SIZE);
    let cell = (cell / 2.0).floor() * 2.0;
    let total_width = cell * count as f32 + total_gap;
    let start_x = rect.center().x - total_width * 0.5;
    let center_y = rect.top() + ART_CENTER_Y;
    for (index, texture) in images.iter().enumerate() {
        let center = Pos2::new(
            start_x + cell * 0.5 + index as f32 * (cell + ART_GAP),
            center_y,
        );
        draw_texture_fit(ui, texture, center, cell);
    }
}

fn draw_texture_fit(ui: &Ui, texture: &TextureHandle, center: Pos2, max_size: f32) {
    let size = texture.size_vec2();
    if size.x <= 0.0 || size.y <= 0.0 {
        return;
    }
    let scale = (max_size / size.x).min(max_size / size.y);
    theme::rotated_texture(ui.painter(), texture.id(), center, size * scale, 0.0);
}

fn draw_centered_text(ui: &Ui, rect: Rect, y: f32, text: &str, size: f32, color: Color32) {
    let galley = ui
        .painter()
        .layout(text.to_owned(), theme::font(size), color, rect.width());
    let position = Pos2::new(rect.center().x - galley.size().x * 0.5, rect.top() + y);
    ui.painter().galley(position, galley, color);
}

fn chinese_count(count: usize) -> String {
    // 中文多开文案用汉字数字，2 读作「两」。
    if count == 2 {
        return "两".to_string();
    }
    let digits = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
    if count < 10 {
        return digits[count].to_string();
    }
    if count < 20 {
        if count.is_multiple_of(10) {
            return "十".to_string();
        }
        return format!("十{}", digits[count % 10]);
    }
    if count < 100 {
        let tens = digits[count / 10];
        if count.is_multiple_of(10) {
            return format!("{tens}十");
        }
        return format!("{tens}十{}", digits[count % 10]);
    }
    count.to_string()
}
