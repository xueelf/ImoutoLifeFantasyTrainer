//! 常用的 UTF-16 转换、打开链接和消息框封装。

use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::{
    MB_ICONINFORMATION, MB_OK, MessageBoxW, SW_SHOWNORMAL,
};
use windows::core::PCWSTR;

pub fn to_wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn from_wide(value: &[u16]) -> String {
    let len = value.iter().position(|c| *c == 0).unwrap_or(value.len());
    String::from_utf16_lossy(&value[..len])
}

pub fn open_url(url: &str) {
    let operation = to_wide("open");
    let target = to_wide(url);
    // SAFETY: 两个 UTF-16 字符串都以 NUL 结尾，调用期间有效。
    unsafe {
        ShellExecuteW(
            None,
            PCWSTR(operation.as_ptr()),
            PCWSTR(target.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        );
    }
}

pub fn message_box(title: &str, text: &str) {
    let title = to_wide(title);
    let text = to_wide(text);
    // SAFETY: 两个 UTF-16 字符串都以 NUL 结尾，调用期间有效。
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(text.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}
