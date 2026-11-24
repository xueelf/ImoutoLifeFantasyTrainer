//! 全局热键注册与事件轮询。

use anyhow::{Context, Result};
use global_hotkey::hotkey::{Code, HotKey};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use std::collections::HashMap;

use crate::config::feature::{FEATURES, FeatureAction, Hotkey};

pub struct HotkeyService {
    _manager: GlobalHotKeyManager,
    actions: HashMap<u32, FeatureAction>,
    failed: Vec<Hotkey>,
}

impl HotkeyService {
    pub fn new() -> Result<Self> {
        let manager = GlobalHotKeyManager::new().context("无法创建全局热键管理器")?;
        let mut actions = HashMap::new();
        let mut failed = Vec::new();

        for feature in FEATURES {
            let hotkey = HotKey::new(None, code(feature.hotkey));
            // 被其他程序占用的热键只记录下来，由启动流程提示用户。
            if manager.register(hotkey).is_ok() {
                actions.insert(hotkey.id(), feature.action);
            } else {
                failed.push(feature.hotkey);
            }
        }

        Ok(Self {
            _manager: manager,
            actions,
            failed,
        })
    }

    pub fn failed(&self) -> &[Hotkey] {
        &self.failed
    }

    pub fn poll(&self) -> Vec<FeatureAction> {
        let receiver = GlobalHotKeyEvent::receiver();
        let mut pressed = Vec::new();

        // 每次轮询清空事件队列，只处理按下状态。
        while let Ok(event) = receiver.try_recv() {
            if event.state != HotKeyState::Pressed {
                continue;
            }
            if let Some(action) = self.actions.get(&event.id) {
                pressed.push(*action);
            }
        }

        pressed
    }
}

fn code(hotkey: Hotkey) -> Code {
    match hotkey {
        Hotkey::F1 => Code::F1,
        Hotkey::F2 => Code::F2,
        Hotkey::F3 => Code::F3,
        Hotkey::F9 => Code::F9,
        Hotkey::F10 => Code::F10,
        Hotkey::F11 => Code::F11,
    }
}
