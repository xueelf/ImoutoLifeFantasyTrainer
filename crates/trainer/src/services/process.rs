//! 打开游戏进程并读取主模块基址。

use anyhow::{Context, Result};
use std::mem::size_of;
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_TIMEOUT};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, MODULEENTRY32W, Module32FirstW, TH32CS_SNAPMODULE,
    TH32CS_SNAPMODULE32,
};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_CREATE_THREAD, PROCESS_DUP_HANDLE, PROCESS_QUERY_INFORMATION,
    PROCESS_SYNCHRONIZE, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    WaitForSingleObject,
};

use crate::services::memory::GameMemory;

// GameMemory 和 SpeedhackService 借用这个句柄。GameSession 负责让本结构体比它们晚释放。
pub struct GameProcess {
    handle: HANDLE,
    pub pid: u32,
    module_base: u32,
}

impl GameProcess {
    pub fn open(pid: u32) -> Result<Self> {
        // 一次拿齐读写内存、冻结线程和注入所需的权限，避免后面反复打开进程。
        let access = PROCESS_CREATE_THREAD
            | PROCESS_VM_OPERATION
            | PROCESS_VM_READ
            | PROCESS_VM_WRITE
            | PROCESS_DUP_HANDLE
            | PROCESS_QUERY_INFORMATION
            | PROCESS_SYNCHRONIZE;

        // SAFETY: pid 来自 Toolhelp 的进程项，access 是标准的权限组合。
        let handle = unsafe { OpenProcess(access, false, pid) }
            .with_context(|| format!("无法打开游戏进程 {pid}"))?;

        let result = (|| -> Result<Self> {
            // SAFETY: pid 就是要读模块列表的进程。
            // snapshot 在下面关闭，出错时也不会漏。
            let snapshot =
                unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid) }
                    .context("无法创建模块快照")?;

            let mut entry = MODULEENTRY32W {
                dwSize: size_of::<MODULEENTRY32W>() as u32,
                ..Default::default()
            };

            // SAFETY: snapshot 有效，entry.dwSize 已经填好。
            let module_result = unsafe { Module32FirstW(snapshot, &mut entry) };
            // SAFETY: snapshot 归本函数所有，后面不再用。
            let _ = unsafe { CloseHandle(snapshot) };
            module_result.context("无法读取游戏主模块")?;

            Ok(Self {
                handle,
                pid,
                module_base: entry.modBaseAddr as usize as u32,
            })
        })();

        if result.is_err() {
            // SAFETY: 句柄是上面创建的，这时 Self 还没构造。
            let _ = unsafe { CloseHandle(handle) };
        }
        result
    }

    pub fn handle(&self) -> HANDLE {
        self.handle
    }

    pub fn is_alive(&self) -> bool {
        // SAFETY: self.handle 是具有 PROCESS_SYNCHRONIZE 权限的有效进程句柄。
        // 0 超时只检查进程内核对象是否仍处于未触发（运行中）状态。
        unsafe { WaitForSingleObject(self.handle, 0) == WAIT_TIMEOUT }
    }

    pub fn memory(&self) -> GameMemory {
        GameMemory::new(self.handle.0 as isize, self.module_base)
    }
}

impl Drop for GameProcess {
    fn drop(&mut self) {
        // SAFETY: self.handle 归本结构体所有，这里只关一次。
        let _ = unsafe { CloseHandle(self.handle) };
    }
}
