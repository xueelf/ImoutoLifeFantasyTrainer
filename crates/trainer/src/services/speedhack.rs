//! 把内嵌的 Speedhack DLL 注入游戏进程，并通过远程线程调用其导出函数。

use anyhow::{Context, Result, anyhow, bail};
use std::ffi::c_void;
use std::fs;
use std::path::PathBuf;
use windows::Win32::Foundation::{
    CloseHandle, DUPLICATE_HANDLE_OPTIONS, DuplicateHandle, FreeLibrary, HANDLE, WAIT_OBJECT_0,
    WAIT_TIMEOUT,
};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress, LoadLibraryW};
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx, VirtualFreeEx,
};
use windows::Win32::System::Threading::{
    CreateRemoteThread, GetCurrentProcessId, GetExitCodeThread, LPTHREAD_START_ROUTINE,
    OpenProcess, PROCESS_DUP_HANDLE, PROCESS_SYNCHRONIZE, WaitForSingleObject,
};
use windows::core::{PCSTR, PCWSTR};

use crate::config::SPEEDHACK_DLL_NAME;
use crate::services::process::GameProcess;
use crate::services::win32::to_wide;

const BOOST_SPEED: f32 = 3.0;
const REMOTE_TIMEOUT_MS: u32 = 10_000;
const SPEED_REQUEST_SIZE: usize = 8;

static SPEEDHACK_DLL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/speedhack.dll"));

pub struct SpeedhackService {
    process_handle: HANDLE,
    remote_module: u32,
    session: i32,
    injected: bool,
    state_unknown: bool,
    set_speed_rva: u32,
}

impl SpeedhackService {
    pub fn new(process: &GameProcess, state_unknown: bool) -> Self {
        Self {
            process_handle: process.handle(),
            remote_module: 0,
            session: 0,
            injected: false,
            state_unknown,
            set_speed_rva: 0,
        }
    }

    pub fn state_unknown(&self) -> bool {
        self.state_unknown
    }

    pub fn enable(&mut self) -> Result<()> {
        // 上一次远程调用超时后，DLL 状态可能已经无法确认，只允许关闭修改器触发看门狗恢复。
        if self.state_unknown {
            bail!("Speedhack 状态未知，请关闭修改器以触发恢复");
        }
        self.ensure_injected()?;
        self.set_speed(BOOST_SPEED)
    }

    pub fn disable(&mut self) -> Result<()> {
        // 游戏进程已经退出时无需再通过远程线程恢复速度。
        // SAFETY: self.process_handle 来自 GameProcess，具有 PROCESS_SYNCHRONIZE 权限。
        if unsafe { WaitForSingleObject(self.process_handle, 0) } != WAIT_TIMEOUT {
            return Ok(());
        }
        if self.state_unknown {
            bail!("Speedhack 状态未知，请关闭修改器以触发恢复");
        }
        if self.injected {
            self.set_speed(1.0)?;
        }
        Ok(())
    }

