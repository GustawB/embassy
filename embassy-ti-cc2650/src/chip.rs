use crate::gpio::impl_pin;
use crate::gpt::impl_gpt;
use crate::rtc::impl_rtc;
use crate::uart::impl_uart;
pub use cc2650 as pac;

embassy_hal_internal::peripherals! {
    UART0,
    AON_RTC,
    GPT0,

    P_00,
    P_01,
    P_02,
    P_03,
    P_04,
    P_05,
    P_06,
    P_07,
    P_08,
    P_09,
    P_10,
    P_11,
    P_12,
    P_13,
    P_14,
    P_15,
    P_16,
    P_17,
    P_18,
    P_19,
    P_20,
    P_21,
    P_22,
    P_23,
    P_24,
    P_25,
    P_26,
    P_27,
    P_28,
    P_29,
    P_30,
    P_31,
}

impl_uart!(UART0, UART0);

impl_rtc!(AON_RTC, AON_RTC);

impl_gpt!(GPT0, GPT0A);

impl_pin!(P_00, 0);
impl_pin!(P_01, 1);
impl_pin!(P_02, 2);
impl_pin!(P_03, 3);
impl_pin!(P_04, 4);
impl_pin!(P_05, 5);
impl_pin!(P_06, 6);
impl_pin!(P_07, 7);
impl_pin!(P_08, 8);
impl_pin!(P_09, 9);
impl_pin!(P_10, 10);
impl_pin!(P_11, 11);
impl_pin!(P_12, 12);
impl_pin!(P_13, 13);
impl_pin!(P_14, 14);
impl_pin!(P_15, 15);
impl_pin!(P_16, 16);
impl_pin!(P_17, 17);
impl_pin!(P_18, 18);
impl_pin!(P_19, 19);
impl_pin!(P_20, 20);
impl_pin!(P_21, 21);
impl_pin!(P_22, 22);
impl_pin!(P_23, 23);
impl_pin!(P_24, 24);
impl_pin!(P_25, 25);
impl_pin!(P_26, 26);
impl_pin!(P_27, 27);
impl_pin!(P_28, 28);
impl_pin!(P_29, 29);
impl_pin!(P_30, 30);
impl_pin!(P_31, 31);

embassy_hal_internal::interrupt_mod!(UART0, AON_RTC, GPT0A, UDMA);
