use std::ptr;

use crate::{
    types::error::{HalResult, check},
    *,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PwmConfig {
    pub max: i32,
    pub deadband_max: i32,
    pub center: i32,
    pub deadband_min: i32,
    pub min: i32,
}

impl PwmConfig {
    pub const STANDARD: Self = Self {
        max: 2000,
        deadband_max: 1550,
        center: 1500,
        deadband_min: 1450,
        min: 1000,
    };

    pub const TALON: Self = Self::STANDARD;
    pub const VICTOR_SP: Self = Self::STANDARD;
    pub const SPARK: Self = Self::STANDARD;
}

pub struct Pwm {
    handle: HAL_DigitalHandle,
}

impl Pwm {
    pub fn new(channel: i32, config: PwmConfig) -> HalResult<Self> {
        let mut status = 0;
        let handle = unsafe {
            HAL_InitializePWMPort(HAL_GetPort(channel), ptr::null(), &mut status)
        };
        check(status)?;

        let pwm = Self { handle };
        pwm.configure(config)?;
        Ok(pwm)
    }

    pub fn configure(&self, config: PwmConfig) -> HalResult<()> {
        let mut status = 0;
        unsafe {
            HAL_SetPWMConfigMicroseconds(
                self.handle,
                config.max,
                config.deadband_max,
                config.center,
                config.deadband_min,
                config.min,
                &mut status,
            );
        }
        check(status)
    }

    pub fn set(&self, speed: f64) -> HalResult<()> {
        let mut status = 0;
        unsafe {
            HAL_SetPWMSpeed(self.handle, speed, &mut status);
        };
        check(status)
    }
}

impl Drop for Pwm {
    fn drop(&mut self) {
        unsafe {
            HAL_FreePWMPort(self.handle);
        }
    }
}
