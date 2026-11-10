//! 异步播放内嵌 WAV 音效。

use windows::Win32::Media::Audio::{PlaySoundA, SND_ASYNC, SND_MEMORY, SND_NODEFAULT};
use windows::core::PCSTR;

use crate::services::assets::AssetService;

pub fn play(assets: &AssetService, name: &str) {
    let Some(bytes) = assets.sound(name) else {
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
