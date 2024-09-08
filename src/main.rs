#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    loop {
        rprintln!("Hello, world!");
        for _ in 0..100_000 {
            nop();
        }
    }
}
