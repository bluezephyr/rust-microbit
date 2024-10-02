#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut is_on: bool = false;
    loop {
        for _ in 0..200_000 {
            nop();
        }
        is_on = !is_on;

        match is_on {
            true => {
                rprintln!("Led is on!");
            }
            false => {
                rprintln!("Led is off!");
            }
        }
    }
}
