//! UDMA support.
//!
//! Notes from TRM:
//! - each channel has 2 priorities: high and low.
//! - `arbitration size` means how many elements are sent before a new channel selection occurs
//! - `burst` vs `single` transfer: burst sends multiple in a batch, it is not interruptible
//!     - in case of UART burst threshold should be configured (e.g. 1/2 * 32) as the same as arbitration size (16)
//!     - `single` can be turned off using UDMA:SETBURST.
//! - apart from powering on and turning on clock gating, the controller must be turned on
//! - DMA generates interrupts for peripherals, so their own interrupt triggers should be turned off
//!   if DMA is in use.

use core::u32;
use core::{ffi::c_void, marker::PhantomData, ptr::addr_of};

use crate::define_peri;
use crate::driverlib;
use crate::pac;
use paste::paste;

const UART0_RX_CHANNEL: u32 = 1;
const UART0_TX_CHANNEL: u32 = 2;

// 1073872896 is the start address of registers for UART0.
// cc2650 crate calls it RegisterBlock; I took this
// addres from said crate.
define_peri!(IUdma, udma0, 1073872896);

macro_rules! static_mut_ref {
    ($static_mut:ident) => {
        (&mut *core::ptr::addr_of_mut!($static_mut))
    };
}

pub(crate) static UDMA: Udma = Udma {};

pub(crate) struct Udma {}

impl Udma {
    #[inline(never)]
    pub(crate) fn enable(&self) {
        // Set the pointer to the channel control map.
        let map_addr = addr_of!(CHANNEL_CONTROL_MAP) as u32;

        // `w.baseptr()` performs shift left 10 bits on your argument,
        // probably because 10 least significant bits on CTRL register
        // are reserved.
        IUDMA.ctrl.write(|w| unsafe { w.bits(map_addr) });

        IUDMA.cfg.write(|w| w.masterenable().set_bit());
    }

    #[inline]
    #[allow(unused)]
    pub(crate) fn disable(&self) {
        IUDMA.cfg.write(|w| w.masterenable().clear_bit());
    }

    // No separate `uart_enable_{tx,rx}` functions because enabling is done
    // only in `uart_transfer_{tx,rx}`.

    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_disable_tx(&self) {
        unsafe {
            driverlib::uDMAChannelDisable(driverlib::UDMA0_BASE, UART0_TX_CHANNEL);
        }
    }

    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_disable_rx(&self) {
        unsafe {
            driverlib::uDMAChannelDisable(driverlib::UDMA0_BASE, UART0_RX_CHANNEL);
        }
    }

    #[inline]
    pub(crate) fn uart_channels_configure(&self) {
        let channel_struct_index_rx = driverlib::UDMA_PRI_SELECT | UART0_RX_CHANNEL;
        // On receive, uDMA repeatedly reads 8 bytes from DR (no incr) and writes it to the
        // destination (increment).
        let channel_control_rx =
            driverlib::UDMA_SIZE_8 | driverlib::UDMA_SRC_INC_NONE | driverlib::UDMA_DST_INC_8 | driverlib::UDMA_ARB_32;
        unsafe {
            driverlib::uDMAChannelControlSet(driverlib::UDMA0_BASE, channel_struct_index_rx, channel_control_rx);
        };

        let channel_struct_index_tx = driverlib::UDMA_PRI_SELECT | UART0_TX_CHANNEL;

        // On send, uDMA repeatedly writes 8 bytes from source (increment) and writes it to the
        // DR (no incr).
        let channel_control_tx =
            driverlib::UDMA_SIZE_8 | driverlib::UDMA_SRC_INC_8 | driverlib::UDMA_DST_INC_NONE | driverlib::UDMA_ARB_32;
        unsafe {
            driverlib::uDMAChannelControlSet(driverlib::UDMA0_BASE, channel_struct_index_tx, channel_control_tx);
        };
    }

    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_transfer_rx(&self, mem: &mut [u8]) {
        unsafe {
            static_mut_ref!(CHANNEL_CONTROL_MAP).primary_channel_1.set_transfer(
                &(*pac::UART0::ptr()).dr as *const pac::uart0::DR as *mut (),
                mem.as_mut_ptr() as *mut (),
                mem.len() as u32,
            );
            driverlib::uDMAChannelEnable(driverlib::UDMA0_BASE, UART0_RX_CHANNEL);
        }
    }

