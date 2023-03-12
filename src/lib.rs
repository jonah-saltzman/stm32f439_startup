#![no_std]

extern crate panic_itm;

use stm32f4xx_hal::{pac::Peripherals};

pub mod settings;
mod constants;

use settings::{ClockSource, SysTickSource, PLLP, AHBFactor, APBxFactor};
use constants::{pll_p_scale, ahb_scale, apbx_scale};

// input  = HSE || HSI
//          input / M       should be 2MHz
// f(vco) = input * (N / M) must be between 100MHz and 432MHz
// f(pll) = f(vco) / P      must be less than 180MHz
// f(usb) = f(vco) / Q      must be 48MHz
pub struct ClockInit
{
    pub pll_source_hse: Option<bool>, // true: HSE, false: HSI, None: PLL off
    pub sys_source: ClockSource, // default: HSI (0)
    pub systick_source: SysTickSource,
    pub timpre: bool,

    pub pll_q: u8, // min: 2 default: 4 (0100)
    pub pll_p: PLLP, // default: 2 (00)
    pub pll_n: u16, // min: 50 max: 432 default: 192
    pub pll_m: u8, // default: 16 (SHOULD BE 4 with HSE bypass)

    pub ahb_pre: AHBFactor,
    pub apb2_pre: APBxFactor,
    pub apb1_pre: APBxFactor,

    pub ahb1_enr: u32,
    pub ahb2_enr: u32,
    pub ahb3_enr: u32,
    pub apb2_enr: u32,
    pub apb1_enr: u32,

}

pub struct ClockSpeeds
{
    pub pll_input: u32,
    pub pll_output: u32,
    pub sysclk: u32,
    pub hclk: u32,
    pub pclk2: u32,
    pub pclk1: u32,
    pub tim2clk: u32,
    pub tim1clk: u32,
    pub systickclk: u32,
}

pub fn init(init: ClockInit, periphs: &mut Peripherals) -> ClockSpeeds {
    let frequencies = enable_clocks(init, periphs);
    enable_uart3(periphs);
    frequencies
}

fn enable_uart3(periphs: &Peripherals) -> () {
    periphs.GPIOD.moder.write(|w| w.moder8().alternate().moder9().alternate());
    periphs.GPIOD.afrh.write(|w| w.afrh8().af7().afrh9().af7());
    periphs.USART3.cr1.modify(|_, w| w.ue().enabled());
    periphs.USART3.brr.write(|w| unsafe { w.bits(0x15B) });
    periphs.USART3.cr1.modify(|_, w| w.te().enabled());
}

fn enable_clocks(init: ClockInit, periphs: &Peripherals) -> ClockSpeeds {
    let frequencies = calculate_clockspeeds(&init);
    if frequencies.hclk > MILLION!(168) { panic!("HCLK too high. Enable overdrive") };
    periphs.RCC.apb1enr.modify(|_, w| w.pwren().enabled());
    let vos = match frequencies.hclk {
        0        ..=120000000 => 0b01,
        120000001..=144000000 => 0b10,
        144000001..=168000000 => 0b11,
        _ => panic!("Invalid HCLK frequency")
    };
    periphs.PWR.cr.modify(|_, w| unsafe { w.vos().bits(vos) });

    osc_config(&init, periphs);

    let flash_latency = (frequencies.hclk / MILLION!(30)) as u8;
    if flash_latency > 5 { panic!("Invalid flash latency") }

    // check latency
    periphs.FLASH.acr.modify(|r, w| {
        if r.latency().bits() < flash_latency {
            w.latency().bits(flash_latency)
        } else {
            w
        }
    });

    // first set PCLKs to min
    periphs.RCC.cfgr.modify(|_, w| w.ppre2().div16().ppre1().div16());

    // then set HCLK
    periphs.RCC.cfgr.modify(|_, w| w.hpre().variant(init.ahb_pre.into()));

    // set clock source
    periphs.RCC.cfgr.modify(|_, w| w.sw().variant(init.sys_source.into()));

    // wait for clock source to switch
    while periphs.RCC.cfgr.read().sws().variant() != init.sys_source.into() {}

    // then set PCLKs
    periphs.RCC.cfgr.modify(|_, w| w
        .ppre2().variant(init.apb2_pre.into())
        .ppre1().variant(init.apb1_pre.into())
    );

    // re-check latency
    periphs.FLASH.acr.modify(|r, w| {
        if r.latency().bits() > flash_latency {
            w.latency().bits(flash_latency)
        } else {
            w
        }
    });

    // enable peripherals
    periphs.RCC.ahb1enr.write(|w| unsafe { w.bits(init.ahb1_enr) });
    periphs.RCC.ahb2enr.write(|w| unsafe { w.bits(init.ahb2_enr) });
    periphs.RCC.ahb3enr.write(|w| unsafe { w.bits(init.ahb3_enr) });
    periphs.RCC.apb2enr.write(|w| unsafe { w.bits(init.apb2_enr) });
    periphs.RCC.apb1enr.write(|w| unsafe { w.bits(init.apb1_enr) });

    // set TIMPRE bit
    periphs.RCC.dckcfgr.write(|w| w.timpre().bit(init.timpre));

    // set SYSTICK source
    periphs.STK.ctrl.write(|w| 
        if init.systick_source == SysTickSource::HclkDiv1 { 
            w.clksource().set_bit() 
        } 
        else { 
            w.clksource().clear_bit() 
        }
    );

    frequencies
}

