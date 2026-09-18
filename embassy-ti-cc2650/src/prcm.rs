use ti_cc2650_pac::PRCM::vals;

use crate::driverlib;
use crate::pac;

#[derive(Clone, Copy)]
#[repr(u32)]
enum PowerDomain {
    Rfc = driverlib::PRCM_DOMAIN_RFCORE,
    Serial = driverlib::PRCM_DOMAIN_SERIAL,
    Peripherals = driverlib::PRCM_DOMAIN_PERIPH,
    Vims = driverlib::PRCM_DOMAIN_VIMS,
    Sysbus = driverlib::PRCM_DOMAIN_SYSBUS,
    Cpu = driverlib::PRCM_DOMAIN_CPU,
}

#[derive(Clone, Copy, Default)]
pub struct PowerDomains(u32);

impl PowerDomains {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn rfc(self) -> Self {
        Self(self.0 | PowerDomain::Rfc as u32)
    }

    pub const fn serial(self) -> Self {
        Self(self.0 | PowerDomain::Serial as u32)
    }

    pub const fn peripherals(self) -> Self {
        Self(self.0 | PowerDomain::Peripherals as u32)
    }

    pub const fn vims(self) -> Self {
        Self(self.0 | PowerDomain::Vims as u32)
    }

    pub const fn sysbus(self) -> Self {
        Self(self.0 | PowerDomain::Sysbus as u32)
    }

    pub const fn cpu(self) -> Self {
        Self(self.0 | PowerDomain::Cpu as u32)
    }

    pub const fn all() -> Self {
        Self::empty().rfc().serial().peripherals().sysbus().vims().cpu()
    }
}

impl Into<u32> for PowerDomains {
    fn into(self) -> u32 {
        self.0
    }
}

pub struct Prcm {
    prcm: pac::PRCM::PRCM,
}

impl Prcm {
    pub fn new() -> Self {
        Self { prcm: pac::PRCM }
    }

    #[inline]
    pub fn enable_domains(&self, domains: PowerDomains) {
        unsafe { driverlib::PRCMPowerDomainOn(domains.into()) };
        while !Self::are_enabled(domains) {}
    }

    #[inline]
    pub fn disable_domains(&self, domains: PowerDomains) {
        unsafe { driverlib::PRCMPowerDomainOff(domains.into()) }
    }

    #[inline]
    pub fn are_enabled(domains: PowerDomains) -> bool {
        let status = unsafe { driverlib::PRCMPowerDomainStatus(domains.into()) };
        status & driverlib::PRCM_DOMAIN_POWER_ON != 0
    }

    #[inline]
    pub fn enable_clocks(&self, clocks: Clocks) {
        Clock::enable_clocks(&self.prcm, clocks);
    }

    #[inline]
    pub fn rfc_modesel_configure(&self) {
        self.prcm.RFCMODESEL().write(|w| w.set_CURR(vals::CURR::MODE5));
    }
}

#[derive(Clone, Copy, Default)]
pub struct Clocks {
    gpio: bool,
    uart: bool,
    gpt: bool,
    dma: bool,
    crypto: bool,
    rfc: bool,
    i2c: bool,
}

impl Clocks {
    pub const fn empty() -> Self {
        Self {
            gpio: false,
            uart: false,
            gpt: false,
            dma: false,
            crypto: false,
            rfc: false,
            i2c: false,
        }
    }

    pub const fn gpio(self) -> Self {
        Self { gpio: true, ..self }
    }

    pub const fn uart(self) -> Self {
        Self { uart: true, ..self }
    }

    pub const fn gpt(self) -> Self {
        Self { gpt: true, ..self }
    }

    pub const fn dma(self) -> Self {
        Self { dma: true, ..self }
    }

    pub const fn crypto(self) -> Self {
        Self { crypto: true, ..self }
    }

    pub const fn rfc(self) -> Self {
        Self { rfc: true, ..self }
    }

    pub const fn i2c(self) -> Self {
        Self { i2c: true, ..self }
    }
}

pub(crate) struct Clock;

