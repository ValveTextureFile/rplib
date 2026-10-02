use crate::*;
use std::{panic, process, thread, time::Duration};

pub trait Robot: Send + 'static {
    fn robot_init(&mut self) {}
    fn disabled_periodic(&mut self) {}
    fn teleop_periodic(&mut self) {}
    fn auton_periodic(&mut self) {}
}

pub fn run<R: Robot>(make: impl FnOnce() -> R + Send + 'static) -> ! {
    unsafe {
        assert!(HAL_Initialize(500, 0) != 0, "Failed to init HAL");
    };

    let def_hook = panic::take_hook();
    panic::set_hook(Box::new(move |inf| {
        def_hook(inf);
        process::exit(1);
    }));

    unsafe {
        if HAL_HasMain() != 0 {
            thread::spawn(move || robot_loop(make));
            HAL_RunMain();
            process::exit(0);
        } else {
            robot_loop(make);
        }
    }
}

fn robot_loop<R: Robot>(make: impl FnOnce() -> R) -> ! {
    let mut robot = make();
    robot.robot_init();

    unsafe { HAL_ObserveUserProgramStarting() };

    loop {
        unsafe { HAL_RefreshDSData() };

        let mut word: HAL_ControlWord = unsafe { std::mem::zeroed() };
        unsafe { HAL_GetControlWord(&mut word) };

        if word.enabled() == 0 {
            unsafe { HAL_ObserveUserProgramDisabled() };
            robot.disabled_periodic();
        } else if word.autonomous() != 0 {
            unsafe { HAL_ObserveUserProgramAutonomous() };
            robot.auton_periodic();
        } else {
            unsafe { HAL_ObserveUserProgramTeleop() };
            robot.teleop_periodic();
        }

        
    }
}
