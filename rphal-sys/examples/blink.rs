use std::{os::macos::raw::stat, ptr, thread, time::Duration};

use rphal_sys::*;

fn robot() {
    unsafe {
        let mut status = 0;
        let port = HAL_GetPort(0);
        let dio = HAL_InitializeDIOPort(port, 0, ptr::null(), &mut status);
        assert_eq!(status, 0, "unable to init dio");

        let mut on = false;
        loop {
            on = !on;

            HAL_SetDIO(dio, on as HAL_Bool, &mut status);
            thread::sleep(Duration::from_millis(500));
        }

    }
}

fn main() {
    unsafe {
        assert!(HAL_Initialize(500, 0) != 0, "hal failed to init");

        if HAL_HasMain() != 0 {
            thread::spawn(robot);
            HAL_RunMain();
        } else {
            robot();
        }
    }
}