    #[inline]
    pub(crate) fn uart_transfer_tx(&self, mem: &[u8]) {
        unsafe {
            static_mut_ref!(CHANNEL_CONTROL_MAP).primary_channel_2.set_transfer(
                mem.as_ptr() as *mut (),
                &(*pac::UART0::ptr()).dr as *const pac::uart0::DR as *mut (),
                mem.len() as u32,
            );
            driverlib::uDMAChannelEnable(driverlib::UDMA0_BASE, UART0_TX_CHANNEL);
        }
    }

    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_is_enabled_rx(&self) -> bool {
        unsafe { driverlib::uDMAChannelIsEnabled(driverlib::UDMA0_BASE, UART0_RX_CHANNEL) }
    }

    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_is_enabled_tx(&self) -> bool {
        unsafe { driverlib::uDMAChannelIsEnabled(driverlib::UDMA0_BASE, UART0_TX_CHANNEL) }
    }

    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_request_done_rx(&self) -> bool {
        unsafe { static_mut_ref!(CHANNEL_CONTROL_MAP).primary_channel_1.is_request_done() }
    }

    #[inline]
    pub(crate) fn uart_request_done_tx(&self) -> bool {
        unsafe { static_mut_ref!(CHANNEL_CONTROL_MAP).primary_channel_2.is_request_done() }
    }

    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_request_done_rx_mask(&self) {
        unsafe {
            static_mut_ref!(CHANNEL_CONTROL_MAP)
                .primary_channel_1
                .request_done_mask()
        }
    }

    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_request_done_rx_unmask(&self) {
        unsafe {
            static_mut_ref!(CHANNEL_CONTROL_MAP)
                .primary_channel_1
                .request_done_unmask()
        }
    }

    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_request_done_rx_clear(&self) {
        unsafe {
            static_mut_ref!(CHANNEL_CONTROL_MAP)
                .primary_channel_1
                .request_done_clear()
        }
    }

    #[inline]
    pub(crate) fn uart_request_done_tx_mask(&self) {
        unsafe {
            static_mut_ref!(CHANNEL_CONTROL_MAP)
                .primary_channel_2
                .request_done_mask()
        }
    }

    #[inline]
    pub(crate) fn uart_request_done_tx_unmask(&self) {
        unsafe {
            static_mut_ref!(CHANNEL_CONTROL_MAP)
                .primary_channel_2
                .request_done_unmask()
        }
    }

    #[inline]
    pub(crate) fn uart_request_done_tx_clear(&self) {
        unsafe {
            static_mut_ref!(CHANNEL_CONTROL_MAP)
                .primary_channel_2
                .request_done_clear()
        }
    }

    // Safety: use only when uDMA rx disabled.
    #[inline]
    #[allow(unused)]
    pub(crate) fn uart_dest_addr_rx_get(&self) -> u32 {
        unsafe { static_mut_ref!(CHANNEL_CONTROL_MAP).primary_channel_1.dest_end_ptr }
    }
}

mod channel_control_entry_kind {
    pub(super) trait Sealed {}
    pub(super) trait ChannelControlEntryKind: Sealed {}

    pub(super) struct Primary;
    impl Sealed for Primary {}
    impl ChannelControlEntryKind for Primary {}
    #[cfg(feature = "full_udma_table")]
    pub(super) struct Alternate;
    #[cfg(feature = "full_udma_table")]
    impl Sealed for Alternate {}
    #[cfg(feature = "full_udma_table")]
    impl ChannelControlEntryKind for Alternate {}
}
#[cfg(feature = "full_udma_table")]
use channel_control_entry_kind::Alternate;
use channel_control_entry_kind::{ChannelControlEntryKind, Primary};

pub mod control_word {
    use crate::driverlib;