    fn ensure_injected(&mut self) -> Result<()> {
        if self.injected {
            return Ok(());
        }

        // 注入顺序：释放 DLL 到临时目录，在游戏里加载它，复制看门狗句柄，再调用 Setup。
        let dll_path = ensure_dll()?;
        let (setup_rva, set_speed_rva) = load_export_offsets(&dll_path)?;

        let path_text = dll_path
            .to_str()
            .context("Speedhack DLL 路径不是有效 UTF-8")?;
        let path_wide = to_wide(path_text);
        let path_bytes: Vec<u8> = path_wide
            .iter()
            .flat_map(|unit| unit.to_le_bytes())
            .collect();

        let remote_path = self.allocate(path_bytes.len())?;
        let mut path_may_still_run = false;
        let mut setup_may_still_run = false;
        let result = (|| -> Result<()> {
            self.write(remote_path, &path_bytes)?;

            let kernel32_name = to_wide("kernel32.dll");
            // SAFETY: 模块名是以 NUL 结尾的 UTF-16 字符串。
            let kernel32 = unsafe { GetModuleHandleW(PCWSTR(kernel32_name.as_ptr())) }
                .context("无法定位 kernel32.dll")?;
            // SAFETY: kernel32 是有效句柄，导出名以 NUL 结尾。
            let load_library =
                unsafe { GetProcAddress(kernel32, PCSTR(c"LoadLibraryW".as_ptr() as *const u8)) };
            let Some(load_library) = load_library else {
                bail!("无法定位 LoadLibraryW");
            };

            let module = self.remote_call(
                load_library as usize as u32,
                remote_path as usize as *const c_void,
                &mut path_may_still_run,
            )?;
            if module == 0 {
                bail!("游戏进程加载 Speedhack DLL 失败");
            }
            self.remote_module = module;

            let watchdog = self.create_watchdog_handle()?;
            let setup_result = match self.remote_call(
                module.wrapping_add(setup_rva),
                watchdog as usize as *const c_void,
                &mut setup_may_still_run,
            ) {
                Ok(result) => result,
                Err(error) => {
                    if !setup_may_still_run {
                        self.close_remote_handle(watchdog);
                    }
                    return Err(error);
                }
            };
            let session = setup_result as i32;
            if session <= 0 {
                bail!("Speedhack DLL 初始化失败");
            }

            self.session = session;
            self.injected = true;
            self.set_speed_rva = set_speed_rva;
            Ok(())
        })();

        // 超时后远程线程仍可能在使用这块路径缓冲区，此时不能释放。
        if !path_may_still_run {
            // SAFETY: remote_path 是在目标进程里用 VirtualAllocEx 分配的，
            // 这里释放一次。
            let _ = unsafe { VirtualFreeEx(self.process_handle, remote_path, 0, MEM_RELEASE) };
        }
        if path_may_still_run || setup_may_still_run {
            self.state_unknown = true;
            return Err(anyhow!("Speedhack 状态未知，请关闭修改器以触发恢复"));
        }
        result
    }

    fn set_speed(&mut self, speed: f32) -> Result<()> {
        let remote = self.allocate(SPEED_REQUEST_SIZE)?;
        let mut may_still_run = false;
        let result = (|| -> Result<()> {
            // 请求布局与 DLL 里的 SpeedRequest 一致：4 字节倍率 + 4 字节会话号。
            let mut request = [0u8; SPEED_REQUEST_SIZE];
            request[..4].copy_from_slice(&speed.to_le_bytes());
            request[4..].copy_from_slice(&self.session.to_le_bytes());
            self.write(remote, &request)?;
            let accepted = self.remote_call(
                self.remote_module.wrapping_add(self.set_speed_rva),
                remote as usize as *const c_void,
                &mut may_still_run,
            )?;
            if accepted != 1 {
                bail!("Speedhack DLL 拒绝变速请求");
            }
            Ok(())
        })();

        // 超时后远程线程仍可能读取这块请求缓冲区，此时不能释放。
        if !may_still_run {
            // SAFETY: remote 是在目标进程里用 VirtualAllocEx 分配的，
            // 这里释放一次。
            let _ = unsafe { VirtualFreeEx(self.process_handle, remote, 0, MEM_RELEASE) };
        }
        if may_still_run {
            self.state_unknown = true;
            return Err(anyhow!("Speedhack 状态未知，请关闭修改器以触发恢复"));
        }
        result
    }

