use rphal_sys::{
    robot::Robot,
    types::digital::{DIOState, DigitalOutput},
    *,
};

struct Blinky {
    led: DigitalOutput,
    on: DIOState,
}

impl Robot for Blinky {
    fn teleop_periodic(&mut self) {
        self.on = !self.on;
        self.led.set(self.on).unwrap();
    }
}

fn main() -> types::error::HalResult<()> {
    rphal_sys::robot::run(|| Blinky {
        led: DigitalOutput::new(0).unwrap(),
        on: DIOState::LOW,
    })
}
