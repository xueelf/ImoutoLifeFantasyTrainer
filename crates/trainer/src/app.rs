//! 主窗口状态机：轮询游戏进程，维护附加会话、菜单值、弹窗和动画。

use eframe::egui::{
    self, Align2, Color32, FontData, FontDefinitions, FontFamily, FontId, Pos2, Rect, Sense,
    Stroke, TextStyle, TextureHandle, Vec2, ViewportCommand,
};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::config;
use crate::config::feature::{self, FeatureAction};
use crate::config::pointer::{FieldId, MenuValues, NavAction};
use crate::i18n::{self, Language};
use crate::services::assets::AssetService;
use crate::services::hotkeys::HotkeyService;
use crate::services::probe;
use crate::services::session::GameSession;
use crate::services::{sound, win32};
use crate::ui::connection::{self, ConnectionState};
use crate::ui::dialogs::{self, PromptResult, PromptState};
use crate::ui::feature_list::{self, FeatureView};
use crate::ui::menu::{self, MenuAction, MenuState};
use crate::ui::splash::SplashState;
use crate::ui::theme;

const COVER_FLIP_SECS: f32 = 0.32;

const PAGE_MARGIN: f32 = 24.0;
const HEADER_TITLE_POS: Pos2 = Pos2::new(PAGE_MARGIN, 8.0);
const HEADER_RULE_Y: f32 = 42.0;
const HEADER_SUBTITLE_POS: Pos2 = Pos2::new(PAGE_MARGIN, 46.0);
const HEADER_STATUS_DOT_Y: f32 = 54.0;
const CLOSE_X_OFFSET: f32 = 60.0;
const CLOSE_Y: f32 = 4.0;
const CLOSE_SIZE: f32 = 36.0;
const DRAG_WIDTH_OFFSET: f32 = 96.0;

const COVER_CENTER: Pos2 = Pos2::new(126.0, 258.0);
const COVER_ANGLE_DEG: f32 = -2.0;
const COVER_IMAGE_SIZE: Vec2 = Vec2::new(192.0, 256.0);
const COVER_SHADOW_OFFSET: f32 = 4.0;
const COVER_HOVER_OUTLINE: f32 = 8.0;

const TAPE_CENTER: Pos2 = Pos2::new(126.0, 124.0);
const TAPE_SIZE: Vec2 = Vec2::new(92.0, 30.0);
const TAPE_ANGLE_DEG: f32 = -4.0;

const CONTENT_X: f32 = 252.0;
const CONTENT_Y: f32 = 122.0;
const CONTENT_WIDTH: f32 = 284.0;
const CONTENT_HEIGHT: f32 = 272.0;
const CONTENT_HEADER_HEIGHT: f32 = 24.0;

pub struct TrainerApp {
    assets: AssetService,
    hotkeys: Option<HotkeyService>,
    language: Language,
    features: Vec<FeatureView>,
    menu_values: MenuValues,
    menu_state: MenuState,
    session: Option<GameSession>,
    unknown_speed_pids: HashSet<u32>,
    multi_count: usize,
    game_detected: bool,
    game_wait_started: Option<Instant>,
    cover_texture: Option<TextureHandle>,
    cover_name: Option<&'static str>,
    cover_flip: f32,
    icon_texture: Option<TextureHandle>,
    multi_images: Vec<TextureHandle>,
    multi_selection: Vec<usize>,
    intro_sound_played: bool,
    radio_image: Option<TextureHandle>,
    splash: SplashState,
    prompt_dialog: Option<PromptState>,
    about_dialog: bool,
    error_dialog: Option<String>,
    timeout_dialog: bool,
    last_poll: Option<Instant>,
}

