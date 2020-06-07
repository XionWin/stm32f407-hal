use core::marker::PhantomData;

pub struct AF0;
pub struct AF1;
pub struct AF2;
pub struct AF3;
pub struct AF4;
pub struct AF5;
pub struct AF6;
pub struct AF7;
pub struct AF8;
pub struct AF9;
pub struct AF10;
pub struct AF11;
pub struct AF12;
pub struct AF13;
pub struct AF14;
pub struct AF15;


/// Some alternate mode (type state)
pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// Some alternate mode in open drain configuration (type state)
pub struct AlternateOD<MODE> {
    _mode: PhantomData<MODE>,
}