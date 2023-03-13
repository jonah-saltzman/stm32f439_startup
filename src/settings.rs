use stm32f4xx_hal::pac::rcc::cfgr::{HPRE_A, SW_A, SWS_A, PPRE1_A};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SysTickSource {
    HclkDiv8 = 0,
    HclkDiv1 = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ClockSource {
    Hsi = 0,
    Hse = 1,
    Pll = 2,
}

impl Into<SW_A> for ClockSource {
    #[inline(always)]
    fn into(self) -> SW_A {
        match self {
            ClockSource::Hse => SW_A::Hse,
            ClockSource::Hsi => SW_A::Hsi,
            ClockSource::Pll => SW_A::Pll,
        }
    }
}

impl Into<Option<SWS_A>> for ClockSource {
    #[inline(always)]
    fn into(self) -> Option<SWS_A> {
        match self {
            ClockSource::Hse => Some(SWS_A::Hse),
            ClockSource::Hsi => Some(SWS_A::Hsi),
            ClockSource::Pll => Some(SWS_A::Pll),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLP {
    Two   = 0b00,
    Four  = 0b01,
    Six   = 0b10,
    Eight = 0b11,
}

impl Into<u8> for PLLP {
    #[inline(always)]
    fn into(self) -> u8 {
        match self {
            PLLP::Two => 0b00,
            PLLP::Four => 0b01,
            PLLP::Six => 0b10,
            PLLP::Eight => 0b11
        }
    }
}

impl Into<u32> for PLLP {
    #[inline(always)]
    fn into(self) -> u32 {
        match self {
            PLLP::Two => 0b00,
            PLLP::Four => 0b01,
            PLLP::Six => 0b10,
            PLLP::Eight => 0b11
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AHBFactor {
    One                   = 0b0000,
    Two                   = 0b1000,
    Four                  = 0b1001,
    Eight                 = 0b1010,
    Sixteen               = 0b1011,
    SixtyFour             = 0b1100,
    OneHundredTwentyEight = 0b1101,
    TwoHundredFiftySix    = 0b1110,
    FiveHundredTwelve     = 0b1111,
}

impl Into<HPRE_A> for AHBFactor {
    #[inline(always)]
    fn into(self) -> HPRE_A {
        match self {
            AHBFactor::One                   => HPRE_A::Div1,
            AHBFactor::Two                   => HPRE_A::Div2,
            AHBFactor::Four                  => HPRE_A::Div4,
            AHBFactor::Eight                 => HPRE_A::Div8,
            AHBFactor::Sixteen               => HPRE_A::Div16,
            AHBFactor::SixtyFour             => HPRE_A::Div64,
            AHBFactor::OneHundredTwentyEight => HPRE_A::Div128,
            AHBFactor::TwoHundredFiftySix    => HPRE_A::Div256,
            AHBFactor::FiveHundredTwelve     => HPRE_A::Div512,
        }
    }
}

impl Into<u32> for AHBFactor {
    #[inline(always)]
    fn into(self) -> u32 {
        match self {
            AHBFactor::One                   => 1,
            AHBFactor::Two                   => 2,
            AHBFactor::Four                  => 4,
            AHBFactor::Eight                 => 8,
            AHBFactor::Sixteen               => 16,
            AHBFactor::SixtyFour             => 64,
            AHBFactor::OneHundredTwentyEight => 128,
            AHBFactor::TwoHundredFiftySix    => 256,
            AHBFactor::FiveHundredTwelve     => 512,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum APBxFactor {
    One     = 0b000,
    Two     = 0b100,
    Four    = 0b101,
    Eight   = 0b110,
    Sixteen = 0b111,
}

impl Into<PPRE1_A> for APBxFactor {
    fn into(self) -> PPRE1_A {
        match self {
            APBxFactor::One => PPRE1_A::Div1,
            APBxFactor::Two => PPRE1_A::Div2,
            APBxFactor::Four => PPRE1_A::Div4,
            APBxFactor::Eight => PPRE1_A::Div8,
            APBxFactor::Sixteen => PPRE1_A::Div16
        }
    }
}

impl Into<u32> for APBxFactor {
    #[inline(always)]
    fn into(self) -> u32 {
        match self {
            APBxFactor::One => 1,
            APBxFactor::Two => 2,
            APBxFactor::Four => 4,
            APBxFactor::Eight => 8,
            APBxFactor::Sixteen => 16
        }
    }
}