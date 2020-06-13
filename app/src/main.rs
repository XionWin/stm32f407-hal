#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32f4 as stm32;
extern crate stm32f407_hal as hal;

use cortex_m_rt::entry;

pub use hal::{gpio::*, prelude::*, rcc::*, time::*, timer::*, delay::*};

#[entry]
fn main() -> ! {
    let dp = stm32::stm32f407::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let gpioc = dp.GPIOC.split();

    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

    // Create a timer based on SysTick

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high().unwrap();

    loop {
        led.set_low().unwrap();
        delay.delay_ms(1000_u32);
        led.set_high().unwrap();
        delay.delay_ms(1000_u32);
    }
}