    #[derive(Clone, Copy)]
    pub struct ControlWord {
        pub data_size: DataSize,
        pub src_addr_inc: SrcAddrIncrement,
        pub dst_addr_inc: DstAddrIncrement,
        pub arbitration_size: ArbitrationSize,
    }

    impl ControlWord {
        #[inline]
        pub fn as_u32(&self) -> u32 {
            self.data_size as u32 | self.src_addr_inc as u32 | self.dst_addr_inc as u32 | self.arbitration_size as u32
        }
    }

    #[derive(Clone, Copy)]
    #[repr(u32)]
    pub enum DataSize {
        Size8 = driverlib::UDMA_SIZE_8,
        Size16 = driverlib::UDMA_SIZE_16,
        Size32 = driverlib::UDMA_SIZE_32,
    }

    #[derive(Clone, Copy)]
    #[repr(u32)]
    pub enum SrcAddrIncrement {
        Inc8 = driverlib::UDMA_SRC_INC_8,
        Inc16 = driverlib::UDMA_SRC_INC_16,
        Inc32 = driverlib::UDMA_SRC_INC_32,
        IncNone = driverlib::UDMA_SRC_INC_NONE,
    }

    #[derive(Clone, Copy)]
    #[repr(u32)]
    pub enum DstAddrIncrement {
        Inc8 = driverlib::UDMA_DST_INC_8,
        Inc16 = driverlib::UDMA_DST_INC_16,
        Inc32 = driverlib::UDMA_DST_INC_32,
        IncNone = driverlib::UDMA_DST_INC_NONE,
    }

    #[derive(Clone, Copy)]
    #[repr(u32)]
    pub enum ArbitrationSize {
        Arb1 = driverlib::UDMA_ARB_1,
        Arb2 = driverlib::UDMA_ARB_2,
        Arb4 = driverlib::UDMA_ARB_4,
        Arb8 = driverlib::UDMA_ARB_8,
        Arb16 = driverlib::UDMA_ARB_16,
        Arb32 = driverlib::UDMA_ARB_32,
        Arb64 = driverlib::UDMA_ARB_64,
        Arb128 = driverlib::UDMA_ARB_128,
        Arb256 = driverlib::UDMA_ARB_256,
        Arb512 = driverlib::UDMA_ARB_512,
        Arb1024 = driverlib::UDMA_ARB_1024,
    }
}
pub use control_word::ControlWord;

#[repr(C, align(16))]
struct ChannelControlEntry<KIND: ChannelControlEntryKind, const INDEX: u32> {
    src_end_ptr: u32,
    dest_end_ptr: u32,
    control_word: u32,
    _unused: u32,

    _phantom: PhantomData<KIND>,
}

impl<const INDEX: u32> ChannelControlEntry<Primary, INDEX> {
    #[allow(unused)]
    fn software_request(&self) {
        IUDMA.softreq.write(|w| unsafe { w.chnls().bits(1 << INDEX) })
    }

    fn is_request_done(&self) -> bool {
        IUDMA.reqdone.read().chnls().bits() & (1 << INDEX) != 0
    }

    fn request_done_clear(&self) {
        IUDMA.reqdone.write(|w| unsafe { w.chnls().bits(1 << INDEX) })
    }

    fn request_done_mask(&self) {
        IUDMA
            .donemask
            .modify(|r, w| unsafe { w.chnls().bits(r.chnls().bits() | (1 << INDEX)) })
    }

    fn request_done_unmask(&self) {
        IUDMA
            .donemask
            .modify(|r, w| unsafe { w.chnls().bits(r.chnls().bits() & !(1 << INDEX)) })
    }
}

impl<KIND: ChannelControlEntryKind, const INDEX: u32> ChannelControlEntry<KIND, INDEX> {
    const fn new() -> Self {
        Self {
            src_end_ptr: 0,
            dest_end_ptr: 0,
            control_word: 0,
            _unused: 0,
            _phantom: PhantomData,
        }
    }

