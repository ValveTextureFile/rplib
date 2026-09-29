use std::ptr;

use crate::{
    error::{HalResult, check},
    *,
};

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

    pub fn set(&self, value: bool) -> HalResult<()> {
        let mut status = 0;
        unsafe { HAL_SetDIO(self.handle, value as HAL_Bool, &mut status); };
        check(status)
    }
}

impl Drop for DigitalOutput{
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDIOPort(self.handle);
        }
    }
}