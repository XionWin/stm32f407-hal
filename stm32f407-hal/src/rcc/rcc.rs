use crate::rcc::CFGR;

/// Constrained RCC peripheral
pub struct Rcc {
    pub cfgr: CFGR,
}