    fn create_watchdog_handle(&self) -> Result<u32> {
        // 复制一个当前进程句柄到游戏进程。修改器退出后句柄进入 signaled 状态，
        // DLL 的看门狗线程据此把速度还原。
        // SAFETY: 用同步/复制句柄权限打开当前进程，没有额外前提。
        let self_handle = unsafe {
            OpenProcess(
                PROCESS_SYNCHRONIZE | PROCESS_DUP_HANDLE,
                false,
                GetCurrentProcessId(),
            )
        }
        .context("无法打开修改器进程句柄")?;

        let mut remote = HANDLE::default();
        // SAFETY: 两个源句柄都有效，目标进程有 PROCESS_DUP_HANDLE 权限。
        // remote 指向可写的栈变量。
        let duplicated = unsafe {
            DuplicateHandle(
                self_handle,
                self_handle,
                self.process_handle,
                &mut remote,
                PROCESS_SYNCHRONIZE.0,
                false,
                DUPLICATE_HANDLE_OPTIONS(0),
            )
        };
        // SAFETY: self_handle 归本函数所有，已经用完了。
        let _ = unsafe { CloseHandle(self_handle) };
        duplicated.context("无法为游戏进程创建看门狗句柄")?;
        Ok(remote.0 as usize as u32)
    }

    fn close_remote_handle(&self, handle: u32) {
        if handle == 0 {
            return;
        }
        // CreateRemoteThread 失败时 Setup 根本没有启动，需要由修改器收回复制到目标进程的句柄。
        // DUPLICATE_CLOSE_SOURCE (1) 会直接关闭源进程（此处即游戏进程）中的句柄。
        // SAFETY: self.process_handle 有 PROCESS_DUP_HANDLE 权限，handle 是刚复制进去的有效句柄。
        let _ = unsafe {
            DuplicateHandle(
                self.process_handle,
                HANDLE(handle as usize as *mut c_void),
                HANDLE::default(),
                std::ptr::null_mut(),
                0,
                false,
                DUPLICATE_HANDLE_OPTIONS(1),
            )
        };
    }

