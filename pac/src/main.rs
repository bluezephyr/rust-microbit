#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use hal::{gpio::Level, pac};
use nrf52833_hal as hal;
use panic_halt as _;

const LED_STATES: [(bool, bool); 4] = [(true, false), (true, true), (false, true), (false, false)];

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let col1_input = col1.into_pullup_input();
    col1 = col1_input.into_push_pull_output(Level::Low);
    let mut col2 = port0.p0_11.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);
    let mut row2 = port0.p0_22.into_push_pull_output(Level::Low);

    let mut i: usize = 0;
    loop {
        let row1_led = LED_STATES[i].0;
        let col1_led = LED_STATES[i].1;
        let _ = row1.set_state(PinState::from(row1_led));
        let _ = row2.set_state(PinState::from(!row1_led));
        let _ = col1.set_state(PinState::from(col1_led));
        let _ = col2.set_state(PinState::from(!col1_led));
        for _ in 0..200_000 {
            nop();
        }
        i = (i + 1) % 4;
    }
}
