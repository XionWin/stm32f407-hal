use crate::time::Hertz;

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    pub(in crate) hclk: Hertz,
    pub(in crate) pclk1: Hertz,
    pub(in crate) pclk2: Hertz,
    pub(in crate) ppre1: u8,
    pub(in crate) ppre2: u8,
    pub(in crate) sysclk: Hertz,
    pub(in crate) pll48clk: Option<Hertz>,
}

impl Clocks {
    /// Returns the frequency of the AHB1
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    /// Returns the prescaler of the APB1
    pub fn ppre1(&self) -> u8 {
        self.ppre1
    }

    /// Returns the prescaler of the APB2
    pub fn ppre2(&self) -> u8 {
        self.ppre2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns the frequency of the PLL48 clock line
    pub fn pll48clk(&self) -> Option<Hertz> {
        self.pll48clk
    }

    /// Returns true if the PLL48 clock is within USB
    /// specifications. It is required to use the USB functionality.
    pub fn is_pll48clk_valid(&self) -> bool {
        // USB specification allow +-0.25%
        self.pll48clk
            .map(|freq| (48_000_000 - freq.0 as i32).abs() <= 48_000_000 * 25 / 10000)
            .unwrap_or(false)
    }
}