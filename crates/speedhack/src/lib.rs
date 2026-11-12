#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::undocumented_unsafe_blocks)]

//! 注入到游戏进程的时钟缩放 DLL。
//!
//! hook 三个常用计时函数，把真实时间乘以倍率后返回。修改器退出时，
//! 复制到游戏进程的看门狗句柄会触发还原，避免游戏停留在加速状态。

#[cfg(not(all(target_os = "windows", target_arch = "x86")))]
compile_error!("speedhack only supports 32-bit Windows targets");

use std::ffi::{CStr, c_void};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::thread;

use detour::{Threads, Transaction, static_detour};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0};
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
use windows_sys::Win32::System::Performance::QueryPerformanceCounter;
use windows_sys::Win32::System::SystemInformation::{GetTickCount, GetTickCount64};
use windows_sys::Win32::System::Threading::{INFINITE, WaitForSingleObject};

const SPEED_ONE: f32 = 1.0;

type GetTickCountFn = unsafe extern "system" fn() -> u32;
type GetTickCount64Fn = unsafe extern "system" fn() -> u64;
type QueryPerformanceCounterFn = unsafe extern "system" fn(*mut i64) -> i32;

static_detour! {
    static GetTickCountHook: unsafe extern "system" fn() -> u32;
    static GetTickCount64Hook: unsafe extern "system" fn() -> u64;
    static QueryPerformanceCounterHook: unsafe extern "system" fn(*mut i64) -> i32;
}

/// 虚拟时钟的基准点。
///
/// 改速前先把当前虚拟时间写进 `offset`，之后只缩放新增的真实时间，
/// 这样切换倍率时不会出现时间跳变。
#[derive(Clone, Copy)]
struct ClockState {
    gtc_base: u32,
    gtc_offset: u32,
    gtc64_base: u64,
    gtc64_offset: u64,
    qpc_base: i64,
    qpc_offset: i64,
    speed: f32,
    session: i32,
    next_session: i32,
}

impl ClockState {
    const fn new() -> Self {
        Self {
            gtc_base: 0,
            gtc_offset: 0,
            gtc64_base: 0,
            gtc64_offset: 0,
            qpc_base: 0,
            qpc_offset: 0,
            speed: SPEED_ONE,
            session: 0,
            next_session: 1,
        }
    }

    fn begin_session(&mut self) -> i32 {
        let session = self.next_session;
        self.next_session = self.next_session.wrapping_add(1).max(1);
        self.session = session;
        session
    }

    fn is_session(&self, session: i32) -> bool {
        self.session == session
    }

    /// 只让看门狗失效当前会话，避免旧会话的线程误伤新会话。
    fn invalidate_session(&mut self, session: i32) -> bool {
        if self.session == session {
            self.session = 0;
            true
        } else {
            false
        }
    }
}

static CLOCK: Mutex<ClockState> = Mutex::new(ClockState::new());
static HOOK_READY: AtomicBool = AtomicBool::new(false);
static HOOK_INIT_LOCK: Mutex<()> = Mutex::new(());

fn clock() -> MutexGuard<'static, ClockState> {
    CLOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// 修改器写入远程内存的请求布局。字段顺序和大小必须与 trainer 侧一致。
#[repr(C)]
pub struct SpeedRequest {
    speed: f32,
    session: i32,
}

// 修改器按同样的 8 字节布局写入请求。
const _: () = assert!(std::mem::size_of::<SpeedRequest>() == 8);

/// 把真实时钟的增量乘以倍率，再叠加到虚拟时钟基准上。
fn scaled_u32(real: u32, base: u32, offset: u32, speed: f32) -> u32 {
    let elapsed = real.wrapping_sub(base);
    // 先转 u64 再截断，让 32 位时钟按自身周期回绕，而不是在 3x 时提前饱和。
    let delta = (elapsed as f64 * speed as f64) as u64;
    offset.wrapping_add(delta as u32)
}

fn scaled_u64(real: u64, base: u64, offset: u64, speed: f32) -> u64 {
    let elapsed = real.wrapping_sub(base);
    offset.wrapping_add((elapsed as f64 * speed as f64) as u64)
}

fn scaled_i64(real: i64, base: i64, offset: i64, speed: f32) -> i64 {
    let elapsed = real.wrapping_sub(base);
    offset.wrapping_add((elapsed as f64 * speed as f64) as i64)
}

extern "system" fn get_tick_count_detour() -> u32 {
    let state = clock();
    // SAFETY: hook 已初始化并启用，可以安全调用。
    let real = unsafe { GetTickCountHook.call() };
    scaled_u32(real, state.gtc_base, state.gtc_offset, state.speed)
}

extern "system" fn get_tick_count64_detour() -> u64 {
    let state = clock();
    // SAFETY: hook 已初始化并启用，可以安全调用。
    let real = unsafe { GetTickCount64Hook.call() };
    scaled_u64(real, state.gtc64_base, state.gtc64_offset, state.speed)
}

