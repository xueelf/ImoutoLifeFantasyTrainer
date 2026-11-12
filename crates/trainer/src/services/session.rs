//! 一次游戏会话：把内存读写、冻结和变速服务串在一起。

use anyhow::{Context, Result, bail};

use crate::config::feature::FeatureAction;
use crate::config::pointer::{self, PointerChain};
use crate::services::freeze::FreezeService;
use crate::services::memory::GameMemory;
use crate::services::process::GameProcess;
use crate::services::speedhack::SpeedhackService;

pub struct GameSession {
    // 字段按释放顺序排列：speedhack、freeze 必须先于进程句柄停止。
    speedhack: SpeedhackService,
    freeze: FreezeService,
    memory: GameMemory,
    process: GameProcess,
    shutdown_done: bool,
}

impl GameSession {
    /// process 必须已经通过 probe 的代码特征和字段检查。
    pub fn new(process: GameProcess, speedhack_unknown: bool) -> Self {
        let memory = process.memory();
        let freeze = FreezeService::new(memory);
        let speedhack = SpeedhackService::new(&process, speedhack_unknown);

        Self {
            speedhack,
            freeze,
            memory,
            process,
            shutdown_done: false,
        }
    }

    pub fn speedhack_unknown(&self) -> bool {
        self.speedhack.state_unknown()
    }

    pub fn pid(&self) -> u32 {
        self.process.pid
    }

    pub fn take_freeze_error(&self) -> Option<String> {
        self.freeze.take_error()
    }

    pub fn read_menu_values(&self, paths: &[String]) -> Vec<(String, i32)> {
        // 读取失败的字段直接跳过，调用方通过返回数量判断数据是否完整。
        paths
            .iter()
            .filter_map(|path| {
                let chain = pointer::lookup(path)?;
                let value = self.memory.read_chain(chain).ok()?;
                Some((path.clone(), value))
            })
            .collect()
    }

    pub fn write_chain(&self, path: &str, value: i32) -> Result<()> {
        // 先按指针表的安全范围校验，再写进游戏。
        let (min, max) = pointer::bounds(path);
        if !(min..=max).contains(&value) {
            bail!("数值超出范围：{min}..={max}");
        }
        let chain = pointer::lookup(path).with_context(|| format!("未知指针：{path}"))?;
        self.freeze.write_value(chain.field_offset, value)
    }

    pub fn shutdown(&mut self) -> Result<()> {
        // 先停止冻结，避免 speedhack 远程调用超时期间继续写内存。
        self.freeze.unlock_all();
        let result = self.speedhack.disable();
        self.shutdown_done = result.is_ok();
        result
    }

    pub fn enable_feature(&mut self, action: FeatureAction) -> Result<()> {
        match action {
            FeatureAction::InstantWin => self.lock(&pointer::BATTLE_WINNER, 1),
            FeatureAction::Invincibility => self.lock(&pointer::BATTLE_SCORE, 114_514),
            FeatureAction::TripleSpeed => self.speedhack.enable(),
            FeatureAction::TimeStop => self.lock_time(),
            FeatureAction::UnlimitedBaths => self.lock(&pointer::TOOK_BATH, 0),
            FeatureAction::UnlimitedTea => self.lock(&pointer::HAD_TEA, 0),
        }
    }

    pub fn disable_feature(&mut self, action: FeatureAction) -> Result<()> {
        match action {
            FeatureAction::InstantWin => {
                self.unlock(&pointer::BATTLE_WINNER);
                Ok(())
            }
            FeatureAction::Invincibility => {
                self.unlock(&pointer::BATTLE_SCORE);
                Ok(())
            }
            FeatureAction::TripleSpeed => self.speedhack.disable(),
            FeatureAction::TimeStop => {
                self.unlock(&pointer::TIME_DAY);
                self.unlock(&pointer::TIME_HOUR);
                self.unlock(&pointer::TIME_MINUTE);
                self.unlock(&pointer::TIME_WEEK);
                Ok(())
            }
            FeatureAction::UnlimitedBaths => {
                self.unlock(&pointer::TOOK_BATH);
                Ok(())
            }
            FeatureAction::UnlimitedTea => {
                self.unlock(&pointer::HAD_TEA);
                Ok(())
            }
        }
    }

    fn lock(&self, chain: &PointerChain, value: i32) -> Result<()> {
        self.freeze.lock(chain.field_offset, value)
    }

    fn unlock(&self, chain: &PointerChain) {
        self.freeze.unlock(chain.field_offset);
    }

    fn lock_time(&self) -> Result<()> {
        // 先读取四项当前值，再逐项加锁。任一项失败就回滚已锁字段，避免只停了一半时间。
        let locked = [
            (
                &pointer::TIME_DAY,
                self.memory.read_chain(&pointer::TIME_DAY)?,
            ),
            (
                &pointer::TIME_HOUR,
                self.memory.read_chain(&pointer::TIME_HOUR)?,
            ),
            (
                &pointer::TIME_MINUTE,
                self.memory.read_chain(&pointer::TIME_MINUTE)?,
            ),
            (
                &pointer::TIME_WEEK,
                self.memory.read_chain(&pointer::TIME_WEEK)?,
            ),
        ];

        let mut done: Vec<&PointerChain> = Vec::new();
        for (chain, value) in locked {
            if let Err(error) = self.lock(chain, value) {
                for chain in done {
                    self.unlock(chain);
                }
                return Err(error);
            }
            done.push(chain);
        }
        Ok(())
    }
}

impl Drop for GameSession {
    fn drop(&mut self) {
        // shutdown 失败或未调用时，至少尝试恢复速度。冻结总是先解锁。
        self.freeze.unlock_all();
        if !self.shutdown_done {
            let _ = self.speedhack.disable();
        }
    }
}
