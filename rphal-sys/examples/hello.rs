use rphal_sys::*;

fn main() {
    unsafe {
        let ok = HAL_Initialize(500, 0);
        assert!(ok != 0, "HAL failed to initalize");

        let mut status = 0;

        let t = HAL_GetFPGATime(&mut status);

        println!("HAL is up, FPGA time is {t} ms (status :: {status}) ");
    }
}