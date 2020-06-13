use crate::stm32::{
    TIM1, TIM2, TIM3, TIM4, TIM5, TIM8
};

use crate::gpio::{
    gpioa::*, gpiob::*, gpioc::*, gpiod::*, gpioe::*, gpioh::*, gpioi::*,
    AF1, AF2, AF3,
    Alternate
};

// Output channels marker traits
pub trait PinC1<TIM> {}
pub trait PinC2<TIM> {}
pub trait PinC3<TIM> {}
pub trait PinC4<TIM> {}

macro_rules! channel_impl {
    ( $( $TIM:ident, $PINC:ident, $PINX:ident, $MODE:ident<$AF:ident>; )+ ) => {
        $(
            impl $PINC<$TIM> for $PINX<$MODE<$AF>> {}
        )+
    };
}

channel_impl!(
    TIM1, PinC1, PE9, Alternate<AF1>;
    TIM1, PinC2, PE11, Alternate<AF1>;
    TIM1, PinC3, PE13, Alternate<AF1>;
    TIM1, PinC4, PE14, Alternate<AF1>;
    
    TIM1, PinC1, PA8, Alternate<AF1>;
    TIM1, PinC2, PA9, Alternate<AF1>;
    TIM1, PinC3, PA10, Alternate<AF1>;
    TIM1, PinC4, PA11, Alternate<AF1>;

    TIM2, PinC1, PA0, Alternate<AF1>;
    TIM2, PinC2, PA1, Alternate<AF1>;
    TIM2, PinC3, PA2, Alternate<AF1>;
    TIM2, PinC4, PA3, Alternate<AF1>;

    TIM2, PinC2, PB3, Alternate<AF1>;
    TIM2, PinC3, PB10, Alternate<AF1>;
    TIM2, PinC4, PB11, Alternate<AF1>;

    TIM2, PinC1, PA5, Alternate<AF1>;
    TIM2, PinC1, PA15, Alternate<AF1>;

    TIM3, PinC1, PA6, Alternate<AF2>;
    TIM3, PinC2, PA7, Alternate<AF2>;
    TIM3, PinC3, PB0, Alternate<AF2>;
    TIM3, PinC4, PB1, Alternate<AF2>;

    TIM3, PinC1, PB4, Alternate<AF2>;
    TIM3, PinC2, PB5, Alternate<AF2>;

    TIM3, PinC1, PC6, Alternate<AF2>;
    TIM3, PinC2, PC7, Alternate<AF2>;
    TIM3, PinC3, PC8, Alternate<AF2>;
    TIM3, PinC4, PC9, Alternate<AF2>;

    TIM4, PinC1, PB6, Alternate<AF2>;
    TIM4, PinC2, PB7, Alternate<AF2>;
    TIM4, PinC3, PB8, Alternate<AF2>;
    TIM4, PinC4, PB9, Alternate<AF2>;

    TIM4, PinC1, PD12, Alternate<AF2>;
    TIM4, PinC2, PD13, Alternate<AF2>;
    TIM4, PinC3, PD14, Alternate<AF2>;
    TIM4, PinC4, PD15, Alternate<AF2>;

    TIM5, PinC1, PH10, Alternate<AF2>;
    TIM5, PinC2, PH11, Alternate<AF2>;
    TIM5, PinC3, PH12, Alternate<AF2>;
    TIM5, PinC4, PI0, Alternate<AF2>;
    
    TIM5, PinC1, PA0, Alternate<AF2>;
    TIM5, PinC2, PA1, Alternate<AF2>;
    TIM5, PinC3, PA2, Alternate<AF2>;
    TIM5, PinC4, PA3, Alternate<AF2>;

    TIM8, PinC1, PC6, Alternate<AF3>;
    TIM8, PinC2, PC7, Alternate<AF3>;
    TIM8, PinC3, PC8, Alternate<AF3>;
    TIM8, PinC4, PC9, Alternate<AF3>;
    
    TIM8, PinC1, PI5, Alternate<AF3>;
    TIM8, PinC2, PI6, Alternate<AF3>;
    TIM8, PinC3, PI7, Alternate<AF3>;
    TIM8, PinC4, PI2, Alternate<AF3>;
);
