use rphal_sys::{
    digital::DigitalOutput, robot::Robot, *
};

struct Blinky {
    led: DigitalOutput,
    on: bool
}

impl Robot for Blinky {
    fn teleop_periodic(&mut self) {
        self.on = !self.on;
        self.led.set(self.on).unwrap();
    }
}

fn main() -> error::HalResult<()> {
    rphal_sys::robot::run(|| Blinky {
        led: DigitalOutput::new(0).unwrap(),
        on: false
    })
}