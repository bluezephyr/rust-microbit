#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac as pac;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // Set the pins as output
    p.P0.pin_cnf[21].write(|w| w.dir().output());
    p.P0.pin_cnf[28].write(|w| w.dir().output());

    // Set PIN 21 and clear PIN 28
    p.P0.out.write(|w| w.pin21().bit(true));
    p.P0.out.write(|w| w.pin28().bit(false));

    let mut led_on = true;
    loop {
        p.P0.out.write(|w| w.pin21().bit(led_on));
        for _ in 0..200_000 {
            nop();
        }
        led_on = !led_on;
    }
}