    fn set_transfer(&self, src: *mut (), dest: *mut (), len: u32) {
        unsafe {
            driverlib::uDMAChannelTransferSet(
                driverlib::UDMA0_BASE,
                INDEX,
                driverlib::UDMA_MODE_BASIC,
                src as *mut c_void,
                dest as *mut c_void,
                len,
            )
        }
    }
}

#[repr(C, align(1024))]
struct ChannelControlMap {
    primary_channel_0: ChannelControlEntry<Primary, 0>, // Software 0
    primary_channel_1: ChannelControlEntry<Primary, 1>, // UART0_RX
    primary_channel_2: ChannelControlEntry<Primary, 2>, // UART0_TX
    #[cfg(feature = "full_udma_table")]
    primary_channel_3: ChannelControlEntry<Primary, 3>, // SSP0_RX
    #[cfg(feature = "full_udma_table")]
    primary_channel_4: ChannelControlEntry<Primary, 4>, // SSP0_TX
    #[cfg(feature = "full_udma_table")]
    primary_channel_5: ChannelControlEntry<Primary, 5>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_6: ChannelControlEntry<Primary, 6>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_7: ChannelControlEntry<Primary, 7>, // AUX_ADC
    #[cfg(feature = "full_udma_table")]
    primary_channel_8: ChannelControlEntry<Primary, 8>, // AUX_SW
    #[cfg(feature = "full_udma_table")]
    primary_channel_9: ChannelControlEntry<Primary, 9>, // GPT0_A
    #[cfg(feature = "full_udma_table")]
    primary_channel_10: ChannelControlEntry<Primary, 10>, // GPT0_B
    #[cfg(feature = "full_udma_table")]
    primary_channel_11: ChannelControlEntry<Primary, 11>, // GPT1_A
    #[cfg(feature = "full_udma_table")]
    primary_channel_12: ChannelControlEntry<Primary, 12>, // GPT1_B
    #[cfg(feature = "full_udma_table")]
    primary_channel_13: ChannelControlEntry<Primary, 13>, // AON_PROG2
    #[cfg(feature = "full_udma_table")]
    primary_channel_14: ChannelControlEntry<Primary, 14>, // DMA_PROG
    #[cfg(feature = "full_udma_table")]
    primary_channel_15: ChannelControlEntry<Primary, 15>, // AON_RTC
    #[cfg(feature = "full_udma_table")]
    primary_channel_16: ChannelControlEntry<Primary, 16>, // SSP1_RX
    #[cfg(feature = "full_udma_table")]
    primary_channel_17: ChannelControlEntry<Primary, 17>, // SSP1_TX
    #[cfg(feature = "full_udma_table")]
    primary_channel_18: ChannelControlEntry<Primary, 18>, // Software 1
    #[cfg(feature = "full_udma_table")]
    primary_channel_19: ChannelControlEntry<Primary, 19>, // Software 2
    #[cfg(feature = "full_udma_table")]
    primary_channel_20: ChannelControlEntry<Primary, 20>, // Software 3
    #[cfg(feature = "full_udma_table")]
    primary_channel_21: ChannelControlEntry<Primary, 21>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_22: ChannelControlEntry<Primary, 22>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_23: ChannelControlEntry<Primary, 23>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_24: ChannelControlEntry<Primary, 24>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_25: ChannelControlEntry<Primary, 25>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_26: ChannelControlEntry<Primary, 26>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_27: ChannelControlEntry<Primary, 27>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_28: ChannelControlEntry<Primary, 28>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_29: ChannelControlEntry<Primary, 29>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_30: ChannelControlEntry<Primary, 30>, // Reserved
    #[cfg(feature = "full_udma_table")]
    primary_channel_31: ChannelControlEntry<Primary, 31>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_0: ChannelControlEntry<Alternate, 32>, // Software 0
    #[cfg(feature = "full_udma_table")]
    alternate_channel_1: ChannelControlEntry<Alternate, 33>, // UART0_RX
    #[cfg(feature = "full_udma_table")]
    alternate_channel_2: ChannelControlEntry<Alternate, 34>, // UART0_TX
    #[cfg(feature = "full_udma_table")]
    alternate_channel_3: ChannelControlEntry<Alternate, 35>, // SSP0_RX
    #[cfg(feature = "full_udma_table")]
    alternate_channel_4: ChannelControlEntry<Alternate, 36>, // SSP0_TX
    #[cfg(feature = "full_udma_table")]
    alternate_channel_5: ChannelControlEntry<Alternate, 37>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_6: ChannelControlEntry<Alternate, 38>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_7: ChannelControlEntry<Alternate, 39>, // AUX_ADC
    #[cfg(feature = "full_udma_table")]
    alternate_channel_8: ChannelControlEntry<Alternate, 40>, // AUX_SW
    #[cfg(feature = "full_udma_table")]
    alternate_channel_9: ChannelControlEntry<Alternate, 41>, // GPT0_A
    #[cfg(feature = "full_udma_table")]
    alternate_channel_10: ChannelControlEntry<Alternate, 42>, // GPT0_B
    #[cfg(feature = "full_udma_table")]
    alternate_channel_11: ChannelControlEntry<Alternate, 43>, // GPT1_A
    #[cfg(feature = "full_udma_table")]
    alternate_channel_12: ChannelControlEntry<Alternate, 44>, // GPT1_B
    #[cfg(feature = "full_udma_table")]
    alternate_channel_13: ChannelControlEntry<Alternate, 45>, // AON_PROG2
    #[cfg(feature = "full_udma_table")]
    alternate_channel_14: ChannelControlEntry<Alternate, 46>, // DMA_PROG
    #[cfg(feature = "full_udma_table")]
    alternate_channel_15: ChannelControlEntry<Alternate, 47>, // AON_RTC
    #[cfg(feature = "full_udma_table")]
    alternate_channel_16: ChannelControlEntry<Alternate, 48>, // SSP1_RX
    #[cfg(feature = "full_udma_table")]
    alternate_channel_17: ChannelControlEntry<Alternate, 49>, // SSP1_TX
    #[cfg(feature = "full_udma_table")]
    alternate_channel_18: ChannelControlEntry<Alternate, 50>, // Software 1
    #[cfg(feature = "full_udma_table")]
    alternate_channel_19: ChannelControlEntry<Alternate, 51>, // Software 2
    #[cfg(feature = "full_udma_table")]
    alternate_channel_20: ChannelControlEntry<Alternate, 52>, // Software 3
    #[cfg(feature = "full_udma_table")]
    alternate_channel_21: ChannelControlEntry<Alternate, 53>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_22: ChannelControlEntry<Alternate, 54>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_23: ChannelControlEntry<Alternate, 55>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_24: ChannelControlEntry<Alternate, 56>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_25: ChannelControlEntry<Alternate, 57>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_26: ChannelControlEntry<Alternate, 58>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_27: ChannelControlEntry<Alternate, 59>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_28: ChannelControlEntry<Alternate, 60>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_29: ChannelControlEntry<Alternate, 61>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_30: ChannelControlEntry<Alternate, 62>, // Reserved
    #[cfg(feature = "full_udma_table")]
    alternate_channel_31: ChannelControlEntry<Alternate, 63>, // Reserved
}

impl ChannelControlMap {}

static mut CHANNEL_CONTROL_MAP: ChannelControlMap = ChannelControlMap {
    primary_channel_0: ChannelControlEntry::new(),
    primary_channel_1: ChannelControlEntry::new(),
    primary_channel_2: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_3: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_4: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_5: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_6: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_7: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_8: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_9: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_10: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_11: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_12: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_13: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_14: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_15: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_16: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_17: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_18: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_19: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_20: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_21: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_22: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_23: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_24: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_25: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_26: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_27: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_28: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_29: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_30: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    primary_channel_31: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_0: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_1: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_2: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_3: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_4: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_5: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_6: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_7: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_8: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_9: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_10: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_11: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_12: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_13: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_14: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_15: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_16: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_17: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_18: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_19: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_20: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_21: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_22: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_23: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_24: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_25: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_26: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_27: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_28: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_29: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_30: ChannelControlEntry::new(),
    #[cfg(feature = "full_udma_table")]
    alternate_channel_31: ChannelControlEntry::new(),
};
