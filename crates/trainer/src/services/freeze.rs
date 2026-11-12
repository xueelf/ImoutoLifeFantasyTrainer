//! 后台冻结线程：周期把锁定值写回游戏，并在空闲时降低计时器精度。

use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use windows::Win32::Media::{timeBeginPeriod, timeEndPeriod};

use crate::services::memory::GameMemory;

#[derive(Clone, Copy)]
struct LockEntry {
    field_offset: u32,
    value: i32,
}

pub struct FreezeService {
    memory: GameMemory,
    entries: Arc<Mutex<Vec<LockEntry>>>,
    wake: Arc<Condvar>,
    stop: Arc<AtomicBool>,
    error: Arc<Mutex<Option<String>>>,
    thread: Option<JoinHandle<()>>,
}

impl FreezeService {
    pub fn new(memory: GameMemory) -> Self {
        let entries = Arc::new(Mutex::new(Vec::<LockEntry>::new()));
        let wake = Arc::new(Condvar::new());
        let stop = Arc::new(AtomicBool::new(false));
        let error = Arc::new(Mutex::new(None::<String>));

        let thread_entries = Arc::clone(&entries);
        let thread_wake = Arc::clone(&wake);
        let thread_stop = Arc::clone(&stop);
        let thread_error = Arc::clone(&error);
        let thread = thread::spawn(move || {
            freeze_loop(
                memory,
                thread_entries,
                thread_wake,
                thread_stop,
                thread_error,
            );
        });

        Self {
            memory,
            entries,
            wake,
            stop,
            error,
            thread: Some(thread),
        }
    }

    pub fn lock(&self, field_offset: u32, value: i32) -> Result<()> {
        // 先登记再立即写一次。写入失败时撤销登记，避免后台线程反复报错。
        {
            let mut entries = lock_entries(&self.entries);
            if let Some(entry) = entries
                .iter_mut()
                .find(|entry| entry.field_offset == field_offset)
            {
                entry.value = value;
            } else {
                entries.push(LockEntry {
                    field_offset,
                    value,
                });
            }
        }
        self.wake.notify_one();

        let result = self.memory.write_offset(field_offset, value);
        if result.is_err() {
            self.unlock(field_offset);
        }
        result
    }

    pub fn write_value(&self, field_offset: u32, value: i32) -> Result<()> {
        // 如果字段已被冻结，同步更新锁定值，避免菜单写入被后台线程覆盖。
        let mut entries = lock_entries(&self.entries);
        self.memory.write_offset(field_offset, value)?;
        if let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.field_offset == field_offset)
        {
            entry.value = value;
        }
        Ok(())
    }

    pub fn unlock(&self, field_offset: u32) {
        let mut entries = lock_entries(&self.entries);
        entries.retain(|entry| entry.field_offset != field_offset);
    }

    pub fn unlock_all(&self) {
        lock_entries(&self.entries).clear();
    }

    pub fn take_error(&self) -> Option<String> {
        lock_error(&self.error).take()
    }
}

fn freeze_loop(
    memory: GameMemory,
    entries: Arc<Mutex<Vec<LockEntry>>>,
    wake: Arc<Condvar>,
    stop: Arc<AtomicBool>,
    error: Arc<Mutex<Option<String>>>,
) {
    let mut high_resolution = false;

    loop {
        // 没有锁定时阻塞在条件变量上。恢复高精度计时器，避免空闲时浪费电量。
        let mut entries = lock_entries(&entries);
        while entries.is_empty() {
            if stop.load(Ordering::Relaxed) {
                end_high_resolution(&mut high_resolution);
                return;
            }
            end_high_resolution(&mut high_resolution);
            entries = wake
                .wait(entries)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }

        if stop.load(Ordering::Relaxed) {
            break;
        }
        if !high_resolution {
            // SAFETY: 只提升当前进程的计时器精度。空闲或退出时成对还原。
            unsafe {
                timeBeginPeriod(1);
            }
            high_resolution = true;
        }

        let mut failed = None;
        entries.retain(
            |entry| match memory.write_offset(entry.field_offset, entry.value) {
                Ok(()) => true,
                Err(write_error) => {
                    if failed.is_none() {
                        failed = Some(write_error.to_string());
                    }
                    false
                }
            },
        );
        if let Some(message) = failed {
            // 只保留第一条错误，避免错误弹窗被后续失败反复触发。
            let mut slot = lock_error(&error);
            if slot.is_none() {
                *slot = Some(message);
            }
        }

        drop(entries);
        thread::sleep(Duration::from_millis(2));
    }

    end_high_resolution(&mut high_resolution);
}

fn end_high_resolution(high_resolution: &mut bool) {
    if *high_resolution {
        // SAFETY: 与同一线程中的 timeBeginPeriod(1) 配对。
        unsafe {
            timeEndPeriod(1);
        }
        *high_resolution = false;
    }
}

fn lock_entries(entries: &Mutex<Vec<LockEntry>>) -> MutexGuard<'_, Vec<LockEntry>> {
    entries
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn lock_error(error: &Mutex<Option<String>>) -> MutexGuard<'_, Option<String>> {
    error
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl Drop for FreezeService {
    fn drop(&mut self) {
        // 通知线程退出并等待它结束，确保进程句柄关闭前不再写内存。
        self.stop.store(true, Ordering::Relaxed);
        self.wake.notify_all();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
