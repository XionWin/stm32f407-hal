use cast::{u16, u32};
use nb;
use embedded_hal::timer::{Cancel, CountDown, Periodic};
use void::Void;

use crate::stm32::RCC;

use crate::stm32::{
    TIM1, TIM2, TIM3, TIM4, TIM5,
    TIM6, TIM7, TIM8, TIM9, TIM10,
    TIM11, TIM12, TIM13, TIM14
};

use crate::rcc::Clocks;
use crate::time::Hertz;
use crate::timer::{Timer, Event, Error};

macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $timXen:ident, $timXrst:ident, $apbenr:ident, $apbrstr:ident, $pclk:ident, $ppre:ident),)+) => {
        $(
            impl Timer<$TIM> {
                /// Configures a TIM peripheral as a periodic count down timer
                pub fn $tim<T>(tim: $TIM, timeout: T, clocks: Clocks) -> Self
                where
                    T: Into<Hertz>,
                {
                    // enable and reset peripheral to a clean slate state
                    let rcc = unsafe { &(*RCC::ptr()) };
                    rcc.$apbenr.modify(|_, w| w.$timXen().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().clear_bit());

                    let mut timer = Timer {
                        clocks,
                        tim,
                    };
                    timer.start(timeout);

                    timer
                }

                /// Starts listening for an `event`
                ///
                /// Note, you will also have to enable the TIM2 interrupt in the NVIC to start
                /// receiving events.
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Enable update event interrupt
                            self.tim.dier.write(|w| w.uie().set_bit());
                        }
                    }
                }

                /// Clears interrupt associated with `event`.
                ///
                /// If the interrupt is not cleared, it will immediately retrigger after
                /// the ISR has finished.
                pub fn clear_interrupt(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Clear interrupt flag
                            self.tim.sr.write(|w| w.uif().clear_bit());
                        }
                    }
                }

                /// Stops listening for an `event`
                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::TimeOut => {
                            // Enable update event interrupt
                            self.tim.dier.write(|w| w.uie().clear_bit());
                        }
                    }
                }

                /// Releases the TIM peripheral
                pub fn release(self) -> $TIM {
                    // pause counter
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    self.tim
                }
            }

            impl CountDown for Timer<$TIM> {
                type Time = Hertz;

                fn start<T>(&mut self, timeout: T)
                where
                    T: Into<Hertz>,
                {
                    // pause
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    // reset counter
                    self.tim.cnt.reset();

                    let frequency = timeout.into().0;
                    let pclk_mul = if self.clocks.$ppre() == 1 { 1 } else { 2 };
                    let ticks = self.clocks.$pclk().0 * pclk_mul / frequency;

                    let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                    self.tim.psc.write(|w| w.psc().bits(psc) );

                    let arr = u16(ticks / u32(psc + 1)).unwrap();
                    self.tim.arr.write(|w| unsafe { w.bits(u32(arr)) });

                    // start counter
                    self.tim.cr1.modify(|_, w| w.cen().set_bit());
                }

                fn wait(&mut self) -> nb::Result<(), Void> {
                    if self.tim.sr.read().uif().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.tim.sr.modify(|_, w| w.uif().clear_bit());
                        Ok(())
                    }
                }
            }

            impl Cancel for Timer<$TIM>
            {
                type Error = Error;

                fn cancel(&mut self) -> Result<(), Self::Error> {
                    let is_counter_enabled = self.tim.cr1.read().cen().is_enabled();
                    if !is_counter_enabled {
                        return Err(Self::Error::Disabled);
                    }

                    // disable counter
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    Ok(())
                }
            }

            impl Periodic for Timer<$TIM> {}
        )+
    }
}

hal! {
    TIM1: (tim1, tim1en, tim1rst, apb2enr, apb2rstr, pclk2, ppre2),
    TIM2: (tim2, tim2en, tim2rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM3: (tim3, tim3en, tim3rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM4: (tim4, tim4en, tim4rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM5: (tim5, tim5en, tim5rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM6: (tim6, tim6en, tim6rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM7: (tim7, tim7en, tim7rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM8: (tim8, tim8en, tim8rst, apb2enr, apb2rstr, pclk2, ppre2),
    TIM9: (tim9, tim9en, tim9rst, apb2enr, apb2rstr, pclk2, ppre2),
    TIM10: (tim10, tim10en, tim10rst, apb2enr, apb2rstr, pclk2, ppre2),
    TIM11: (tim11, tim11en, tim11rst, apb2enr, apb2rstr, pclk2, ppre2),
    TIM12: (tim12, tim12en, tim12rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM13: (tim13, tim13en, tim13rst, apb1enr, apb1rstr, pclk1, ppre1),
    TIM14: (tim14, tim14en, tim14rst, apb1enr, apb1rstr, pclk1, ppre1),
}

