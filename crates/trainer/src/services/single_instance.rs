//! 用命名互斥锁限制同时只运行一个修改器。

use anyhow::{Context, Result};
use windows::Win32::Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError, HANDLE};
use windows::Win32::System::Threading::CreateMutexW;
use windows::core::PCWSTR;

use crate::services::win32::to_wide;

pub struct SingleInstance {
    handle: HANDLE,
}

impl SingleInstance {
    pub fn acquire() -> Result<Option<Self>> {
        let name = to_wide("Local\\ImoutoLifeFantasyTrainer");
        // SAFETY: name 是以 NUL 结尾的 UTF-16 字符串。
        // 每会话互斥锁不用设置安全属性。
        let handle = unsafe { CreateMutexW(None, false, PCWSTR(name.as_ptr())) }
            .context("无法创建单实例互斥锁")?;

        // ERROR_ALREADY_EXISTS 表示已有实例在运行。句柄仍然要关闭，直接返回 None 会漏。
        // SAFETY: GetLastError 没有前置条件，调用完马上读取即可。
        if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
            // SAFETY: handle 是 CreateMutexW 返回的，这个分支之后不再用。
            let _ = unsafe { CloseHandle(handle) };
            return Ok(None);
        }

        Ok(Some(Self { handle }))
    }
}

impl Drop for SingleInstance {
    fn drop(&mut self) {
        // SAFETY: self.handle 归本结构体所有，这里只关一次。
        let _ = unsafe { CloseHandle(self.handle) };
    }
}
