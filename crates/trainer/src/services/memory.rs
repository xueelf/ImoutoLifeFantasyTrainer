//! 通过进程句柄读写游戏内存，并解析统一的指针链。

use anyhow::{Context, Result};
use std::ffi::c_void;
use windows::Win32::Foundation::{HANDLE, WAIT_TIMEOUT};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Threading::WaitForSingleObject;

use crate::config::CURRENT_OFFSET;
use crate::config::pointer::{MENU_BLOCK_SIZE, MenuValues};

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

    pub fn read_into(&self, address: u32, buffer: &mut [u8]) -> Result<()> {
        // SAFETY: 句柄有 PROCESS_VM_READ 权限，只要 GameMemory 还在就有效。
        // buffer 在调用期间有效且可写满 buffer.len() 字节。
        unsafe {
            ReadProcessMemory(
                self.handle(),
                address as usize as *const c_void,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len(),
                None,
            )
        }
        .with_context(|| format!("读取游戏内存失败：0x{address:08X}"))
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
        let mut bytes = [0u8; 4];
        self.read_into(address, &mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn write_u32(&self, address: u32, value: u32) -> Result<()> {
        self.write_bytes(address, &value.to_le_bytes())
    }

    /// 解析所有字段共用的 root：先读 global 指针，再解引用一次。
    pub fn resolve_root(&self) -> Result<u32> {
        // root = [[module_base + CURRENT_OFFSET] + 0]
        let global = self.module_base.wrapping_add(CURRENT_OFFSET);
        let root = self.read_u32(global)?;
        self.read_u32(root)
    }

    pub fn read_at_root(&self, root: u32, field_offset: u32) -> Result<i32> {
        Ok(self.read_u32(root.wrapping_add(field_offset))? as i32)
    }

    pub fn read_menu_values(&self) -> Result<MenuValues> {
        let root = self.resolve_root()?;
        let mut block = [0u8; MENU_BLOCK_SIZE];
        self.read_into(root, &mut block)?;
        Ok(MenuValues::from_root_block(&block))
    }

    pub fn write_at_root(&self, root: u32, field_offset: u32, value: i32) -> Result<()> {
        self.write_u32(root.wrapping_add(field_offset), value as u32)
    }

    pub fn write_offset(&self, field_offset: u32, value: i32) -> Result<()> {
        let root = self.resolve_root()?;
        self.write_at_root(root, field_offset, value)
    }
}