extern "system" fn query_performance_counter_detour(counter: *mut i64) -> i32 {
    if counter.is_null() {
        return 0;
    }

    let state = clock();
    // SAFETY: hook 已初始化并启用。调用方保证 `counter`
    // 是有效的 LARGE_INTEGER 指针。
    let result = unsafe { QueryPerformanceCounterHook.call(counter) };
    if result == 0 {
        return 0;
    }

    // SAFETY: 原始调用成功，已经写入计数值。
    let real = unsafe { *counter };
    // SAFETY: `counter` 已经检查过，这次调用期间仍然有效。
    unsafe {
        *counter = scaled_i64(real, state.qpc_base, state.qpc_offset, state.speed);
    }
    1
}

fn resolve_symbol(module: &str, symbol: &CStr) -> Option<*const c_void> {
    let module_name: Vec<u16> = module.encode_utf16().chain(std::iter::once(0)).collect();
    // SAFETY: `module_name` 是以 NUL 结尾的 UTF-16 字符串。
    let module = unsafe { GetModuleHandleW(module_name.as_ptr()) };
    if module.is_null() {
        return None;
    }

    // SAFETY: `module` 是有效句柄，`symbol` 以 NUL 结尾。
    let address = unsafe { GetProcAddress(module, symbol.as_ptr() as *const u8) };
    address.map(|function| function as *const c_void)
}

fn initialize_hooks() -> bool {
    let (Some(get_tick_count), Some(get_tick_count64), Some(query_performance_counter)) = (
        resolve_symbol("kernel32.dll", c"GetTickCount"),
        resolve_symbol("kernel32.dll", c"GetTickCount64"),
        resolve_symbol("kernel32.dll", c"QueryPerformanceCounter"),
    ) else {
        return false;
    };

    // SAFETY: 地址来自 kernel32，签名与 Win32 文档一致。
    let get_tick_count: GetTickCountFn = unsafe { std::mem::transmute(get_tick_count) };
    // SAFETY: 同上。
    let get_tick_count64: GetTickCount64Fn = unsafe { std::mem::transmute(get_tick_count64) };
    // SAFETY: 同上。
    let query_performance_counter: QueryPerformanceCounterFn =
        unsafe { std::mem::transmute(query_performance_counter) };

    // initialize 只准备 trampoline，commit 才真正写入入口补丁。
    // SAFETY: 目标函数和 detour 函数签名一致。
    // 调用方保证 `Setup` 只执行一次。
    unsafe {
        if GetTickCountHook
            .initialize(get_tick_count, || get_tick_count_detour())
            .is_err()
        {
            return false;
        }
        if GetTickCount64Hook
            .initialize(get_tick_count64, || get_tick_count64_detour())
            .is_err()
        {
            return false;
        }
        if QueryPerformanceCounterHook
            .initialize(query_performance_counter, |counter| {
                query_performance_counter_detour(counter)
            })
            .is_err()
        {
            return false;
        }

        // 装 hook 前先记下真实时钟，让虚拟时钟从当前游戏时间起步。
        let gtc = GetTickCount();
        let gtc64 = GetTickCount64();
        let mut qpc = 0i64;
        let qpc_ok = QueryPerformanceCounter(&mut qpc) != 0;

        let mut state = clock();
        state.gtc_base = gtc;
        state.gtc_offset = gtc;
        state.gtc64_base = gtc64;
        state.gtc64_offset = gtc64;
        if qpc_ok {
            state.qpc_base = qpc;
            state.qpc_offset = qpc;
        }
        state.speed = SPEED_ONE;
    }

    let mut transaction = Transaction::new();
    transaction
        .enable(&GetTickCountHook)
        .enable(&GetTickCount64Hook)
        .enable(&QueryPerformanceCounterHook);

    // SAFETY: 三个 hook 都已初始化。挂起所有线程后写入入口补丁，
    // 即使游戏正在运行也安全。
    unsafe { transaction.commit(Threads::All) }.is_ok()
}

/// 切换倍率前先把当前虚拟时间固化到基准，保证时间连续。
fn apply_speed_locked(state: &mut ClockState, new_speed: f32) {
    // SAFETY: hook 已初始化并启用，这里通过 trampoline 调用原函数。
    let gtc_real = unsafe { GetTickCountHook.call() };
    // SAFETY: 同上。
    let gtc64_real = unsafe { GetTickCount64Hook.call() };

    let gtc_virtual = scaled_u32(gtc_real, state.gtc_base, state.gtc_offset, state.speed);
    let gtc64_virtual = scaled_u64(
        gtc64_real,
        state.gtc64_base,
        state.gtc64_offset,
        state.speed,
    );

    state.gtc_base = gtc_real;
    state.gtc_offset = gtc_virtual;
    state.gtc64_base = gtc64_real;
    state.gtc64_offset = gtc64_virtual;

    let mut qpc_real = 0i64;
    // SAFETY: hook 已初始化并启用，`qpc_real` 可写。
    if unsafe { QueryPerformanceCounterHook.call(&mut qpc_real) } != 0 {
        let qpc_virtual = scaled_i64(qpc_real, state.qpc_base, state.qpc_offset, state.speed);
        state.qpc_base = qpc_real;
        state.qpc_offset = qpc_virtual;
    }

    state.speed = new_speed;
}

