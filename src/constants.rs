use crate::{MILLION, settings::{APBxFactor, AHBFactor, PLLP}};

pub const HSI_FREQ: u32 = MILLION!(16);
pub const HSE_FREQ: u32 = MILLION!(8);

#[macro_export]
macro_rules! MILLION {
    ($n:expr) => {
        $n * 1000000u32
    };
}

#[inline(always)]
pub fn apbx_scale(x: APBxFactor) -> u32 {
    match x {
        APBxFactor::One     => 1,
        APBxFactor::Two     => 2,
        APBxFactor::Four    => 4,
        APBxFactor::Eight   => 8,
        APBxFactor::Sixteen => 16,
    }
}

#[inline(always)]
pub fn ahb_scale(x: AHBFactor) -> u32 {
    match x {
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

#[inline(always)]
pub fn pll_p_scale(x: PLLP) -> u32 {
    match x {
        PLLP::Two   => 2,
        PLLP::Four  => 4,
        PLLP::Six   => 6,
        PLLP::Eight => 8,
    }
}