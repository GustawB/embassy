use crate::driverlib;

#[derive(Clone, Copy)]
#[repr(u32)]
enum PowerDomain {
    Rfc = driverlib::PRCM_DOMAIN_RFCORE,
    Serial = driverlib::PRCM_DOMAIN_SERIAL,
    Peripherals = driverlib::PRCM_DOMAIN_PERIPH,
    Sysbus = driverlib::PRCM_DOMAIN_SYSBUS,
    Vims = driverlib::PRCM_DOMAIN_VIMS,
    Cpu = driverlib::PRCM_DOMAIN_CPU,
    Timer = driverlib::PRCM_DOMAIN_TIMER,
    Clkctrl = driverlib::PRCM_DOMAIN_CLKCTRL,
    Mcu = driverlib::PRCM_DOMAIN_MCU,
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

    pub const fn sysbus(&self) -> Self {
        Self(self.0 | PowerDomain::Sysbus as u32)
    }

    pub const fn vims(self) -> Self {
        Self(self.0 | PowerDomain::Vims as u32)
    }

    pub const fn cpu(self) -> Self {
        Self(self.0 | PowerDomain::Cpu as u32)
    }

    pub const fn timer(self) -> Self {
        Self(self.0 | PowerDomain::Timer as u32)
    }

    pub const fn clkctrl(self) -> Self {
        Self(self.0 | PowerDomain::Clkctrl as u32)
    }

    pub const fn mcu(self) -> Self {
        Self(self.0 | PowerDomain::Mcu as u32)
    }

    pub const fn all() -> Self {
        Self::empty()
            .rfc()
            .serial()
            .sysbus()
            .peripherals()
            .vims()
            .cpu()
            .timer()
            .clkctrl()
            .mcu()
    }
}

impl Into<u32> for PowerDomains {
    fn into(self) -> u32 {
        self.0
    }
}

pub struct Prcm {
    prcm: cc2650::PRCM,
}

impl Prcm {
    pub fn new(prcm: cc2650::PRCM) -> Self {
        Self { prcm }
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
        self.prcm.rfcmodesel.write(|w| w.curr().mode5());
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
    fn reload_clock_controller(clkloadctl: &cc2650::prcm::CLKLOADCTL) {
        // Unfortunately, static inline fns.
        // driverlib::PRCMLoadSet();
        // while !driverlib::PRCMLoadGet() {}

        // Load settings into CLKCTRL and wait for LOAD_DONE
        clkloadctl.modify(|_r, w| w.load().set_bit());
        loop {
            if clkloadctl.read().load_done().bit_is_set() {
                break;
            }
        }
    }

    pub(crate) fn enable_clocks(prcm: &cc2650::prcm::RegisterBlock, clocks: Clocks) {
        if clocks.gpio {
            prcm.gpioclkgr.write(|w| w.clk_en().set_bit());
            prcm.gpioclkgs.write(|w| w.clk_en().set_bit());
            prcm.gpioclkgds.write(|w| w.clk_en().set_bit());
        }
        if clocks.uart {
            prcm.uartclkgr.write(|w| w.clk_en().set_bit());
            prcm.uartclkgs.write(|w| w.clk_en().set_bit());
            prcm.uartclkgds.write(|w| w.clk_en().set_bit());
        }
        if clocks.gpt {
            prcm.gptclkgr.write(|w| w.clk_en().gpt0());
            prcm.gptclkgs.write(|w| w.clk_en().gpt0());
            prcm.gptclkgds.write(|w| w.clk_en().gpt0());
        }
        if clocks.dma || clocks.crypto {
            prcm.secdmaclkgr
                .write(|w| w.dma_clk_en().bit(clocks.dma).crypto_clk_en().bit(clocks.crypto));
            prcm.secdmaclkgs
                .write(|w| w.dma_clk_en().bit(clocks.dma).crypto_clk_en().bit(clocks.crypto));
            prcm.secdmaclkgds
                .write(|w| w.dma_clk_en().bit(clocks.dma).crypto_clk_en().bit(clocks.crypto));
        }

        if clocks.rfc {
            prcm.rfcclkg.write(|w| w.clk_en().set_bit());
        }
        if clocks.i2c {
            prcm.i2cclkgr.write(|w| w.clk_en().set_bit());
            // prcm.i2cclkgs.write(|w| w.clk_en().set_bit());
            // prcm.i2cclkgds.write(|w| w.clk_en().set_bit());
        }

        Self::reload_clock_controller(&prcm.clkloadctl);
    }

    #[cfg(feature = "ieee")]
    pub(crate) fn disable_clocks(prcm: &cc2650::prcm::RegisterBlock, clocks: Clocks) {
        if clocks.gpio {
            prcm.gpioclkgr.write(|w| w.clk_en().clear_bit());
            prcm.gpioclkgs.write(|w| w.clk_en().clear_bit());
            prcm.gpioclkgds.write(|w| w.clk_en().clear_bit());
        }
        if clocks.uart {
            prcm.uartclkgr.write(|w| w.clk_en().clear_bit());
            prcm.uartclkgs.write(|w| w.clk_en().clear_bit());
            prcm.uartclkgds.write(|w| w.clk_en().clear_bit());
        }
        if clocks.gpt {
            prcm.gptclkgr.write(|w| w);
            prcm.gptclkgs.write(|w| w);
            prcm.gptclkgds.write(|w| w);
        }
        if clocks.dma || clocks.crypto {
            prcm.secdmaclkgr
                .write(|w| w.dma_clk_en().bit(!clocks.dma).crypto_clk_en().bit(!clocks.crypto));
            prcm.secdmaclkgs
                .write(|w| w.dma_clk_en().bit(!clocks.dma).crypto_clk_en().bit(!clocks.crypto));
            prcm.secdmaclkgds
                .write(|w| w.dma_clk_en().bit(!clocks.dma).crypto_clk_en().bit(!clocks.crypto));
        }

        if clocks.rfc {
            prcm.rfcclkg.write(|w| w.clk_en().clear_bit());
        }
        if clocks.i2c {
            prcm.i2cclkgr.write(|w| w.clk_en().clear_bit());
            // prcm.i2cclkgs.write(|w| w.clk_en().clear_bit());
            // prcm.i2cclkgds.write(|w| w.clk_en().clear_bit());
        }
        Self::reload_clock_controller(&prcm.clkloadctl);
    }
}
