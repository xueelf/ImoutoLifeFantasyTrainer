//! 异步播放内嵌 WAV 音效。

use windows::Win32::Media::Audio::{PlaySoundA, SND_ASYNC, SND_MEMORY, SND_NODEFAULT};
use windows::core::PCSTR;

use crate::services::assets::AssetService;

#[derive(Clone, Copy)]
pub enum Sound {
    Enter,
    Cancel,
    Select,
    Intro,
}

impl Sound {
    pub const fn path(self) -> &'static str {
        match self {
            Self::Enter => "sounds/Enter.wav",
            Self::Cancel => "sounds/Cancel.wav",
            Self::Select => "sounds/Select.wav",
            Self::Intro => "sounds/onj001.wav",
        }
    }
}

pub fn play(assets: &AssetService, sound: Sound) {
    let Some(bytes) = assets.sound(sound) else {
        return;
    };
    // SAFETY: bytes 指向内嵌的静态 WAV 数据，比异步播放活得久。
    // 标志位是内存播放的标准组合。
    unsafe {
        let _ = PlaySoundA(
            PCSTR(bytes.as_ptr()),
            None,
            SND_MEMORY | SND_ASYNC | SND_NODEFAULT,
        );
    }
}