fn start_watchdog(watchdog: usize, session: i32) -> bool {
    if watchdog == 0 {
        return true;
    }

    // 等待放在独立线程，避免阻塞 Setup。线程创建失败时由这里关闭句柄。
    let spawned = thread::Builder::new()
        .name("speedhack-watchdog".to_string())
        .spawn(move || {
            let handle = watchdog as HANDLE;
            // SAFETY: 修改器把自己的进程句柄复制到游戏进程，再传给 Setup。
            // 这个句柄由当前线程负责关闭。
            let result = unsafe { WaitForSingleObject(handle, INFINITE) };
            if result == WAIT_OBJECT_0 {
                let mut state = clock();
                if state.invalidate_session(session) {
                    apply_speed_locked(&mut state, SPEED_ONE);
                }
            }
            // SAFETY: 复制出来的句柄已经不用了。
            let _ = unsafe { CloseHandle(handle) };
        })
        .is_ok();

    if !spawned {
        // SAFETY: 线程没创建成功，这个复制出来的句柄由这里关闭。
        let _ = unsafe { CloseHandle(watchdog as HANDLE) };
    }
    spawned
}

/// 初始化只做一次。原子标记和互斥锁配合，避免并发 `Setup` 重复挂 hook。
fn ensure_hooks() -> bool {
    if HOOK_READY.load(Ordering::Acquire) {
        return true;
    }

    let _guard = HOOK_INIT_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if HOOK_READY.load(Ordering::Acquire) {
        return true;
    }
    if initialize_hooks() {
        HOOK_READY.store(true, Ordering::Release);
        true
    } else {
        false
    }
}

/// 由修改器通过远程线程调用，返回本次会话号。
///
/// `watchdog` 是修改器进程句柄在当前进程中的副本。修改器退出后它会被
/// 标记为 signaled，看门狗线程收到后把速度还原为 1x。
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn Setup(watchdog: usize) -> i32 {
    if !ensure_hooks() {
        if watchdog != 0 {
            // SAFETY: 初始化失败时没有线程负责这个复制出来的句柄。
            let _ = unsafe { CloseHandle(watchdog as HANDLE) };
        }
        return 0;
    }

    let session = {
        let mut state = clock();
        let session = state.begin_session();
        apply_speed_locked(&mut state, SPEED_ONE);
        session
    };
    if !start_watchdog(watchdog, session) {
        let mut state = clock();
        state.invalidate_session(session);
        return 0;
    }
    session
}

/// 修改当前会话的速度倍率。
///
/// # Safety
///
/// `request` 必须在调用期间指向有效的 `SpeedRequest`。
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "system" fn SetSpeed(request: *const SpeedRequest) -> i32 {
    // SAFETY: 调用方保证 `request` 为 null 或指向有效的 `SpeedRequest`。
    let Some(request) = (unsafe { request.as_ref() }) else {
        return 0;
    };

    if !HOOK_READY.load(Ordering::Acquire) || !request.speed.is_finite() || request.speed <= 0.0 {
        return 0;
    }

    let mut state = clock();
    // 会话号防止旧实例或延迟请求改到新会话的速度。
    if !state.is_session(request.session) {
        return 0;
    }
    apply_speed_locked(&mut state, request.speed);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_one_preserves_elapsed_time() {
        assert_eq!(scaled_u32(1_500, 1_000, 1_000, 1.0), 1_500);
        assert_eq!(scaled_u64(1_500, 1_000, 1_000, 1.0), 1_500);
        assert_eq!(scaled_i64(1_500, 1_000, 1_000, 1.0), 1_500);
    }

    #[test]
    fn speed_multiplies_elapsed_time() {
        assert_eq!(scaled_u32(1_010, 1_000, 1_000, 3.0), 1_030);
        assert_eq!(scaled_u64(1_010, 1_000, 1_000, 3.0), 1_030);
        assert_eq!(scaled_i64(1_010, 1_000, 1_000, 3.0), 1_030);
    }

    #[test]
    fn tick_count_wraps_instead_of_saturating() {
        let expected = 4_500_000_000u64 as u32;
        assert_eq!(scaled_u32(1_500_000_000, 0, 0, 3.0), expected);
    }

    #[test]
    fn virtual_clock_is_monotonic() {
        let speed = 3.0;
        let mut base = 1_000;
        let mut offset = 1_000;
        let mut last = 0;
        for real in (1_000..1_100).step_by(7) {
            let virtual_time = scaled_u64(real, base, offset, speed);
            assert!(virtual_time >= last);
            last = virtual_time;
            base = real;
            offset = virtual_time;
        }
    }

    #[test]
    fn new_session_rejects_old_and_late_requests() {
        let mut state = ClockState::new();
        let old = state.begin_session();
        let current = state.begin_session();
        assert!(!state.is_session(old));
        assert!(state.is_session(current));
        assert!(!state.invalidate_session(old));
        assert!(state.is_session(current));
        assert!(state.invalidate_session(current));
        assert!(!state.is_session(current));
        assert_eq!(state.session, 0);
    }
}
