//! 通过进程句柄读写游戏内存，并解析统一的指针链。

use anyhow::{Context, Result};
use std::ffi::c_void;
use windows::Win32::Foundation::{HANDLE, WAIT_TIMEOUT};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Threading::WaitForSingleObject;

use crate::config::CURRENT_OFFSET;
use crate::config::pointer::PointerChain;

// 句柄归 GameProcess 所有，这里只是借用。不能活得比所属的 GameSession 久。
#[derive(Clone, Copy)]
pub struct GameMemory {
    handle: isize,
    module_base: u32,
}

impl GameMemory {
    pub fn new(handle: isize, module_base: u32) -> Self {
        Self {
            handle,
            module_base,
        }
    }

    fn handle(&self) -> HANDLE {
        HANDLE(self.handle as *mut c_void)
    }

    pub fn module_base(&self) -> u32 {
        self.module_base
    }

    pub fn is_process_alive(&self) -> bool {
        // SAFETY: 句柄来自 GameProcess，具有 PROCESS_SYNCHRONIZE 权限，在所属会话存活期间有效。
        unsafe { WaitForSingleObject(self.handle(), 0) == WAIT_TIMEOUT }
    }

    pub fn read_bytes(&self, address: u32, size: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; size];
        // SAFETY: 句柄有 PROCESS_VM_READ 权限，只要 GameMemory 还在就有效。
        // buffer 足够写入 size 字节。
        unsafe {
            ReadProcessMemory(
                self.handle(),
                address as usize as *const c_void,
                buffer.as_mut_ptr() as *mut c_void,
                size,
                None,
            )
        }
        .with_context(|| format!("读取游戏内存失败：0x{address:08X}"))?;
        Ok(buffer)
    }

    fn write_bytes(&self, address: u32, bytes: &[u8]) -> Result<()> {
        // SAFETY: 句柄有 PROCESS_VM_WRITE 权限，只要 GameMemory 还在就有效。
        // bytes 在调用期间一直有效。
        unsafe {
            WriteProcessMemory(
                self.handle(),
                address as usize as *const c_void,
                bytes.as_ptr() as *const c_void,
                bytes.len(),
                None,
            )
        }
        .with_context(|| format!("写入游戏内存失败：0x{address:08X}"))
    }

    fn read_u32(&self, address: u32) -> Result<u32> {
        let bytes = self.read_bytes(address, 4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn write_u32(&self, address: u32, value: u32) -> Result<()> {
        self.write_bytes(address, &value.to_le_bytes())
    }

    /// 解析所有字段共用的 root：先读 global 指针，再解引用一次。
    fn resolve_root(&self) -> Result<u32> {
        // root = [[module_base + CURRENT_OFFSET] + 0]
        let global = self.module_base.wrapping_add(CURRENT_OFFSET);
        let root = self.read_u32(global)?;
        self.read_u32(root)
    }

    pub fn read_chain(&self, chain: &PointerChain) -> Result<i32> {
        // 字段地址 = root + field_offset，按小端 i32 读取。
        let root = self.resolve_root()?;
        Ok(self.read_u32(root.wrapping_add(chain.field_offset))? as i32)
    }

    pub fn write_offset(&self, field_offset: u32, value: i32) -> Result<()> {
        let root = self.resolve_root()?;
        self.write_u32(root.wrapping_add(field_offset), value as u32)
    }
}