    fn allocate(&self, size: usize) -> Result<*mut c_void> {
        // SAFETY: 目标进程有 PROCESS_VM_OPERATION 权限。
        // 返回的指针会检查，之后也只配合这个进程句柄使用。
        let address = unsafe {
            VirtualAllocEx(
                self.process_handle,
                None,
                size,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };
        if address.is_null() {
            bail!("无法在游戏进程中分配内存");
        }
        Ok(address)
    }

    fn write(&self, address: *mut c_void, bytes: &[u8]) -> Result<()> {
        // SAFETY: address 是在目标进程里分配的，句柄有 PROCESS_VM_WRITE 权限。
        // bytes 在调用期间有效。
        unsafe {
            WriteProcessMemory(
                self.process_handle,
                address as *const c_void,
                bytes.as_ptr() as *const c_void,
                bytes.len(),
                None,
            )
        }
        .context("写入游戏进程失败")
    }

    /// 在游戏进程里执行一次远程调用。
    ///
    /// `may_still_run` 会告诉调用方：超时或等待失败时远程线程可能还在运行，
    /// 参数内存必须保留到进程退出，不能立即释放。
    fn remote_call(
        &self,
        start: u32,
        parameter: *const c_void,
        may_still_run: &mut bool,
    ) -> Result<u32> {
        *may_still_run = false;
        // 把远程地址转换成 CreateRemoteThread 需要的函数指针。
        // SAFETY: 调用方给出的是符合线程启动 ABI 的 32 位远程函数地址。
        // 这个地址只给当前远程线程用。
        let start: LPTHREAD_START_ROUTINE = Some(unsafe {
            std::mem::transmute::<usize, unsafe extern "system" fn(*mut c_void) -> u32>(
                start as usize,
            )
        });
        // SAFETY: 目标句柄有 PROCESS_CREATE_THREAD 权限。start 是有效的远程入口，
        // parameter 也符合被调用函数的参数要求。
        let thread = unsafe {
            CreateRemoteThread(
                self.process_handle,
                None,
                0,
                start,
                Some(parameter),
                0,
                None,
            )
        }
        .context("无法在游戏进程中创建远程线程")?;
        *may_still_run = true;

        // SAFETY: thread 是上面创建的有效线程句柄。
        let wait = unsafe { WaitForSingleObject(thread, REMOTE_TIMEOUT_MS) };
        if wait == WAIT_TIMEOUT {
            // 线程可能仍在运行，调用方必须保留参数内存。
            // SAFETY: thread 归本函数所有，这个分支之后不再用。
            let _ = unsafe { CloseHandle(thread) };
            bail!("游戏进程调用超时");
        }
        if wait != WAIT_OBJECT_0 {
            // 线程状态未知，调用方必须保留参数内存。
            // SAFETY: thread 归本函数所有，这个分支之后不再用。
            let _ = unsafe { CloseHandle(thread) };
            bail!("等待游戏进程调用失败");
        }
        *may_still_run = false;

        let mut exit_code = 0u32;
        // SAFETY: thread 是有效句柄且线程已结束，exit_code 可写。
        let result =
            unsafe { GetExitCodeThread(thread, &mut exit_code) }.context("无法读取远程线程返回值");
        // SAFETY: thread 归本函数所有，这里关闭一次。
        let _ = unsafe { CloseHandle(thread) };
        result?;
        Ok(exit_code)
    }
}

fn ensure_dll() -> Result<PathBuf> {
    // DLL 内嵌在 exe 里，运行时释放到临时目录。内容变化时覆盖旧文件。
    let directory = std::env::temp_dir().join("ImoutoLifeFantasyTrainer");
    fs::create_dir_all(&directory).context("无法创建临时目录")?;
    let path = directory.join(SPEEDHACK_DLL_NAME);

    let needs_write = match fs::read(&path) {
        Ok(existing) => existing.as_slice() != SPEEDHACK_DLL,
        Err(_) => true,
    };
    if needs_write {
        fs::write(&path, SPEEDHACK_DLL).context("无法释放 Speedhack DLL")?;
    }
    Ok(path)
}

fn load_export_offsets(dll_path: &std::path::Path) -> Result<(u32, u32)> {
    // 先在修改器进程里加载 DLL，只为了拿到导出函数地址，随后立即卸载并换算成 RVA。
    let wide = to_wide(&dll_path.to_string_lossy());
    // SAFETY: 路径是以 NUL 结尾的 UTF-16 字符串，模块在下面释放。
    let module =
        unsafe { LoadLibraryW(PCWSTR(wide.as_ptr())) }.context("无法加载 Speedhack DLL")?;

    let result = (|| -> Result<(u32, u32)> {
        // SAFETY: module 已加载且有效，导出名以 NUL 结尾。
        let setup = unsafe { GetProcAddress(module, PCSTR(c"Setup".as_ptr() as *const u8)) }
            .or_else(|| {
                // SAFETY: 同上，兼容 i686 stdcall 的装饰导出名。
                unsafe { GetProcAddress(module, PCSTR(c"Setup@4".as_ptr() as *const u8)) }
            });
        // SAFETY: module 已加载且有效，导出名以 NUL 结尾。
        let set_speed = unsafe { GetProcAddress(module, PCSTR(c"SetSpeed".as_ptr() as *const u8)) }
            .or_else(|| {
                // SAFETY: 同上，兼容 i686 stdcall 的装饰导出名。
                unsafe { GetProcAddress(module, PCSTR(c"SetSpeed@4".as_ptr() as *const u8)) }
            });
        let Some(setup) = setup else {
            bail!("Speedhack DLL 缺少 Setup");
        };
        let Some(set_speed) = set_speed else {
            bail!("Speedhack DLL 缺少 SetSpeed");
        };
        let base = module.0 as usize as u32;
        Ok((
            (setup as usize as u32).wrapping_sub(base),
            (set_speed as usize as u32).wrapping_sub(base),
        ))
    })();

    // SAFETY: module 归本函数所有，后面不再用。
    let _ = unsafe { FreeLibrary(module) };
    result
}
