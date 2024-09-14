#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut x: usize = 0;
    loop {
        x += 1;
        for _ in 0..x {
            nop();
        }
    }
}