impl Clock {
    pub(crate) fn enable_clocks(prcm: &pac::PRCM::PRCM, clocks: Clocks) {
        if clocks.gpio {
            prcm.GPIOCLKGR().write(|w| w.set_CLK_EN(true));
            prcm.GPIOCLKGS().write(|w| w.set_CLK_EN(true));
            prcm.GPIOCLKGDS().write(|w| w.set_CLK_EN(true));
        }
        if clocks.uart {
            prcm.UARTCLKGR().write(|w| w.set_CLK_EN(true));
            prcm.UARTCLKGS().write(|w| w.set_CLK_EN(true));
            prcm.UARTCLKGDS().write(|w| w.set_CLK_EN(true));
        }
        if clocks.gpt {
            prcm.GPTCLKGR().write(|w| w.set_CLK_EN(vals::GPTCLKGR_CLK_EN::GPT0));
            prcm.GPTCLKGS().write(|w| w.set_CLK_EN(vals::GPTCLKGS_CLK_EN::GPT0));
            prcm.GPTCLKGDS().write(|w| w.set_CLK_EN(vals::GPTCLKGDS_CLK_EN::GPT0));
        }
        if clocks.dma || clocks.crypto {
            prcm.SECDMACLKGR().write(|w| {
                w.set_DMA_CLK_EN(clocks.dma);
                w.set_CRYPTO_CLK_EN(clocks.crypto);
            });
            prcm.SECDMACLKGS().write(|w| {
                w.set_DMA_CLK_EN(clocks.dma);
                w.set_CRYPTO_CLK_EN(clocks.crypto);
            });
            prcm.SECDMACLKGDS().write(|w| {
                w.set_DMA_CLK_EN(clocks.dma);
                w.set_CRYPTO_CLK_EN(clocks.crypto);
            });
        }

        if clocks.rfc {
            prcm.RFCCLKG().write(|w| w.set_CLK_EN(true));
        }
        if clocks.i2c {
            prcm.I2CCLKGR().write(|w| w.set_CLK_EN(true));
            // prcm.i2cclkgs.write(|w| w.clk_en().set_bit());
            // prcm.i2cclkgds.write(|w| w.clk_en().set_bit());
        }

        // Load settings into CLKCTRL and wait for LOAD_DONE
        prcm.CLKLOADCTL().modify(|w| w.set_LOAD(true));
        while !prcm.CLKLOADCTL().read().LOAD_DONE() {}
    }

    // TODO: why this feature? This comes from wprzytula code,
    // I will come back to this when I start working on the radio.
    #[cfg(feature = "ieee")]
    pub(crate) fn disable_clocks(prcm: &pac::PRCM::PRCM, clocks: Clocks) {
        if clocks.gpio {
            prcm.GPIOCLKGR().write(|w| w.set_CLK_EN(false));
            prcm.GPIOCLKGS().write(|w| w.set_CLK_EN(false));
            prcm.GPIOCLKGDS().write(|w| w.set_CLK_EN(false));
        }
        if clocks.uart {
            prcm.UARTCLKGR().write(|w| w.set_CLK_EN(false));
            prcm.UARTCLKGS().write(|w| w.set_CLK_EN(false));
            prcm.UARTCLKGDS().write(|w| w.set_CLK_EN(false));
        }
        if clocks.gpt {
            prcm.GPTCLKGR().write(|w| w.set_CLK_EN(vals::GPTCLKGR_CLK_EN::DIS));
            prcm.GPTCLKGS().write(|w| w.set_CLK_EN(vals::GPTCLKGS_CLK_EN::DIS));
            prcm.GPTCLKGDS().write(|w| w.set_CLK_EN(vals::GPTCLKGDS_CLK_EN::DIS));
        }
        if clocks.dma || clocks.crypto {
            prcm.SECDMACLKGR().write(|w| {
                w.set_DMA_CLK_EN(!clocks.dma);
                w.set_CRYPTO_CLK_EN(!clocks.crypto);
            });
            prcm.SECDMACLKGS().write(|w| {
                w.set_DMA_CLK_EN(!clocks.dma);
                w.set_CRYPTO_CLK_EN(!clocks.crypto);
            });
            prcm.SECDMACLKGDS().write(|w| {
                w.set_DMA_CLK_EN(!clocks.dma);
                w.set_CRYPTO_CLK_EN(!clocks.crypto);
            });
        }

        if clocks.rfc {
            prcm.RFCCLKG().write(|w| w.set_CLK_EN(false));
        }
        if clocks.i2c {
            prcm.I2CCLKGR().write(|w| w.set_CLK_EN(false));
            // prcm.i2cclkgs.write(|w| w.clk_en().set_bit());
            // prcm.i2cclkgds.write(|w| w.clk_en().set_bit());
        }

        // Load settings into CLKCTRL and wait for LOAD_DONE
        prcm.CLKLOADCTL().modify(|w| w.set_LOAD(true));
        while !prcm.CLKLOADCTL().read().LOAD_DONE() {}
    }
}
