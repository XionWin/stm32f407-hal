#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32f4;
extern crate stm32f407_hal as stm32;
extern crate embedded_hal;
extern crate nb;

use cortex_m_rt::entry;

pub use embedded_hal::prelude::*;
pub use embedded_hal::digital::v2::OutputPin;


pub use stm32::gpio::*;

#[entry]
fn main() -> ! {
    let dp = stm32f4::stm32f407::Peripherals::take().unwrap();

    let gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high().unwrap();

    let delay = 5000000u32;
    loop {
        led.set_low().unwrap();
        cortex_m::asm::delay(delay); 
        led.set_high().unwrap();
        cortex_m::asm::delay(delay); 
    }
}
