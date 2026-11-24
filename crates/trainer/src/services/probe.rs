//! 扫描游戏进程，用代码特征和字段范围确认目标。

use anyhow::{Context, Result};
use std::mem::size_of;
use windows::Win32::Foundation::{CloseHandle, ERROR_ACCESS_DENIED};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};

use crate::config::pointer::MenuValues;
use crate::config::{GAME_PROCESS_NAMES, SINGLE_INSTANCE_PATTERN_OFFSET};
use crate::services::memory::GameMemory;
use crate::services::process::GameProcess;
use crate::services::win32::from_wide;

/// 通过了 exe 代码特征检查的进程。
/// `values` 为 `Some` 表示已知内存字段当前已初始化且读得出正常值。
pub struct FoundGame {
    pub process: GameProcess,
    pub values: Option<MenuValues>,
}

pub fn find_games() -> Result<Vec<FoundGame>> {
    // SAFETY: 用 pid 0 + TH32CS_SNAPPROCESS 枚举进程是标准做法。
    // 快照句柄归本函数所有，返回前会关掉。
    let snapshot =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }.context("无法创建进程快照")?;

    let mut entry = PROCESSENTRY32W {
        dwSize: size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    let mut games = Vec::new();
    let mut open_error = None;
    // 同名进程可能不止一个。打开失败的只记录 ACCESS_DENIED，最后再统一提示管理员权限。
    // SAFETY: snapshot 是有效快照，entry.dwSize 已经填好。
    if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
        loop {
            let name = from_wide(&entry.szExeFile);
            if is_game_process(&name) {
                match GameProcess::open(entry.th32ProcessID) {
                    Ok(process) => {
                        if let Some(game) = inspect(process) {
                            games.push(game);
                        }
                    }
                    Err(error) => {
                        let access_denied = error
                            .downcast_ref::<windows::core::Error>()
                            .is_some_and(|error| error.code() == ERROR_ACCESS_DENIED.to_hresult());
                        if access_denied && open_error.is_none() {
                            open_error = Some(error);
                        }
                    }
                }
            }

            // SAFETY: 迭代过程中 snapshot 和 entry 一直有效。
            if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                break;
            }
        }
    }

    // SAFETY: snapshot 归本函数所有，后面不再用。
    let _ = unsafe { CloseHandle(snapshot) };

    if games.is_empty()
        && let Some(error) = open_error
    {
        return Err(error).context("检测到游戏进程，但无法打开。请尝试以管理员身份运行修改器");
    }

    Ok(games)
}

fn inspect(process: GameProcess) -> Option<FoundGame> {
    let memory = process.memory();
    if !has_game_code(&memory) {
        return None;
    }

    let values = memory
        .read_menu_values()
        .ok()
        .filter(MenuValues::is_valid_game_state);
    Some(FoundGame { process, values })
}

fn has_game_code(memory: &GameMemory) -> bool {
    let address = memory
        .module_base()
        .wrapping_add(SINGLE_INSTANCE_PATTERN_OFFSET);
    let mut bytes = [0u8; 15];
    if memory.read_into(address, &mut bytes).is_err() {
        return false;
    }

    // 匹配 WOLF 单实例检查开头的 push 0 / push imm32 / call / test eax,eax / je。
    // 不依赖完整函数，只要这几个字节还在就能认出版本。
    bytes[0] == 0x6A
        && bytes[1] == 0x00
        && bytes[2] == 0x68
        && bytes[7] == 0xE8
        && bytes[12] == 0x85
        && bytes[13] == 0xC0
        && bytes[14] == 0x74
}

fn is_game_process(name: &str) -> bool {
    GAME_PROCESS_NAMES
        .iter()
        .any(|candidate| name.eq_ignore_ascii_case(candidate))
}
