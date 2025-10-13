#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use cortex_m_rt::entry;
// use panic_halt as _; // panic handler - halt
use panic_rtt_target as _; // panic handler - rtt
use stm32f4xx_hal::{self as hal, rcc::Config};

use crate::hal::{pac, prelude::*};
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Hello, world!");
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(48.MHz()));

        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioc = dp.GPIOC.split(&mut rcc);
        let mut led = gpioc.pc13.into_push_pull_output();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&rcc.clocks);

        loop {
            // On for 1s, off for 1s.
            led.toggle();
            delay.delay_ms(500);
            rprintln!("Toggle here!");
        }
    }

    loop {}
}
