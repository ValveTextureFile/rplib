use rphal_sys::{
    robot::run,
    types::joystick::{XboxAxis, XboxButton, XboxController},
    *,
};
use std::io::Write;

macro_rules! clearscreen {
    () => {
        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().flush().unwrap();
    };
}

struct MockJoystickKB {
    js: XboxController,
    t: u32,
}

impl robot::Robot for MockJoystickKB {
    fn teleop_periodic(&mut self) {
        self.t += 1;

        if self.t % 10 != 0 {
            return;
        }

        clearscreen!();
        println!(
            "LX {:+.2}  LY {:+.2}  A {}  B {}  dpad {:?}",
            self.js.axis(XboxAxis::LeftX),
            self.js.axis(XboxAxis::LeftY),
            self.js.button(XboxButton::A),
            self.js.button(XboxButton::B),
            self.js.dpad(),
        );
    }
}

fn main() {
    run(|| MockJoystickKB {
        js: XboxController::new(0),
        t: 0,
    })
}
