use std::{ops::Not, ptr};

use crate::*;

use super::error::{HalResult, check};

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIOState {
    HIGH = 1,
    LOW = 0,
}

impl DIOState {
    pub fn toggle(&self) -> Self {
        if self == &Self::HIGH {
            Self::LOW
        } else {
            Self::HIGH
        }
    }
}

impl Not for DIOState {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.toggle()
    }
}

//? Im starting to think that there might be a better way to wrap DIO, but this is fine for now

pub struct DigitalOutput {
    handle: HAL_DigitalHandle,
}

impl DigitalOutput {
    pub fn new(channel: i32) -> HalResult<Self> {
        let mut status = 0;
        let handle =
            unsafe { HAL_InitializeDIOPort(HAL_GetPort(channel), 0, ptr::null(), &mut status) };
        check(status)?;
        Ok(Self { handle })
    }

    pub fn set(&self, value: DIOState) -> HalResult<()> {
        let mut status = 0;
        unsafe {
            HAL_SetDIO(self.handle, value as HAL_Bool, &mut status);
        };
        check(status)
    }
}

impl Drop for DigitalOutput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDIOPort(self.handle);
        }
    }
}

pub struct DigitalInput {
    handle: HAL_DigitalHandle,
}

impl DigitalInput {
    pub fn new(channel: i32) -> HalResult<Self> {
        let mut status = 0;
        let handle =
            unsafe { HAL_InitializeDIOPort(HAL_GetPort(channel), 1, ptr::null(), &mut status) };

        check(status)?;
        Ok(Self { handle })
    }

    pub fn recv(&self) -> HalResult<DIOState> {
        let mut status = 0;
        let recvd: HAL_Bool;
        unsafe { recvd = HAL_GetDIO(self.handle, &mut status) };
        check(status)?;
        match recvd {
            1 => Ok(DIOState::HIGH),
            0 => Ok(DIOState::LOW),
            _ => unreachable!(),
        }
    }
}

impl Drop for DigitalInput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDIOPort(self.handle);
        }
    }
}