fn calculate_clockspeeds(init: &ClockInit) -> ClockSpeeds {
    use constants::{HSI_FREQ, HSE_FREQ};

    let pll_base = if init.pll_source_hse == Some(true) { HSE_FREQ } else { HSI_FREQ };
    let pll_input = pll_base / init.pll_m as u32;
    let pll_vco = pll_input * init.pll_n as u32;
    let pll_output = pll_vco / pll_p_scale(init.pll_p);

    if pll_vco < MILLION!(100) || pll_vco > MILLION!(432) { panic!("Invalid PLL VCO frequency") };
    if pll_input != MILLION!(2) { panic!("Invalid PLL input frequency") };
    if pll_output > MILLION!(180) { panic!("PLL main frequency too high") };

    let sysclk = match init.sys_source {
        ClockSource::Hsi => MILLION!(16),
        ClockSource::Hse => MILLION!(8),
        ClockSource::Pll => pll_output,
    };

    let hclk = sysclk / ahb_scale(init.ahb_pre);
    if hclk > MILLION!(180) { panic!("HCLK frequency too high") };
    let apb2pre = apbx_scale(init.apb2_pre);
    let apb1pre = apbx_scale(init.apb1_pre);
    let pclk2 = hclk / apb2pre;
    let pclk1 = hclk / apb1pre;
    if pclk2 > MILLION!(90) || pclk1 > MILLION!(45) { panic!("Invalid peripheral clock frequency") };
    let (tim2clk, tim1clk) = if init.timpre {
        (
            if apb2pre <= 4 { hclk } else { pclk2 * 4 },
            if apb1pre <= 4 { hclk } else { pclk1 * 4 }
        )
    } else {
        (
            if apb2pre == 1 { pclk2 } else { pclk2 * 2},
            if apb1pre == 1 { pclk1 } else { pclk1 * 2}
        )
    };
    let systickclk = if init.systick_source == SysTickSource::HclkDiv1 { hclk } else { hclk / 8 };
    ClockSpeeds { pll_input, pll_output, sysclk, hclk, pclk2, pclk1, tim2clk, tim1clk, systickclk }
}

fn osc_config(init: &ClockInit, periphs: &Peripherals) -> () {

    if init.sys_source == ClockSource::Hse || init.pll_source_hse == Some(true) {
        // enable HSE with bypass
        periphs.RCC.cr.modify(|_, w| w.hsebyp().bypassed().hseon().on());
        while periphs.RCC.cr.read().hserdy().is_not_ready() {};
    } else {
        periphs.RCC.cr.modify(|_, w| w.hseon().off());
        while periphs.RCC.cr.read().hserdy().is_ready() {};
    }

    if init.sys_source == ClockSource::Hsi || init.pll_source_hse == Some(false) {
        periphs.RCC.cr.modify(|_, w| w.hsion().on());
        while periphs.RCC.cr.read().hsirdy().is_not_ready() {};
    }

    // disable PLL
    periphs.RCC.cr.modify(|_, w| w.pllon().off());
    while periphs.RCC.cr.read().pllrdy().is_ready() {};

    if let Some(pll_src) = init.pll_source_hse {

        // write reg
        periphs.RCC.pllcfgr.write(|w| unsafe { w
            .pllsrc().bit(pll_src)
            .pllq().bits(init.pll_q)
            .pllp().bits(init.pll_p.into())
            .plln().bits(init.pll_n)
            .pllm().bits(init.pll_m)
        });

        // enable PLL
        periphs.RCC.cr.modify(|_, w| w.pllon().on());
        while periphs.RCC.cr.read().pllrdy().is_not_ready() {};
    }
}