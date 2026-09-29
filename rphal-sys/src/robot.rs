use std::error::Error;

pub trait Robot: Send + 'static {
    fn robot_init(&mut self) {}
    fn disabled_periodic(&mut self) {}
    fn teleop_periodic(&mut self) {}
    fn auton_periodic(&mut self) {}
}

pub fn run<R: Robot>(make: impl FnOnce() -> R + Send + 'static) -> Result<(), Box<dyn Error>> {
    
    todo!()
}