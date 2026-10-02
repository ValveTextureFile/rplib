use crate::*;

pub struct Joystick {
    port: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PovDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl PovDirection {
    fn from_angle(angle: i16) -> Option<Self> {
        if angle < 0 || angle % 45 != 0 {
            return None;
        }
        Some(match (angle / 45) % 8 {
            0 => Self::Up,
            1 => Self::UpRight,
            2 => Self::Right,
            3 => Self::DownRight,
            4 => Self::Down,
            5 => Self::DownLeft,
            6 => Self::Left,
            7 => Self::UpLeft,
            _ => unreachable!(),
        })
    }
}

impl Joystick {
    pub fn new(port: i32) -> Self {
        Self { port }
    }

    pub fn axis(&self, axis: usize) -> f32 {
        let mut axes: HAL_JoystickAxes = unsafe { std::mem::zeroed() };
        unsafe { HAL_GetJoystickAxes(self.port, &mut axes) };

        if axis < axes.count as usize {
            axes.axes[axis]
        } else {
            0.0
        }
    }

    pub fn button(&self, button: u32) -> bool {
        let mut buttons: HAL_JoystickButtons = unsafe { std::mem::zeroed() };
        unsafe { HAL_GetJoystickButtons(self.port, &mut buttons) };
        button >= 1 && button <= buttons.count as u32 && (buttons.buttons >> (button - 1)) & 1 == 1
    }

    pub fn pov(&self, pov: usize) -> Option<PovDirection> {
        let mut povs: HAL_JoystickPOVs = unsafe { std::mem::zeroed() };
        unsafe { HAL_GetJoystickPOVs(self.port, &mut povs) };
        if pov >= povs.count as usize {
            return None;
        }
        PovDirection::from_angle(povs.povs[pov])
    }
}

// exbox

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XboxButton {
    A = 1,
    B,
    X,
    Y,
    LeftBumper,
    RightBumper,
    Back,
    Start,
    LeftStick,
    RightStick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XboxAxis {
    LeftX,
    LeftY, /* pushing left up give negative?? */
    LeftTrigger,
    RightTrigger,
    RightX,
    RightY,
}

pub struct XboxController {
    hid: Joystick,
}

impl XboxController {
    pub fn new(port: i32) -> Self {
        Self {
            hid: Joystick::new(port),
        }
    }

    pub fn button(&self, b: XboxButton) -> bool {
        self.hid.button(b as u32)
    }
    pub fn axis(&self, a: XboxAxis) -> f32 {
        self.hid.axis(a as usize)
    }
    pub fn dpad(&self) -> Option<PovDirection> {
        self.hid.pov(0)
    }
}
