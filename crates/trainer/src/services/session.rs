//! 一次游戏会话：把内存读写、冻结和变速服务串在一起。

use anyhow::{Result, bail};

use crate::config::feature::FeatureAction;
use crate::config::pointer::{
    BATTLE_SCORE_OFFSET, BATTLE_WINNER_OFFSET, FieldId, HAD_TEA_OFFSET, MenuValues,
    TOOK_BATH_OFFSET,
};
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

    pub fn is_process_alive(&self) -> bool {
        self.process.is_alive()
    }

    pub fn unlock_all(&self) {
        self.freeze.unlock_all();
    }

    pub fn take_freeze_error(&self) -> Option<String> {
        self.freeze.take_error()
    }

    pub fn read_menu_values(&self) -> Option<MenuValues> {
        let mut values = MenuValues::default();
        for field in FieldId::ALL {
            let value = self.memory.read_offset(field.offset()).ok()?;
            values.set(field, value);
        }
        Some(values)
    }

    pub fn write_field(&self, field: FieldId, value: i32) -> Result<()> {
        let (min, max) = field.bounds();
        if !(min..=max).contains(&value) {
            bail!("数值超出范围：{min}..={max}");
        }
        self.freeze.write_value(field.offset(), value)
    }

    pub fn shutdown(&mut self) -> Result<()> {
        // 先停止冻结，避免 speedhack 远程调用超时期间继续写内存。
        self.freeze.unlock_all();
        self.shutdown_done = true;
        self.speedhack.disable()
    }

    pub fn enable_feature(&mut self, action: FeatureAction) -> Result<()> {
        match action {
            FeatureAction::InstantWin => self.freeze.lock(BATTLE_WINNER_OFFSET, 1),
            FeatureAction::Invincibility => self.freeze.lock(BATTLE_SCORE_OFFSET, 114_514),
            FeatureAction::TripleSpeed => self.speedhack.enable(),
            FeatureAction::TimeStop => self.lock_time(),
            FeatureAction::UnlimitedBaths => self.freeze.lock(TOOK_BATH_OFFSET, 0),
            FeatureAction::UnlimitedTea => self.freeze.lock(HAD_TEA_OFFSET, 0),
        }
    }

    pub fn disable_feature(&mut self, action: FeatureAction) -> Result<()> {
        match action {
            FeatureAction::InstantWin => {
                self.freeze.unlock(BATTLE_WINNER_OFFSET);
                Ok(())
            }
            FeatureAction::Invincibility => {
                self.freeze.unlock(BATTLE_SCORE_OFFSET);
                Ok(())
            }
            FeatureAction::TripleSpeed => self.speedhack.disable(),
            FeatureAction::TimeStop => {
                self.freeze.unlock(FieldId::TimeDay.offset());
                self.freeze.unlock(FieldId::TimeHour.offset());
                self.freeze.unlock(FieldId::TimeMinute.offset());
                self.freeze.unlock(FieldId::TimeWeek.offset());
                Ok(())
            }
            FeatureAction::UnlimitedBaths => {
                self.freeze.unlock(TOOK_BATH_OFFSET);
                Ok(())
            }
            FeatureAction::UnlimitedTea => {
                self.freeze.unlock(HAD_TEA_OFFSET);
                Ok(())
            }
        }
    }

    fn lock_time(&self) -> Result<()> {
        // 先读取四项当前值，再逐项加锁。任一项失败就回滚已锁字段，避免只停了一半时间。
        let fields = [
            FieldId::TimeDay,
            FieldId::TimeHour,
            FieldId::TimeMinute,
            FieldId::TimeWeek,
        ];
        let mut locked = [(0u32, 0i32); 4];
        for (slot, field) in locked.iter_mut().zip(fields) {
            let offset = field.offset();
            *slot = (offset, self.memory.read_offset(offset)?);
        }

        for (done, (offset, value)) in locked.into_iter().enumerate() {
            if let Err(error) = self.freeze.lock(offset, value) {
                for (rollback_offset, _) in &locked[..done] {
                    self.freeze.unlock(*rollback_offset);
                }
                return Err(error);
            }
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
