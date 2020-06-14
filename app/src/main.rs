#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;
extern crate nb;
extern crate stm32f4 as stm32;
extern crate stm32f407_hal as hal;

use cortex_m_rt::entry;
use nb::block;

pub use hal::{gpio::*, prelude::*, rcc::*, time::*, timer::*, delay::*, serial::*};

#[entry]
fn main() -> ! {
    let dp = stm32::stm32f407::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

    // Create a timer based on SysTick

    let mut delay = Delay::new(cp.SYST, clocks);


    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();
    let tx = gpioa.pa9.into_alternate_af7();
    let rx = gpioa.pa10.into_alternate_af7();
    // let serial_1 = Serial::usart1(
    //     dp.USART1,
    //     (tx, rx),
    //     Config::default().baudrate(115_200.bps()),
    //     clocks,
    // )
    // .unwrap();

    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high().unwrap();

      let serial_1 = Serial::usart1(
        dp.USART1,
        (tx, rx),
        config::Config::default().baudrate(115_200.bps()),
        clocks
    )
    .unwrap();

    let (mut tx, mut rx) = serial_1.split();

    let sent = 0x08;

    // The `block!` macro makes an operation block until it finishes
    // NOTE the error type is `!`

    loop {
        block!(tx.write(sent)).ok();

        let received = rx.read();

        if received.is_ok() && received.unwrap() == sent {
            led.set_low().unwrap();
            delay.delay_ms(100u32);
            led.set_high().unwrap();
            delay.delay_ms(100u32);
        } else {
            led.set_high().unwrap();
            delay.delay_ms(1000u32);
        }
    }
}
