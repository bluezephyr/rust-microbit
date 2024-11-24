#![no_std]
#![no_main]

use nrf52833_hal as hal; // Needed for the linker - can probably be removed

use core::ptr::write_volatile;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Configuration registers for PIN 21 and 28
    const PORT0_CNF_PIN21_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
    const PORT0_CNF_PIN28_ADDR: *mut u32 = 0x5000_0770 as *mut u32;
    const DIR_OUT_POS: u32 = 0;

    unsafe {
        // Configure PIN 21 and 28 as output
        // Need to wrap the call with `unsafe` to tell the borrow checker that we know what we are
        // doing. We also need to use write_volatile so that the calls are not optimized away.
        write_volatile(PORT0_CNF_PIN21_ADDR, (1 as u32) << DIR_OUT_POS);
        write_volatile(PORT0_CNF_PIN28_ADDR, (1 as u32) << DIR_OUT_POS);
    }

    // PIN control register and the position for PIN21 (row 1). We don't need to change PIN 28 (col 1)
    // since it is default set to 0 and we want it to be that.
    const PORT0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const PORT0_OUT_ROW1_POS: u32 = 21;

    let mut led_on = true;
    loop {
        unsafe {
            write_volatile(PORT0_OUT_ADDR, (led_on as u32) << PORT0_OUT_ROW1_POS);
        }
        for _ in 0..100_000 {
            nop();
        }
        led_on = !led_on;
    }
}
