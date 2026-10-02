use std::{error::Error, ffi::CStr, fmt::Display};

use crate::HAL_GetErrorMessage;

#[derive(Debug)]
pub struct HalError {
    pub code: i32,
    pub msg: String,
}

impl Error for HalError {}

impl Display for HalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} :: {}", self.code, self.msg)
    }
}

pub type HalResult<T> = Result<T, HalError>;

pub(crate) fn check(status: i32) -> HalResult<()> {
    if status == 0 {
        return Ok(());
    }

    let msg = unsafe { CStr::from_ptr(HAL_GetErrorMessage(status)) }
        .to_string_lossy()
        .into_owned();
    Err(HalError { code: status, msg })
}