impl TrainerApp {
    pub fn new(cc: &eframe::CreationContext<'_>, assets: AssetService, language: Language) -> Self {
        let text = i18n::strings(language);
        install_fonts(&cc.egui_ctx, &assets);
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        cc.egui_ctx.all_styles_mut(|style| {
            style.visuals.window_fill = theme::MENU_PAPER;
            style.visuals.window_stroke = Stroke::new(1.0, theme::INK);
            style.visuals.window_corner_radius = egui::CornerRadius::from(5.0);
            style.visuals.window_shadow = egui::epaint::Shadow::NONE;
            style.visuals.popup_shadow = egui::epaint::Shadow::NONE;
        });

        let features = feature::FEATURES
            .iter()
            .map(|definition| FeatureView {
                definition,
                is_on: false,
            })
            .collect();

        let cover_texture = load_texture(
            &cc.egui_ctx,
            &assets,
            "cover",
            assets.card(config::COVER_PLACEHOLDER),
        );
        let icon_texture = load_texture(&cc.egui_ctx, &assets, "app-icon", assets.icon());
        let multi_images = assets
            .sd_images_excluding("sd_icon01.png")
            .into_iter()
            .enumerate()
            .filter_map(|(index, (name, bytes))| {
                load_texture(
                    &cc.egui_ctx,
                    &assets,
                    &format!("state-multi-{index}-{name}"),
                    Some(bytes),
                )
            })
            .collect();
        let radio_image = load_texture(
            &cc.egui_ctx,
            &assets,
            "state-radio",
            assets.sd("sd_icon01.png"),
        );
        let splash = SplashState::new(&cc.egui_ctx, &assets);

        let (hotkeys, error_dialog) = match HotkeyService::new() {
            Ok(service) => {
                let failed = service
                    .failed()
                    .iter()
                    .map(|hotkey| hotkey.label())
                    .collect::<Vec<_>>();
                let error = if failed.is_empty() {
                    None
                } else {
                    Some(text.hotkeys_busy.replace("{}", &failed.join(", ")))
                };
                (Some(service), error)
            }
            Err(error) => (
                None,
                Some(text.hotkeys_failed.replace("{}", &error.to_string())),
            ),
        };

        Self {
            assets,
            hotkeys,
            language,
            features,
            menu_values: MenuValues::default(),
            menu_state: MenuState::default(),
            session: None,
            unknown_speed_pids: HashSet::new(),
            multi_count: 0,
            game_detected: false,
            game_wait_started: None,
            cover_texture,
            cover_name: Some(config::COVER_PLACEHOLDER),
            cover_flip: 0.0,
            icon_texture,
            multi_images,
            multi_selection: Vec::new(),
            intro_sound_played: false,
            radio_image,
            splash,
            prompt_dialog: None,
            about_dialog: false,
            error_dialog,
            timeout_dialog: false,
            last_poll: None,
        }
    }

    fn text(&self) -> &'static i18n::Strings {
        i18n::strings(self.language)
    }

    fn set_language(&mut self, ctx: &egui::Context, language: Language) {
        if self.language == language {
            return;
        }
        self.language = language;
        ctx.send_viewport_cmd(ViewportCommand::Title(
            i18n::strings(language).title.to_owned(),
        ));
        self.play_sound("Enter");
    }

    fn poll(&mut self, ctx: &egui::Context) {
        if self
            .session
            .as_ref()
            .is_some_and(|session| !session.is_process_alive())
        {
            self.game_detected = false;
            self.detach(ctx);
        }
        self.handle_freeze_error();
        // 进程扫描每秒最多一次，避免频繁创建 Toolhelp 快照。
        if self
            .last_poll
            .is_some_and(|last| last.elapsed() < Duration::from_secs(1))
        {
            return;
        }
        self.last_poll = Some(Instant::now());

        let games = match probe::find_games() {
            Ok(games) => games,
            Err(error) => {
                self.splash.stop();
                self.set_error(self.text().scan_error.replace("{}", &error.to_string()));
                return;
            }
        };

        self.unknown_speed_pids
            .retain(|pid| games.iter().any(|game| game.process.pid == *pid));
        self.game_detected = !games.is_empty();
        if self.game_detected {
            self.timeout_dialog = false;
        } else {
            if self.game_wait_started.is_some() {
                self.splash.stop();
            }
            self.game_wait_started = None;
        }

        let count = games.len();
        if count >= 2 {
            // 多开时先断开当前会话，等只剩一个进程再重新附加。
            self.detach(ctx);
            self.game_wait_started = None;
            self.splash.stop();
            self.close_menu();
            if self.multi_count != count {
                let entering = self.multi_count == 0;
                self.multi_count = count;
                self.multi_selection = select_multi_images(self.multi_images.len(), count);
                if entering {
                    self.play_intro_sound();
                }
            }
            return;
        }

        self.multi_count = 0;
        self.multi_selection.clear();
        let Some(game) = games.into_iter().next() else {
            self.detach(ctx);
            return;
        };
        let Some(values) = game.values else {
            // 给游戏一点时间初始化字段。等太久就收起动画，显示读取提示。
            if self.game_wait_started.is_none() {
                self.game_wait_started = Some(Instant::now());
            }
            if self
                .game_wait_started
                .is_some_and(|start| start.elapsed() >= Duration::from_secs(5))
            {
                self.splash.stop();
            }
            self.detach(ctx);
            return;
        };

        if self
            .session
            .as_ref()
            .is_some_and(|session| session.pid() == game.process.pid)
        {
            // 游戏没换进程时直接采用本次扫描读到的菜单值快照。
            self.game_wait_started = None;
            self.menu_values = values;
            return;
        }

        // 新进程：先清理旧会话，再按 PID 恢复未知的变速状态。
        self.detach(ctx);
        let speed_unknown = self.unknown_speed_pids.contains(&game.process.pid);
        self.session = Some(GameSession::new(game.process, speed_unknown));
        self.menu_values = values;
        self.game_wait_started = None;
        self.splash.stop();
        self.close_menu();
        self.prompt_dialog = None;
        for feature in &mut self.features {
            feature.is_on = false;
        }
        self.load_random_cover(ctx);
        self.play_intro_sound();
    }

    fn detach(&mut self, ctx: &egui::Context) {
        let Some(mut session) = self.session.take() else {
            return;
        };
        // 游戏已经退出时不要再弹变速恢复错误，否则关闭游戏会留下一个无用弹窗。
        let report_speed_error = self.game_detected && session.is_process_alive();
        let pid = session.pid();
        let speed_error = session.shutdown().err();
        if session.speedhack_unknown() {
            self.unknown_speed_pids.insert(pid);
        }

        for feature in &mut self.features {
            feature.is_on = false;
        }
        self.menu_values.clear();
        self.close_menu();
        self.prompt_dialog = None;
        self.set_cover_placeholder(ctx);

        if report_speed_error && let Some(error) = speed_error {
            let message = self
                .text()
                .speed_restore_failed
                .replace("{}", &error.to_string());
            self.set_error(message);
        }
    }

    fn handle_freeze_error(&mut self) {
        let Some(error) = self
            .session
            .as_ref()
            .and_then(|session| session.take_freeze_error())
        else {
            return;
        };
        if let Some(session) = &self.session {
            session.unlock_all();
        }
        let message = self.text().write_error.replace("{}", &error);
        self.set_error(message);
        // 冻结线程只会上报第一条写入错误。变速由 speedhack 自己管理状态，不在这里关闭。
        for feature in &mut self.features {
            if feature.definition.action != FeatureAction::TripleSpeed {
                feature.is_on = false;
            }
        }
    }

    fn poll_hotkeys(&mut self) {
        let actions = match &self.hotkeys {
            Some(service) => service.poll(),
            None => return,
        };
        // 没有会话或弹窗打开时忽略热键，避免误触后台功能。
        if self.session.is_none() || self.dialog_open() {
            return;
        }
        for action in actions {
            self.toggle_feature(action);
        }
    }

    fn toggle_feature(&mut self, action: FeatureAction) {
        let Some(index) = self
            .features
            .iter()
            .position(|feature| feature.definition.action == action)
        else {
            return;
        };
        let was_on = self.features[index].is_on;
        let (result, pid, state_unknown) = {
            let Some(session) = self.session.as_mut() else {
                return;
            };
            let result = if was_on {
                session.disable_feature(action)
            } else {
                session.enable_feature(action)
            };
            (result, session.pid(), session.speedhack_unknown())
        };

        if action == FeatureAction::TripleSpeed && state_unknown {
            // 变速请求超时后状态可能未知，记下 PID，下次附加时继续提示恢复。
            self.unknown_speed_pids.insert(pid);
        }

        match result {
            Ok(()) => {
                self.features[index].is_on = !was_on;
                self.play_sound(if was_on { "Cancel" } else { "Enter" });
            }
            Err(error) => {
                self.set_error(error.to_string());
            }
        }
    }

    fn apply_menu_action(&mut self, ctx: &egui::Context, action: MenuAction) {
        match action {
            MenuAction::Language(language) => self.set_language(ctx, language),
            MenuAction::Prompt(field) => {
                let value = self.menu_values.get(field);
                self.prompt_dialog = Some(PromptState {
                    field,
                    value: value.to_string(),
                    error: None,
                });
            }
            MenuAction::Check { field, checked } => {
                self.write_menu_value(
                    field,
                    i32::from(checked),
                    if checked { "Enter" } else { "Cancel" },
                );
            }
            MenuAction::Select { field, value } => {
                self.write_menu_value(field, value, "Enter");
            }
            MenuAction::Command(command) => {
                self.close_menu();
                self.play_sound("Enter");
                match command {
                    NavAction::Documentation => win32::open_url(config::DOCUMENTATION_URL),
                    NavAction::Repository => win32::open_url(config::REPOSITORY_URL),
                    NavAction::About => self.about_dialog = true,
                }
            }
        }
    }

    fn write_menu_value(&mut self, field: FieldId, value: i32, sound_name: &str) {
        let (min, max) = field.bounds();
        if !(min..=max).contains(&value) {
            let message = self
                .text()
                .value_out_of_range
                .replacen("{}", &min.to_string(), 1)
                .replacen("{}", &max.to_string(), 1);
            self.set_error(message);
            return;
        }
        let Some(session) = &self.session else {
            return;
        };
        // 游戏里星期由日期决定，单独改周会立刻对不上。这里在写日期时一并算好星期。
        let week = (field == FieldId::TimeDay).then(|| (value.max(1) - 1).rem_euclid(7));
        let mut result = session.write_field(field, value);
        if result.is_ok()
            && let Some(week) = week
        {
            result = session.write_field(FieldId::TimeWeek, week);
        }
        match result {
            Ok(()) => {
                self.menu_values.set(field, value);
                if let Some(week) = week {
                    self.menu_values.set(FieldId::TimeWeek, week);
                }
                self.play_sound(sound_name);
            }
            Err(error) => {
                self.set_error(self.text().write_error.replace("{}", &error.to_string()));
            }
        }
    }

    fn tick_splash(&mut self) {
        // 找到游戏后就不再倒计时，等游戏数据可读并附加成功再收起动画。
        if self.splash.tick(!self.game_detected) {
            self.timeout_dialog = true;
        }
    }

    fn play_intro_sound(&mut self) {
        if self.intro_sound_played {
            return;
        }
        self.intro_sound_played = true;
        self.play_sound("onj001");
    }

    fn play_sound(&self, name: &str) {
        sound::play(&self.assets, name);
    }

    fn set_error(&mut self, message: String) {
        self.close_menu();
        self.error_dialog = Some(message);
    }

    fn close_menu(&mut self) {
        self.menu_state.open = None;
        self.menu_state.open_submenu = None;
        self.menu_state.open_select = None;
    }

    fn dialog_open(&self) -> bool {
        self.prompt_dialog.is_some()
            || self.about_dialog
            || self.error_dialog.is_some()
            || self.timeout_dialog
    }

    fn set_cover(&mut self, ctx: &egui::Context, name: &'static str, bytes: Option<&'static [u8]>) {
        self.cover_name = Some(name);
        if let Some(texture) = load_texture(ctx, &self.assets, "cover", bytes) {
            self.cover_texture = Some(texture);
        }
    }

    fn set_cover_placeholder(&mut self, ctx: &egui::Context) {
        self.set_cover(
            ctx,
            config::COVER_PLACEHOLDER,
            self.assets.card(config::COVER_PLACEHOLDER),
        );
    }

    fn load_random_cover(&mut self, ctx: &egui::Context) {
        if let Some((name, bytes)) = self.assets.random_card_excluding(None) {
            self.set_cover(ctx, name, Some(bytes));
        }
    }

    fn cycle_cover(&mut self, ctx: &egui::Context) {
        if self.cover_name == Some(config::COVER_PLACEHOLDER) {
            return;
        }
        let current = self.cover_name;
        if let Some((name, bytes)) = self.assets.random_card_excluding(current) {
            self.set_cover(ctx, name, Some(bytes));
            self.cover_flip = COVER_FLIP_SECS;
            self.play_sound("Select");
        }
    }

    fn draw_header(&mut self, ui: &mut egui::Ui) {
        let text = self.text();
        let ctx = ui.ctx().clone();
        let painter = ui.painter();
        let title_font = theme::font(24.0);
        let title = text.title;
        painter.text(
            HEADER_TITLE_POS,
            Align2::LEFT_TOP,
            title,
            title_font.clone(),
            theme::INK,
        );
        let title_width = theme::measure(ui, title, title_font).x;
        ui.painter().line_segment(
            [
                Pos2::new(PAGE_MARGIN, HEADER_RULE_Y),
                Pos2::new(PAGE_MARGIN + 2.0 + title_width, HEADER_RULE_Y - 0.3),
            ],
            Stroke::new(1.0, theme::INK),
        );

        let sub_font = theme::font(12.0);
        let subtitle = text.subtitle;
        ui.painter().text(
            HEADER_SUBTITLE_POS,
            Align2::LEFT_TOP,
            subtitle,
            sub_font.clone(),
            theme::MUTED,
        );
        let subtitle_width = theme::measure(ui, subtitle, sub_font.clone()).x;
        let status_x = PAGE_MARGIN + subtitle_width + 10.0;

        let (status, connected) = if self.session.is_some() {
            (text.connected.to_string(), true)
        } else if self.multi_count >= 2 {
            (
                text.multi_detected
                    .replace("{}", &self.multi_count.to_string()),
                true,
            )
        } else if self.game_detected {
            (text.game_found.to_string(), true)
        } else {
            (text.waiting_game.to_string(), false)
        };
        if connected {
            ui.painter().circle_filled(
                Pos2::new(status_x + 3.0, HEADER_STATUS_DOT_Y),
                2.6,
                theme::INK,
            );
        } else {
            ui.painter().circle_stroke(
                Pos2::new(status_x + 3.0, HEADER_STATUS_DOT_Y),
                2.6,
                Stroke::new(1.0, theme::MUTED),
            );
        }
        ui.painter().text(
            Pos2::new(status_x + 9.0, HEADER_SUBTITLE_POS.y),
            Align2::LEFT_TOP,
            status,
            sub_font,
            if connected { theme::INK } else { theme::MUTED },
        );

        let close_rect = Rect::from_min_size(
            Pos2::new(config::WINDOW_WIDTH - CLOSE_X_OFFSET, CLOSE_Y),
            Vec2::splat(CLOSE_SIZE),
        );
        let close = ui.interact(
            close_rect,
            ui.scope_id().with("header-close"),
            Sense::click(),
        );
        let close_center = Pos2::new(close_rect.right() - 8.0, close_rect.center().y);
        let close_color = if close.hovered() {
            theme::INK
        } else {
            theme::MUTED
        };
        for (dx, dy) in [(-7.0, -7.0), (-7.0, 7.0)] {
            ui.painter().line_segment(
                [
                    close_center + Vec2::new(dx, dy),
                    close_center + Vec2::new(-dx, -dy),
                ],
                Stroke::new(1.6, close_color),
            );
        }
        if close.clicked() {
            self.close_menu();
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        let _ = close.on_hover_text(text.close);

        let drag = ui.interact(
            Rect::from_min_size(
                Pos2::ZERO,
                Vec2::new(
                    config::WINDOW_WIDTH - DRAG_WIDTH_OFFSET,
                    config::HEADER_HEIGHT,
                ),
            ),
            ui.scope_id().with("header-drag"),
            Sense::drag(),
        );
        if drag.drag_started() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }
    }

    fn draw_cover(&mut self, ui: &mut egui::Ui) {
        let center = COVER_CENTER;
        let cover_size = Vec2::new(config::COVER_WIDTH, config::COVER_HEIGHT);
        let hit = Rect::from_center_size(center, cover_size);
        let can_cycle = self.cover_name != Some(config::COVER_PLACEHOLDER);
        let interactive = can_cycle && !self.dialog_open() && self.menu_state.open.is_none();
        let response = ui.interact(
            hit,
            ui.scope_id().with("cover"),
            if interactive {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        let hovered = response.hovered() && interactive;
        if hovered {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if response.clicked() && interactive {
            let ctx = ui.ctx().clone();
            self.cycle_cover(&ctx);
        }

        let dt = ui.input(|input| input.stable_dt).min(0.1);
        self.cover_flip = (self.cover_flip - dt).max(0.0);
        let (squash, wobble) = if self.cover_flip > 0.0 {
            ui.ctx().request_repaint();
            let progress = 1.0 - self.cover_flip / COVER_FLIP_SECS;
            let squash = 1.0 - (progress * std::f32::consts::PI).sin() * 0.35;
            let wobble = (progress * std::f32::consts::TAU).sin() * (1.0 - progress) * 0.06;
            (squash, wobble)
        } else {
            (1.0, 0.0)
        };
        let angle = COVER_ANGLE_DEG.to_radians() + wobble;
        let drawn_size = Vec2::new(cover_size.x * squash, cover_size.y);

        if hovered {
            theme::rotated_rect(
                ui.painter(),
                center,
                drawn_size + Vec2::splat(COVER_HOVER_OUTLINE),
                angle,
                Color32::TRANSPARENT,
                Stroke::new(1.0, theme::MUTED),
            );
        }
        theme::rotated_rect(
            ui.painter(),
            center + Vec2::splat(COVER_SHADOW_OFFSET),
            drawn_size,
            angle,
            theme::shadow(),
            Stroke::NONE,
        );
        theme::rotated_rect(
            ui.painter(),
            center,
            drawn_size,
            angle,
            Color32::WHITE,
            Stroke::new(if hovered { 2.0 } else { 1.5 }, theme::INK),
        );
        if let Some(texture) = &self.cover_texture {
            theme::rotated_texture(
                ui.painter(),
                texture.id(),
                center,
                Vec2::new(COVER_IMAGE_SIZE.x * squash, COVER_IMAGE_SIZE.y),
                angle,
            );
        }
        theme::tape(
            ui.painter(),
            TAPE_CENTER,
            TAPE_SIZE,
            TAPE_ANGLE_DEG.to_radians(),
        );
    }

    fn draw_content(&mut self, ui: &mut egui::Ui) {
        let text = self.text();
        let content = Rect::from_min_size(
            Pos2::new(CONTENT_X, CONTENT_Y),
            Vec2::new(CONTENT_WIDTH, CONTENT_HEIGHT),
        );
        let attached = self.session.is_some();
        let hint = if attached {
            text.hint_attached
        } else if self.multi_count >= 2 {
            text.hint_multi
        } else if self.game_detected {
            text.hint_reading
        } else {
            text.hint_disconnected
        };
        ui.painter().text(
            content.left_top() + Vec2::new(0.0, 2.0),
            Align2::LEFT_TOP,
            text.features,
            theme::font(12.0),
            theme::MUTED,
        );
        ui.painter().text(
            Pos2::new(content.right(), content.top() + 3.0),
            Align2::RIGHT_TOP,
            hint,
            theme::mono(10.0),
            theme::MUTED,
        );
        ui.painter().line_segment(
            [
                Pos2::new(content.left(), content.top() + CONTENT_HEADER_HEIGHT - 1.0),
                Pos2::new(content.right(), content.top() + CONTENT_HEADER_HEIGHT - 1.0),
            ],
            Stroke::new(1.0, theme::LINE),
        );

        if attached {
            let list = Rect::from_min_size(
                Pos2::new(content.left(), content.top() + CONTENT_HEADER_HEIGHT),
                Vec2::new(content.width(), content.height() - CONTENT_HEADER_HEIGHT),
            );
            // 菜单或弹窗打开时，功能列表只显示，不响应点击。
            let interactive = self.menu_state.open.is_none() && !self.dialog_open();
            if let Some(action) =
                feature_list::draw(ui, list, &self.features, interactive, self.language)
            {
                self.toggle_feature(action);
            }
        } else {
            let kind = if self.multi_count >= 2 {
                ConnectionState::Multiple(self.multi_count)
            } else if self.game_detected {
                ConnectionState::Waiting
            } else {
                ConnectionState::Disconnected
            };
            let state_rect = Rect::from_min_max(
                Pos2::new(content.left(), content.top() + CONTENT_HEADER_HEIGHT),
                Pos2::new(content.right(), content.bottom()),
            );
            connection::draw(
                ui,
                state_rect,
                kind,
                &self.multi_images,
                &self.multi_selection,
                self.radio_image.as_ref(),
                self.language,
            );
        }
    }

    fn draw_footer(&self, ui: &mut egui::Ui) {
        let text = self.text();
        let left = if self.session.is_some() {
            text.footer_hotkeys
        } else if self.multi_count >= 2 {
            text.footer_multi
        } else if self.game_detected {
            text.footer_refreshing
        } else {
            text.footer_waiting
        };
        let y = config::WINDOW_HEIGHT - config::FOOTER_HEIGHT / 2.0 - 4.0;
        ui.painter().text(
            Pos2::new(PAGE_MARGIN, y),
            Align2::LEFT_CENTER,
            left,
            theme::font(10.0),
            theme::MUTED,
        );
        ui.painter().text(
            Pos2::new(config::WINDOW_WIDTH - PAGE_MARGIN, y),
            Align2::RIGHT_CENTER,
            config::VERSION_LABEL,
            theme::mono(10.0),
            theme::MUTED,
        );
    }

    fn draw_dialogs(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // 同一时间只显示一个弹窗，按输入、错误、超时、关于的顺序处理。
        if let Some(mut dialog) = self.prompt_dialog.take() {
            let current = self.menu_values.get(dialog.field);
            match dialogs::draw_prompt(ui, &mut dialog, current, self.language) {
                PromptResult::Submit(value) => {
                    self.write_menu_value(dialog.field, value, "Enter");
                }
                PromptResult::Cancel => {
                    self.play_sound("Cancel");
                }
                PromptResult::None => {
                    self.prompt_dialog = Some(dialog);
                }
            }
            return;
        }

        if let Some(message) = self.error_dialog.take() {
            if !dialogs::draw_error(ui, &message, self.language) {
                self.error_dialog = Some(message);
            }
            return;
        }

        if self.timeout_dialog {
            if dialogs::draw_timeout(ui, self.language) {
                self.timeout_dialog = false;
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
            return;
        }

        if self.about_dialog && dialogs::draw_about(ui, self.icon_texture.as_ref(), self.language) {
            self.about_dialog = false;
        }
    }
}

impl eframe::App for TrainerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.poll(&ctx);
        self.poll_hotkeys();
        self.tick_splash();

        let full = ui.max_rect();
        ui.painter().rect_filled(full, 0.0, theme::PAPER);

        if self.splash.active {
            if self.splash.draw(ui, self.language) {
                ctx.send_viewport_cmd(ViewportCommand::Close);
                return;
            }
            ctx.request_repaint_after(Duration::from_millis(100));
            return;
        }

        self.draw_header(ui);
        let dialog_open = self.dialog_open();
        menu::draw_tabs(
            ui,
            &mut self.menu_state,
            self.session.is_some(),
            !dialog_open,
            self.language,
        );
        self.draw_cover(ui);
        self.draw_content(ui);
        self.draw_footer(ui);

        if self.menu_state.open.is_some()
            && !dialog_open
            && let Some(action) =
                menu::draw_panels(ui, &self.menu_values, &mut self.menu_state, self.language)
        {
            self.apply_menu_action(&ctx, action);
        }

        self.draw_dialogs(&ctx, ui);
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn install_fonts(ctx: &egui::Context, assets: &AssetService) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "lolita".to_string(),
        Arc::new(FontData::from_static(assets.font())),
    );

    // 标题和正文优先用内嵌的 Lolita，缺字时回退到 egui 默认字体。
    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        family.insert(0, "lolita".to_string());
    }
    if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
        family.push("lolita".to_string());
    }
    ctx.set_fonts(fonts);

    ctx.all_styles_mut(|style| {
        style
            .text_styles
            .insert(TextStyle::Body, FontId::new(14.0, FontFamily::Proportional));
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(12.0, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Small,
            FontId::new(10.0, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(24.0, FontFamily::Proportional),
        );
    });
}

fn select_multi_images(available: usize, count: usize) -> Vec<usize> {
    let take = count.min(available);
    // 部分 Fisher-Yates：只打乱前 count 个，避免为一次展示打乱整个数组。
    let mut indices: Vec<usize> = (0..available).collect();
    for index in 0..take {
        let swap_with = index + fastrand::usize(..available - index);
        indices.swap(index, swap_with);
    }
    indices.truncate(take);
    indices
}

fn load_texture(
    ctx: &egui::Context,
    assets: &AssetService,
    name: &str,
    bytes: Option<&'static [u8]>,
) -> Option<TextureHandle> {
    let bytes = bytes?;
    let image = assets.load_color_image(bytes).ok()?;
    Some(ctx.load_texture(name, image, egui::TextureOptions::LINEAR))
}
