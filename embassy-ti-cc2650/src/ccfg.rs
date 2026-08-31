use crate::pac;

/// This module defines CCFG structure and its default values, and allocates the CCFG structure
/// in its corresponding ELF section (.ccfg). If linker cooperates (i.e., defines .ccfg
/// as the appropriate flash region), then this makes CCFG flashed together with Tock.
mod ccfg_data {
    //*****************************************************************************
    //
    // Customer configuration (ccfg) typedef.
    // The implementation of this struct is required by device ROM boot code
    //  and must be placed at the end of flash. Do not modify this struct!
    //
    //*****************************************************************************

    #[allow(non_snake_case)]
    #[repr(C)]
    struct Ccfg {
        //  Mapped to address
        CCFG_EXT_LF_CLK: u32,         // 0x50003FA8
        CCFG_MODE_CONF_1: u32,        // 0x50003FAC
        CCFG_SIZE_AND_DIS_FLAGS: u32, // 0x50003FB0
        CCFG_MODE_CONF: u32,          // 0x50003FB4
        CCFG_VOLT_LOAD_0: u32,        // 0x50003FB8
        CCFG_VOLT_LOAD_1: u32,        // 0x50003FBC
        CCFG_RTC_OFFSET: u32,         // 0x50003FC0
        CCFG_FREQ_OFFSET: u32,        // 0x50003FC4
        CCFG_IEEE_MAC_0: u32,         // 0x50003FC8
        CCFG_IEEE_MAC_1: u32,         // 0x50003FCC
        CCFG_IEEE_BLE_0: u32,         // 0x50003FD0
        CCFG_IEEE_BLE_1: u32,         // 0x50003FD4
        CCFG_BL_CONFIG: u32,          // 0x50003FD8
        CCFG_ERASE_CONF: u32,         // 0x50003FDC
        CCFG_CCFG_TI_OPTIONS: u32,    // 0x50003FE0
        CCFG_CCFG_TAP_DAP_0: u32,     // 0x50003FE4
        CCFG_CCFG_TAP_DAP_1: u32,     // 0x50003FE8
        CCFG_IMAGE_VALID_CONF: u32,   // 0x50003FEC
        CCFG_CCFG_PROT_31_0: u32,     // 0x50003FF0
        CCFG_CCFG_PROT_63_32: u32,    // 0x50003FF4
        CCFG_CCFG_PROT_95_64: u32,    // 0x50003FF8
        CCFG_CCFG_PROT_127_96: u32,   // 0x50003FFC
    }

    use defaults::*;

    #[allow(unused)]
    #[unsafe(no_mangle)]
    #[unsafe(link_section = ".ccfg")]
    static CCFG: Ccfg = Ccfg {
        // Mapped to address
        CCFG_EXT_LF_CLK: DEFAULT_CCFG_O_EXT_LF_CLK, // 0x50003FA8 (0x50003xxx maps to last
        CCFG_MODE_CONF_1: DEFAULT_CCFG_MODE_CONF_1, // 0x50003FAC  sector in FLASH.
        CCFG_SIZE_AND_DIS_FLAGS: DEFAULT_CCFG_SIZE_AND_DIS_FLAGS, // 0x50003FB0  Independent of FLASH size)
        CCFG_MODE_CONF: DEFAULT_CCFG_MODE_CONF,     // 0x50003FB4
        CCFG_VOLT_LOAD_0: DEFAULT_CCFG_VOLT_LOAD_0, // 0x50003FB8
        CCFG_VOLT_LOAD_1: DEFAULT_CCFG_VOLT_LOAD_1, // 0x50003FBC
        CCFG_RTC_OFFSET: DEFAULT_CCFG_RTC_OFFSET,   // 0x50003FC0
        CCFG_FREQ_OFFSET: DEFAULT_CCFG_FREQ_OFFSET, // 0x50003FC4
        CCFG_IEEE_MAC_0: DEFAULT_CCFG_IEEE_MAC_0,   // 0x50003FC8
        CCFG_IEEE_MAC_1: DEFAULT_CCFG_IEEE_MAC_1,   // 0x50003FCC
        CCFG_IEEE_BLE_0: DEFAULT_CCFG_IEEE_BLE_0,   // 0x50003FD0
        CCFG_IEEE_BLE_1: DEFAULT_CCFG_IEEE_BLE_1,   // 0x50003FD4
        CCFG_BL_CONFIG: DEFAULT_CCFG_BL_CONFIG,     // 0x50003FD8
        CCFG_ERASE_CONF: DEFAULT_CCFG_ERASE_CONF,   // 0x50003FDC
        CCFG_CCFG_TI_OPTIONS: DEFAULT_CCFG_CCFG_TI_OPTIONS, // 0x50003FE0
        CCFG_CCFG_TAP_DAP_0: DEFAULT_CCFG_CCFG_TAP_DAP_0, // 0x50003FE4
        CCFG_CCFG_TAP_DAP_1: DEFAULT_CCFG_CCFG_TAP_DAP_1, // 0x50003FE8
        CCFG_IMAGE_VALID_CONF: DEFAULT_CCFG_IMAGE_VALID_CONF, // 0x50003FEC
        CCFG_CCFG_PROT_31_0: DEFAULT_CCFG_CCFG_PROT_31_0, // 0x50003FF0
        CCFG_CCFG_PROT_63_32: DEFAULT_CCFG_CCFG_PROT_63_32, // 0x50003FF4
        CCFG_CCFG_PROT_95_64: DEFAULT_CCFG_CCFG_PROT_95_64, // 0x50003FF8
        CCFG_CCFG_PROT_127_96: DEFAULT_CCFG_CCFG_PROT_127_96, // 0x50003FFC
    };

    #[allow(unused)]
    mod hw_ccfg {
        //*****************************************************************************
        //
        // This section defines the register offsets of
        // CCFG component
        //
        //*****************************************************************************
        // Extern LF clock configuration
        pub(super) const CCFG_O_EXT_LF_CLK: u32 = 0x00000FA8;

        // Mode Configuration 1
        pub(super) const CCFG_O_MODE_CONF_1: u32 = 0x00000FAC;

        // CCFG Size and Disable Flags
        pub(super) const CCFG_O_SIZE_AND_DIS_FLAGS: u32 = 0x00000FB0;

        // Mode Configuration 0
        pub(super) const CCFG_O_MODE_CONF: u32 = 0x00000FB4;

        // Voltage Load 0
        pub(super) const CCFG_O_VOLT_LOAD_0: u32 = 0x00000FB8;

        // Voltage Load 1
        pub(super) const CCFG_O_VOLT_LOAD_1: u32 = 0x00000FBC;

        // Real Time Clock Offset
        pub(super) const CCFG_O_RTC_OFFSET: u32 = 0x00000FC0;

        // Frequency Offset
        pub(super) const CCFG_O_FREQ_OFFSET: u32 = 0x00000FC4;

        // IEEE MAC Address 0
        pub(super) const CCFG_O_IEEE_MAC_0: u32 = 0x00000FC8;

        // IEEE MAC Address 1
        pub(super) const CCFG_O_IEEE_MAC_1: u32 = 0x00000FCC;

        // IEEE BLE Address 0
        pub(super) const CCFG_O_IEEE_BLE_0: u32 = 0x00000FD0;

        // IEEE BLE Address 1
        pub(super) const CCFG_O_IEEE_BLE_1: u32 = 0x00000FD4;

        // Bootloader Configuration
        pub(super) const CCFG_O_BL_CONFIG: u32 = 0x00000FD8;

        // Erase Configuration
        pub(super) const CCFG_O_ERASE_CONF: u32 = 0x00000FDC;

        // TI Options
        pub(super) const CCFG_O_CCFG_TI_OPTIONS: u32 = 0x00000FE0;

        // Test Access Points Enable 0
        pub(super) const CCFG_O_CCFG_TAP_DAP_0: u32 = 0x00000FE4;

        // Test Access Points Enable 1
        pub(super) const CCFG_O_CCFG_TAP_DAP_1: u32 = 0x00000FE8;

        // Image Valid
        pub(super) const CCFG_O_IMAGE_VALID_CONF: u32 = 0x00000FEC;

        // Protect Sectors 0-31
        pub(super) const CCFG_O_CCFG_PROT_31_0: u32 = 0x00000FF0;

        // Protect Sectors 32-63
        pub(super) const CCFG_O_CCFG_PROT_63_32: u32 = 0x00000FF4;

        // Protect Sectors 64-95
        pub(super) const CCFG_O_CCFG_PROT_95_64: u32 = 0x00000FF8;

        // Protect Sectors 96-127
        pub(super) const CCFG_O_CCFG_PROT_127_96: u32 = 0x00000FFC;

        //*****************************************************************************
        //
        // Register: CCFG_O_EXT_LF_CLK
        //
        //*****************************************************************************
        // Field: [31:24] DIO
        //
        // Unsigned integer, selecting the DIO to supply external 32kHz clock as
        // SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO
        // will be marked as reserved by the pin driver (TI-RTOS environment) and hence
        // not selectable for other usage.
        pub(super) const CCFG_EXT_LF_CLK_DIO_W: u32 = 8;
        pub(super) const CCFG_EXT_LF_CLK_DIO_M: u32 = 0xFF000000;
        pub(super) const CCFG_EXT_LF_CLK_DIO_S: u32 = 24;

        // Field:  [23:0] RTC_INCREMENT
        //
        // Unsigned integer, defining the input frequency of the external clock and is
        // written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows:
        // EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.:
        // RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)
        pub(super) const CCFG_EXT_LF_CLK_RTC_INCREMENT_W: u32 = 24;
        pub(super) const CCFG_EXT_LF_CLK_RTC_INCREMENT_M: u32 = 0x00FFFFFF;
        pub(super) const CCFG_EXT_LF_CLK_RTC_INCREMENT_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_MODE_CONF_1
        //
        //*****************************************************************************
        // Field: [23:20] ALT_DCDC_VMIN
        //
        // Minimum voltage for when DC/DC should be used if alternate DC/DC setting is
        // enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0).
        // Voltage = (28 + ALT_DCDC_VMIN) / 16.
        // 0: 1.75V
        // 1: 1.8125V
        // ...
        // 14: 2.625V
        // 15: 2.6875V
        //
        // NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must
        // be called regularly to apply this field (handled automatically if using TI
        // RTOS!).
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_VMIN_W: u32 = 4;
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_VMIN_M: u32 = 0x00F00000;
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_VMIN_S: u32 = 20;

        // Field:    [19] ALT_DCDC_DITHER_EN
        //
        // Enable DC/DC dithering if alternate DC/DC setting is enabled
        // (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0).
        // 0: Dither disable
        // 1: Dither enable
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN: u32 = 0x00080000;
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN_BITN: u32 = 19;
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN_M: u32 = 0x00080000;
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN_S: u32 = 19;

        // Field: [18:16] ALT_DCDC_IPEAK
        //
        // Inductor peak current if alternate DC/DC setting is enabled
        // (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external
        // inductor!
        // Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) :
        // 0: 31mA (min)
        // ...
        // 4: 47mA
        // ...
        // 7: 59mA (max)
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_IPEAK_W: u32 = 3;
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_IPEAK_M: u32 = 0x00070000;
        pub(super) const CCFG_MODE_CONF_1_ALT_DCDC_IPEAK_S: u32 = 16;

        // Field: [15:12] DELTA_IBIAS_INIT
        //
        // Signed delta value for IBIAS_INIT. Delta value only applies if
        // SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0.
        // See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT
        pub(super) const CCFG_MODE_CONF_1_DELTA_IBIAS_INIT_W: u32 = 4;
        pub(super) const CCFG_MODE_CONF_1_DELTA_IBIAS_INIT_M: u32 = 0x0000F000;
        pub(super) const CCFG_MODE_CONF_1_DELTA_IBIAS_INIT_S: u32 = 12;

        // Field:  [11:8] DELTA_IBIAS_OFFSET
        //
        // Signed delta value for IBIAS_OFFSET. Delta value only applies if
        // SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0.
        // See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET
        pub(super) const CCFG_MODE_CONF_1_DELTA_IBIAS_OFFSET_W: u32 = 4;
        pub(super) const CCFG_MODE_CONF_1_DELTA_IBIAS_OFFSET_M: u32 = 0x00000F00;
        pub(super) const CCFG_MODE_CONF_1_DELTA_IBIAS_OFFSET_S: u32 = 8;

        // Field:   [7:0] XOSC_MAX_START
        //
        // Unsigned value of maximum XOSC startup time (worst case) in units of 100us.
        // Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0.
        pub(super) const CCFG_MODE_CONF_1_XOSC_MAX_START_W: u32 = 8;
        pub(super) const CCFG_MODE_CONF_1_XOSC_MAX_START_M: u32 = 0x000000FF;
        pub(super) const CCFG_MODE_CONF_1_XOSC_MAX_START_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_SIZE_AND_DIS_FLAGS
        //
        //*****************************************************************************
        // Field: [31:16] SIZE_OF_CCFG
        //
        // Total size of CCFG in bytes.
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_SIZE_OF_CCFG_W: u32 = 16;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_SIZE_OF_CCFG_M: u32 = 0xFFFF0000;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_SIZE_OF_CCFG_S: u32 = 16;

        // Field:  [15:4] DISABLE_FLAGS
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DISABLE_FLAGS_W: u32 = 12;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DISABLE_FLAGS_M: u32 = 0x0000FFF0;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DISABLE_FLAGS_S: u32 = 4;

        // Field:     [3] DIS_TCXO
        //
        // Disable TCXO.
        // 0: TCXO functionality enabled.
        // 1: TCXO functionality disabled.
        // Note:
        // An external TCXO is required if DIS_TCXO = 0.
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_TCXO: u32 = 0x00000008;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_TCXO_BITN: u32 = 3;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_TCXO_M: u32 = 0x00000008;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_TCXO_S: u32 = 3;

        // Field:     [2] DIS_GPRAM
        //
        // Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM).
        // 0: GPRAM is enabled and hence CACHE disabled.
        // 1: GPRAM is disabled and instead CACHE is enabled (default).
        // Notes:
        // - Disabling CACHE will reduce CPU execution speed (up to 60%).
        // - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if
        // enabled.
        // See:
        // VIMS:CTL.MODE
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM: u32 = 0x00000004;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM_BITN: u32 = 2;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM_M: u32 = 0x00000004;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM_S: u32 = 2;

        // Field:     [1] DIS_ALT_DCDC_SETTING
        //
        // Disable alternate DC/DC settings.
        // 0: Enable alternate DC/DC settings.
        // 1: Disable alternate DC/DC settings.
        // See:
        // MODE_CONF_1.ALT_DCDC_VMIN
        // MODE_CONF_1.ALT_DCDC_DITHER_EN
        // MODE_CONF_1.ALT_DCDC_IPEAK
        //
        // NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must
        // be called regularly to apply this field (handled automatically if using TI
        // RTOS!).
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING: u32 = 0x00000002;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING_BITN: u32 = 1;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING_M: u32 = 0x00000002;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING_S: u32 = 1;

        // Field:     [0] DIS_XOSC_OVR
        //
        // Disable XOSC override functionality.
        // 0: Enable XOSC override functionality.
        // 1: Disable XOSC override functionality.
        // See:
        // MODE_CONF_1.DELTA_IBIAS_INIT
        // MODE_CONF_1.DELTA_IBIAS_OFFSET
        // MODE_CONF_1.XOSC_MAX_START
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR: u32 = 0x00000001;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR_BITN: u32 = 0;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR_M: u32 = 0x00000001;
        pub(super) const CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_MODE_CONF
        //
        //*****************************************************************************
        // Field: [31:28] VDDR_TRIM_SLEEP_DELTA
        //
        // Signed delta value to apply to the
        // VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H.
        // 0x8 (-8) : Delta = -7
        // ...
        // 0xF (-1) : Delta = 0
        // 0x0 (0) : Delta = +1
        // ...
        // 0x7 (7) : Delta = +8
        pub(super) const CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA_W: u32 = 4;
        pub(super) const CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA_M: u32 = 0xF0000000;
        pub(super) const CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA_S: u32 = 28;

        // Field:    [27] DCDC_RECHARGE
        //
        // DC/DC during recharge in powerdown.
        // 0: Use the DC/DC during recharge in powerdown.
        // 1: Do not use the DC/DC during recharge in powerdown (default).
        //
        // NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must
        // be called regularly to apply this field (handled automatically if using TI
        // RTOS!).
        pub(super) const CCFG_MODE_CONF_DCDC_RECHARGE: u32 = 0x08000000;
        pub(super) const CCFG_MODE_CONF_DCDC_RECHARGE_BITN: u32 = 27;
        pub(super) const CCFG_MODE_CONF_DCDC_RECHARGE_M: u32 = 0x08000000;
        pub(super) const CCFG_MODE_CONF_DCDC_RECHARGE_S: u32 = 27;

        // Field:    [26] DCDC_ACTIVE
        //
        // DC/DC in active mode.
        // 0: Use the DC/DC during active mode.
        // 1: Do not use the DC/DC during active mode (default).
        //
        // NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must
        // be called regularly to apply this field (handled automatically if using TI
        // RTOS!).
        pub(super) const CCFG_MODE_CONF_DCDC_ACTIVE: u32 = 0x04000000;
        pub(super) const CCFG_MODE_CONF_DCDC_ACTIVE_BITN: u32 = 26;
        pub(super) const CCFG_MODE_CONF_DCDC_ACTIVE_M: u32 = 0x04000000;
        pub(super) const CCFG_MODE_CONF_DCDC_ACTIVE_S: u32 = 26;

        // Field:    [25] VDDR_EXT_LOAD
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_MODE_CONF_VDDR_EXT_LOAD: u32 = 0x02000000;
        pub(super) const CCFG_MODE_CONF_VDDR_EXT_LOAD_BITN: u32 = 25;
        pub(super) const CCFG_MODE_CONF_VDDR_EXT_LOAD_M: u32 = 0x02000000;
        pub(super) const CCFG_MODE_CONF_VDDR_EXT_LOAD_S: u32 = 25;

        // Field:    [24] VDDS_BOD_LEVEL
        //
        // VDDS BOD level.
        // 0: VDDS BOD level is 2.0V (necessary for external load mode, or for maximum
        // PA output power on CC13xx).
        // 1: VDDS BOD level is 1.8V (or 1.65V for external regulator mode) (default).
        pub(super) const CCFG_MODE_CONF_VDDS_BOD_LEVEL: u32 = 0x01000000;
        pub(super) const CCFG_MODE_CONF_VDDS_BOD_LEVEL_BITN: u32 = 24;
        pub(super) const CCFG_MODE_CONF_VDDS_BOD_LEVEL_M: u32 = 0x01000000;
        pub(super) const CCFG_MODE_CONF_VDDS_BOD_LEVEL_S: u32 = 24;

        // Field: [23:22] SCLK_LF_OPTION
        //
        // Select source for SCLK_LF.
        // ENUMs:
        // RCOSC_LF                 Low frequency RCOSC (default)
        // XOSC_LF                  32.768kHz low frequency XOSC
        // EXTERNAL_LF              External low frequency clock on DIO defined by
        //                          EXT_LF_CLK.DIO. The RTC tick speed
        //                          AON_RTC:SUBSECINC is updated to
        //                          EXT_LF_CLK.RTC_INCREMENT (done in the
        //                          trimDevice() xxWare boot function). External
        //                          clock must always be running when the chip is
        //                          in standby for VDDR recharge timing.
        // XOSC_HF_DLF              31.25kHz clock derived from 24MHz XOSC (dividing
        //                          by 768 in HW). The RTC tick speed
        //                          [AON_RTC.SUBSECINC.*] is updated to 0x8637BD,
        //                          corresponding to a 31.25kHz clock (done in the
        //                          trimDevice() xxWare boot function). Standby
        //                          power mode is not supported when using this
        //                          clock source.
        pub(super) const CCFG_MODE_CONF_SCLK_LF_OPTION_W: u32 = 2;
        pub(super) const CCFG_MODE_CONF_SCLK_LF_OPTION_M: u32 = 0x00C00000;
        pub(super) const CCFG_MODE_CONF_SCLK_LF_OPTION_S: u32 = 22;
        pub(super) const CCFG_MODE_CONF_SCLK_LF_OPTION_RCOSC_LF: u32 = 0x00C00000;
        pub(super) const CCFG_MODE_CONF_SCLK_LF_OPTION_XOSC_LF: u32 = 0x00800000;
        pub(super) const CCFG_MODE_CONF_SCLK_LF_OPTION_EXTERNAL_LF: u32 = 0x00400000;
        pub(super) const CCFG_MODE_CONF_SCLK_LF_OPTION_XOSC_HF_DLF: u32 = 0x00000000;

        // Field:    [21] VDDR_TRIM_SLEEP_TC
        //
        // 0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated
        // 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time
        // standby mode is entered. This improves low-temperature RCOSC_LF frequency
        // stability in standby mode.
        //
        // When temperature compensation is performed, the delta is calculates this
        // way:
        // Delta = max (delta, min(8, floor(62-temp)/8))
        // Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current
        // temperature in degrees C.
        pub(super) const CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC: u32 = 0x00200000;
        pub(super) const CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC_BITN: u32 = 21;
        pub(super) const CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC_M: u32 = 0x00200000;
        pub(super) const CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC_S: u32 = 21;

        // Field:    [20] RTC_COMP
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_MODE_CONF_RTC_COMP: u32 = 0x00100000;
        pub(super) const CCFG_MODE_CONF_RTC_COMP_BITN: u32 = 20;
        pub(super) const CCFG_MODE_CONF_RTC_COMP_M: u32 = 0x00100000;
        pub(super) const CCFG_MODE_CONF_RTC_COMP_S: u32 = 20;

        // Field: [19:18] XOSC_FREQ
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        // ENUMs:
        // 24M                      24 MHz XOSC_HF
        // 48M                      48 MHz XOSC_HF
        // HPOSC                    HPOSC
        pub(super) const CCFG_MODE_CONF_XOSC_FREQ_W: u32 = 2;
        pub(super) const CCFG_MODE_CONF_XOSC_FREQ_M: u32 = 0x000C0000;
        pub(super) const CCFG_MODE_CONF_XOSC_FREQ_S: u32 = 18;
        pub(super) const CCFG_MODE_CONF_XOSC_FREQ_24M: u32 = 0x000C0000;
        pub(super) const CCFG_MODE_CONF_XOSC_FREQ_48M: u32 = 0x00080000;
        pub(super) const CCFG_MODE_CONF_XOSC_FREQ_HPOSC: u32 = 0x00040000;

        // Field:    [17] XOSC_CAP_MOD
        //
        // Enable modification (delta) to XOSC cap-array. Value specified in
        // XOSC_CAPARRAY_DELTA.
        // 0: Apply cap-array delta
        // 1: Do not apply cap-array delta (default)
        pub(super) const CCFG_MODE_CONF_XOSC_CAP_MOD: u32 = 0x00020000;
        pub(super) const CCFG_MODE_CONF_XOSC_CAP_MOD_BITN: u32 = 17;
        pub(super) const CCFG_MODE_CONF_XOSC_CAP_MOD_M: u32 = 0x00020000;
        pub(super) const CCFG_MODE_CONF_XOSC_CAP_MOD_S: u32 = 17;

        // Field:    [16] HF_COMP
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_MODE_CONF_HF_COMP: u32 = 0x00010000;
        pub(super) const CCFG_MODE_CONF_HF_COMP_BITN: u32 = 16;
        pub(super) const CCFG_MODE_CONF_HF_COMP_M: u32 = 0x00010000;
        pub(super) const CCFG_MODE_CONF_HF_COMP_S: u32 = 16;

        // Field:  [15:8] XOSC_CAPARRAY_DELTA
        //
        // Signed 8-bit value, directly modifying trimmed XOSC cap-array step value.
        // Enabled by XOSC_CAP_MOD.
        pub(super) const CCFG_MODE_CONF_XOSC_CAPARRAY_DELTA_W: u32 = 8;
        pub(super) const CCFG_MODE_CONF_XOSC_CAPARRAY_DELTA_M: u32 = 0x0000FF00;
        pub(super) const CCFG_MODE_CONF_XOSC_CAPARRAY_DELTA_S: u32 = 8;

        // Field:   [7:0] VDDR_CAP
        //
        // Unsigned 8-bit integer, representing the minimum decoupling capacitance
        // (worst case) on VDDR, in units of 100nF. This should take into account
        // capacitor tolerance and voltage dependent capacitance variation. This bit
        // affects the recharge period calculation when going into powerdown or
        // standby.
        //
        // NOTE! If using the following functions this field must be configured (used
        // by TI RTOS):
        // SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()
        pub(super) const CCFG_MODE_CONF_VDDR_CAP_W: u32 = 8;
        pub(super) const CCFG_MODE_CONF_VDDR_CAP_M: u32 = 0x000000FF;
        pub(super) const CCFG_MODE_CONF_VDDR_CAP_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_VOLT_LOAD_0
        //
        //*****************************************************************************
        // Field: [31:24] VDDR_EXT_TP45
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TP45_W: u32 = 8;
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TP45_M: u32 = 0xFF000000;
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TP45_S: u32 = 24;

        // Field: [23:16] VDDR_EXT_TP25
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TP25_W: u32 = 8;
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TP25_M: u32 = 0x00FF0000;
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TP25_S: u32 = 16;

        // Field:  [15:8] VDDR_EXT_TP5
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TP5_W: u32 = 8;
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TP5_M: u32 = 0x0000FF00;
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TP5_S: u32 = 8;

        // Field:   [7:0] VDDR_EXT_TM15
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TM15_W: u32 = 8;
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TM15_M: u32 = 0x000000FF;
        pub(super) const CCFG_VOLT_LOAD_0_VDDR_EXT_TM15_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_VOLT_LOAD_1
        //
        //*****************************************************************************
        // Field: [31:24] VDDR_EXT_TP125
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP125_W: u32 = 8;
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP125_M: u32 = 0xFF000000;
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP125_S: u32 = 24;

        // Field: [23:16] VDDR_EXT_TP105
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP105_W: u32 = 8;
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP105_M: u32 = 0x00FF0000;
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP105_S: u32 = 16;

        // Field:  [15:8] VDDR_EXT_TP85
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP85_W: u32 = 8;
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP85_M: u32 = 0x0000FF00;
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP85_S: u32 = 8;

        // Field:   [7:0] VDDR_EXT_TP65
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP65_W: u32 = 8;
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP65_M: u32 = 0x000000FF;
        pub(super) const CCFG_VOLT_LOAD_1_VDDR_EXT_TP65_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_RTC_OFFSET
        //
        //*****************************************************************************
        // Field: [31:16] RTC_COMP_P0
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_RTC_OFFSET_RTC_COMP_P0_W: u32 = 16;
        pub(super) const CCFG_RTC_OFFSET_RTC_COMP_P0_M: u32 = 0xFFFF0000;
        pub(super) const CCFG_RTC_OFFSET_RTC_COMP_P0_S: u32 = 16;

        // Field:  [15:8] RTC_COMP_P1
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_RTC_OFFSET_RTC_COMP_P1_W: u32 = 8;
        pub(super) const CCFG_RTC_OFFSET_RTC_COMP_P1_M: u32 = 0x0000FF00;
        pub(super) const CCFG_RTC_OFFSET_RTC_COMP_P1_S: u32 = 8;

        // Field:   [7:0] RTC_COMP_P2
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_RTC_OFFSET_RTC_COMP_P2_W: u32 = 8;
        pub(super) const CCFG_RTC_OFFSET_RTC_COMP_P2_M: u32 = 0x000000FF;
        pub(super) const CCFG_RTC_OFFSET_RTC_COMP_P2_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_FREQ_OFFSET
        //
        //*****************************************************************************
        // Field: [31:16] HF_COMP_P0
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_FREQ_OFFSET_HF_COMP_P0_W: u32 = 16;
        pub(super) const CCFG_FREQ_OFFSET_HF_COMP_P0_M: u32 = 0xFFFF0000;
        pub(super) const CCFG_FREQ_OFFSET_HF_COMP_P0_S: u32 = 16;

        // Field:  [15:8] HF_COMP_P1
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_FREQ_OFFSET_HF_COMP_P1_W: u32 = 8;
        pub(super) const CCFG_FREQ_OFFSET_HF_COMP_P1_M: u32 = 0x0000FF00;
        pub(super) const CCFG_FREQ_OFFSET_HF_COMP_P1_S: u32 = 8;

        // Field:   [7:0] HF_COMP_P2
        //
        // Reserved for future use. Software should not rely on the value of a
        // reserved. Writing any other value than the reset/default value may result in
        // undefined behavior.
        pub(super) const CCFG_FREQ_OFFSET_HF_COMP_P2_W: u32 = 8;
        pub(super) const CCFG_FREQ_OFFSET_HF_COMP_P2_M: u32 = 0x000000FF;
        pub(super) const CCFG_FREQ_OFFSET_HF_COMP_P2_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_IEEE_MAC_0
        //
        //*****************************************************************************
        // Field:  [31:0] ADDR
        //
        // Bits[31:0] of the 64-bits custom IEEE MAC address.
        // If different from 0xFFFFFFFF then the value of this field is applied;
        // otherwise use value from FCFG.
        pub(super) const CCFG_IEEE_MAC_0_ADDR_W: u32 = 32;
        pub(super) const CCFG_IEEE_MAC_0_ADDR_M: u32 = 0xFFFFFFFF;
        pub(super) const CCFG_IEEE_MAC_0_ADDR_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_IEEE_MAC_1
        //
        //*****************************************************************************
        // Field:  [31:0] ADDR
        //
        // Bits[63:32] of the 64-bits custom IEEE MAC address.
        // If different from 0xFFFFFFFF then the value of this field is applied;
        // otherwise use value from FCFG.
        pub(super) const CCFG_IEEE_MAC_1_ADDR_W: u32 = 32;
        pub(super) const CCFG_IEEE_MAC_1_ADDR_M: u32 = 0xFFFFFFFF;
        pub(super) const CCFG_IEEE_MAC_1_ADDR_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_IEEE_BLE_0
        //
        //*****************************************************************************
        // Field:  [31:0] ADDR
        //
        // Bits[31:0] of the 64-bits custom IEEE BLE address.
        // If different from 0xFFFFFFFF then the value of this field is applied;
        // otherwise use value from FCFG.
        pub(super) const CCFG_IEEE_BLE_0_ADDR_W: u32 = 32;
        pub(super) const CCFG_IEEE_BLE_0_ADDR_M: u32 = 0xFFFFFFFF;
        pub(super) const CCFG_IEEE_BLE_0_ADDR_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_IEEE_BLE_1
        //
        //*****************************************************************************
        // Field:  [31:0] ADDR
        //
        // Bits[63:32] of the 64-bits custom IEEE BLE address.
        // If different from 0xFFFFFFFF then the value of this field is applied;
        // otherwise use value from FCFG.
        pub(super) const CCFG_IEEE_BLE_1_ADDR_W: u32 = 32;
        pub(super) const CCFG_IEEE_BLE_1_ADDR_M: u32 = 0xFFFFFFFF;
        pub(super) const CCFG_IEEE_BLE_1_ADDR_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_BL_CONFIG
        //
        //*****************************************************************************
        // Field: [31:24] BOOTLOADER_ENABLE
        //
        // Bootloader enable. Boot loader can be accessed if
        // IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and
        // conditions for boot loader backdoor are met).
        // 0xC5: Boot loader is enabled.
        // Any other value: Boot loader is disabled.
        pub(super) const CCFG_BL_CONFIG_BOOTLOADER_ENABLE_W: u32 = 8;
        pub(super) const CCFG_BL_CONFIG_BOOTLOADER_ENABLE_M: u32 = 0xFF000000;
        pub(super) const CCFG_BL_CONFIG_BOOTLOADER_ENABLE_S: u32 = 24;

        // Field:    [16] BL_LEVEL
        //
        // Sets the active level of the selected DIO number BL_PIN_NUMBER if boot
        // loader backdoor is enabled by the BL_ENABLE field.
        // 0: Active low.
        // 1: Active high.
        pub(super) const CCFG_BL_CONFIG_BL_LEVEL: u32 = 0x00010000;
        pub(super) const CCFG_BL_CONFIG_BL_LEVEL_BITN: u32 = 16;
        pub(super) const CCFG_BL_CONFIG_BL_LEVEL_M: u32 = 0x00010000;
        pub(super) const CCFG_BL_CONFIG_BL_LEVEL_S: u32 = 16;

        // Field:  [15:8] BL_PIN_NUMBER
        //
        // DIO number that is level checked if the boot loader backdoor is enabled by
        // the BL_ENABLE field.
        pub(super) const CCFG_BL_CONFIG_BL_PIN_NUMBER_W: u32 = 8;
        pub(super) const CCFG_BL_CONFIG_BL_PIN_NUMBER_M: u32 = 0x0000FF00;
        pub(super) const CCFG_BL_CONFIG_BL_PIN_NUMBER_S: u32 = 8;

        // Field:   [7:0] BL_ENABLE
        //
        // Enables the boot loader backdoor.
        // 0xC5: Boot loader backdoor is enabled.
        // Any other value: Boot loader backdoor is disabled.
        //
        // NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader
        // backdoor is enabled.
        pub(super) const CCFG_BL_CONFIG_BL_ENABLE_W: u32 = 8;
        pub(super) const CCFG_BL_CONFIG_BL_ENABLE_M: u32 = 0x000000FF;
        pub(super) const CCFG_BL_CONFIG_BL_ENABLE_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_ERASE_CONF
        //
        //*****************************************************************************
        // Field:     [8] CHIP_ERASE_DIS_N
        //
        // Chip erase.
        // This bit controls if a chip erase requested through the JTAG WUC TAP will be
        // ignored in a following boot caused by a reset of the MCU VD.
        // A successful chip erase operation will force the content of the flash main
        // bank back to the state as it was when delivered by TI.
        // 0: Disable. Any chip erase request detected during boot will be ignored.
        // 1: Enable. Any chip erase request detected during boot will be performed by
        // the boot FW.
        pub(super) const CCFG_ERASE_CONF_CHIP_ERASE_DIS_N: u32 = 0x00000100;
        pub(super) const CCFG_ERASE_CONF_CHIP_ERASE_DIS_N_BITN: u32 = 8;
        pub(super) const CCFG_ERASE_CONF_CHIP_ERASE_DIS_N_M: u32 = 0x00000100;
        pub(super) const CCFG_ERASE_CONF_CHIP_ERASE_DIS_N_S: u32 = 8;

        // Field:     [0] BANK_ERASE_DIS_N
        //
        // Bank erase.
        // This bit controls if the ROM serial boot loader will accept a received Bank
        // Erase command (COMMAND_BANK_ERASE).
        // A successful Bank Erase operation will erase all main bank sectors not
        // protected by write protect configuration bits in CCFG.
        // 0: Disable the boot loader bank erase function.
        // 1: Enable the boot loader bank erase function.
        pub(super) const CCFG_ERASE_CONF_BANK_ERASE_DIS_N: u32 = 0x00000001;
        pub(super) const CCFG_ERASE_CONF_BANK_ERASE_DIS_N_BITN: u32 = 0;
        pub(super) const CCFG_ERASE_CONF_BANK_ERASE_DIS_N_M: u32 = 0x00000001;
        pub(super) const CCFG_ERASE_CONF_BANK_ERASE_DIS_N_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_CCFG_TI_OPTIONS
        //
        //*****************************************************************************
        // Field:   [7:0] TI_FA_ENABLE
        //
        // TI Failure Analysis.
        // 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis)
        // option with the unlock code.
        // All other values: Disable the functionality of unlocking the TI FA option
        // with the unlock code.
        pub(super) const CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE_W: u32 = 8;
        pub(super) const CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE_M: u32 = 0x000000FF;
        pub(super) const CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_CCFG_TAP_DAP_0
        //
        //*****************************************************************************
        // Field: [23:16] CPU_DAP_ENABLE
        //
        // Enable CPU DAP.
        // 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM
        // boot FW.
        // Any other value: Main CPU DAP access will remain disabled out of
        // power-up/system-reset.
        pub(super) const CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE_W: u32 = 8;
        pub(super) const CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE_M: u32 = 0x00FF0000;
        pub(super) const CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE_S: u32 = 16;

        // Field:  [15:8] PRCM_TAP_ENABLE
        //
        // Enable PRCM TAP.
        // 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW
        // if enabled by corresponding configuration value in FCFG1 defined by TI.
        // Any other value: PRCM TAP access will remain disabled out of
        // power-up/system-reset.
        pub(super) const CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE_W: u32 = 8;
        pub(super) const CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE_M: u32 = 0x0000FF00;
        pub(super) const CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE_S: u32 = 8;

        // Field:   [7:0] TEST_TAP_ENABLE
        //
        // Enable Test TAP.
        // 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW
        // if enabled by corresponding configuration value in FCFG1 defined by TI.
        // Any other value: TEST TAP access will remain disabled out of
        // power-up/system-reset.
        pub(super) const CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE_W: u32 = 8;
        pub(super) const CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE_M: u32 = 0x000000FF;
        pub(super) const CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_CCFG_TAP_DAP_1
        //
        //*****************************************************************************
        // Field: [23:16] PBIST2_TAP_ENABLE
        //
        // Enable PBIST2 TAP.
        // 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot
        // FW if enabled by corresponding configuration value in FCFG1 defined by TI.
        // Any other value: PBIST2 TAP access will remain disabled out of
        // power-up/system-reset.
        pub(super) const CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE_W: u32 = 8;
        pub(super) const CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE_M: u32 = 0x00FF0000;
        pub(super) const CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE_S: u32 = 16;

        // Field:  [15:8] PBIST1_TAP_ENABLE
        //
        // Enable PBIST1 TAP.
        // 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot
        // FW if enabled by corresponding configuration value in FCFG1 defined by TI.
        // Any other value: PBIST1 TAP access will remain disabled out of
        // power-up/system-reset.
        pub(super) const CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE_W: u32 = 8;
        pub(super) const CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE_M: u32 = 0x0000FF00;
        pub(super) const CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE_S: u32 = 8;

        // Field:   [7:0] WUC_TAP_ENABLE
        //
        // Enable WUC TAP
        // 0xC5: WUC TAP access is enabled during power-up/system-reset by ROM boot FW
        // if enabled by corresponding configuration value in FCFG1 defined by TI.
        // Any other value: WUC TAP access will remain disabled out of
        // power-up/system-reset.
        pub(super) const CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE_W: u32 = 8;
        pub(super) const CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE_M: u32 = 0x000000FF;
        pub(super) const CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_IMAGE_VALID_CONF
        //
        //*****************************************************************************
        // Field:  [31:0] IMAGE_VALID
        //
        // This field must have a value of 0x00000000 in order for enabling the boot
        // sequence to transfer control to a flash image.
        // A non-zero value forces the boot sequence to call the boot loader.
        pub(super) const CCFG_IMAGE_VALID_CONF_IMAGE_VALID_W: u32 = 32;
        pub(super) const CCFG_IMAGE_VALID_CONF_IMAGE_VALID_M: u32 = 0xFFFFFFFF;
        pub(super) const CCFG_IMAGE_VALID_CONF_IMAGE_VALID_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_CCFG_PROT_31_0
        //
        //*****************************************************************************
        // Field:    [31] WRT_PROT_SEC_31
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_31: u32 = 0x80000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_31_BITN: u32 = 31;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_31_M: u32 = 0x80000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_31_S: u32 = 31;

        // Field:    [30] WRT_PROT_SEC_30
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_30: u32 = 0x40000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_30_BITN: u32 = 30;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_30_M: u32 = 0x40000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_30_S: u32 = 30;

        // Field:    [29] WRT_PROT_SEC_29
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_29: u32 = 0x20000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_29_BITN: u32 = 29;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_29_M: u32 = 0x20000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_29_S: u32 = 29;

        // Field:    [28] WRT_PROT_SEC_28
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_28: u32 = 0x10000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_28_BITN: u32 = 28;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_28_M: u32 = 0x10000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_28_S: u32 = 28;

        // Field:    [27] WRT_PROT_SEC_27
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_27: u32 = 0x08000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_27_BITN: u32 = 27;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_27_M: u32 = 0x08000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_27_S: u32 = 27;

        // Field:    [26] WRT_PROT_SEC_26
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_26: u32 = 0x04000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_26_BITN: u32 = 26;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_26_M: u32 = 0x04000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_26_S: u32 = 26;

        // Field:    [25] WRT_PROT_SEC_25
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_25: u32 = 0x02000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_25_BITN: u32 = 25;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_25_M: u32 = 0x02000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_25_S: u32 = 25;

        // Field:    [24] WRT_PROT_SEC_24
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_24: u32 = 0x01000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_24_BITN: u32 = 24;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_24_M: u32 = 0x01000000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_24_S: u32 = 24;

        // Field:    [23] WRT_PROT_SEC_23
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_23: u32 = 0x00800000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_23_BITN: u32 = 23;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_23_M: u32 = 0x00800000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_23_S: u32 = 23;

        // Field:    [22] WRT_PROT_SEC_22
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_22: u32 = 0x00400000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_22_BITN: u32 = 22;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_22_M: u32 = 0x00400000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_22_S: u32 = 22;

        // Field:    [21] WRT_PROT_SEC_21
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_21: u32 = 0x00200000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_21_BITN: u32 = 21;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_21_M: u32 = 0x00200000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_21_S: u32 = 21;

        // Field:    [20] WRT_PROT_SEC_20
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_20: u32 = 0x00100000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_20_BITN: u32 = 20;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_20_M: u32 = 0x00100000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_20_S: u32 = 20;

        // Field:    [19] WRT_PROT_SEC_19
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_19: u32 = 0x00080000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_19_BITN: u32 = 19;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_19_M: u32 = 0x00080000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_19_S: u32 = 19;

        // Field:    [18] WRT_PROT_SEC_18
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_18: u32 = 0x00040000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_18_BITN: u32 = 18;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_18_M: u32 = 0x00040000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_18_S: u32 = 18;

        // Field:    [17] WRT_PROT_SEC_17
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_17: u32 = 0x00020000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_17_BITN: u32 = 17;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_17_M: u32 = 0x00020000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_17_S: u32 = 17;

        // Field:    [16] WRT_PROT_SEC_16
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_16: u32 = 0x00010000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_16_BITN: u32 = 16;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_16_M: u32 = 0x00010000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_16_S: u32 = 16;

        // Field:    [15] WRT_PROT_SEC_15
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_15: u32 = 0x00008000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_15_BITN: u32 = 15;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_15_M: u32 = 0x00008000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_15_S: u32 = 15;

        // Field:    [14] WRT_PROT_SEC_14
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_14: u32 = 0x00004000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_14_BITN: u32 = 14;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_14_M: u32 = 0x00004000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_14_S: u32 = 14;

        // Field:    [13] WRT_PROT_SEC_13
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_13: u32 = 0x00002000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_13_BITN: u32 = 13;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_13_M: u32 = 0x00002000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_13_S: u32 = 13;

        // Field:    [12] WRT_PROT_SEC_12
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_12: u32 = 0x00001000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_12_BITN: u32 = 12;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_12_M: u32 = 0x00001000;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_12_S: u32 = 12;

        // Field:    [11] WRT_PROT_SEC_11
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_11: u32 = 0x00000800;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_11_BITN: u32 = 11;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_11_M: u32 = 0x00000800;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_11_S: u32 = 11;

        // Field:    [10] WRT_PROT_SEC_10
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_10: u32 = 0x00000400;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_10_BITN: u32 = 10;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_10_M: u32 = 0x00000400;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_10_S: u32 = 10;

        // Field:     [9] WRT_PROT_SEC_9
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_9: u32 = 0x00000200;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_9_BITN: u32 = 9;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_9_M: u32 = 0x00000200;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_9_S: u32 = 9;

        // Field:     [8] WRT_PROT_SEC_8
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_8: u32 = 0x00000100;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_8_BITN: u32 = 8;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_8_M: u32 = 0x00000100;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_8_S: u32 = 8;

        // Field:     [7] WRT_PROT_SEC_7
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_7: u32 = 0x00000080;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_7_BITN: u32 = 7;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_7_M: u32 = 0x00000080;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_7_S: u32 = 7;

        // Field:     [6] WRT_PROT_SEC_6
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_6: u32 = 0x00000040;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_6_BITN: u32 = 6;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_6_M: u32 = 0x00000040;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_6_S: u32 = 6;

        // Field:     [5] WRT_PROT_SEC_5
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_5: u32 = 0x00000020;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_5_BITN: u32 = 5;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_5_M: u32 = 0x00000020;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_5_S: u32 = 5;

        // Field:     [4] WRT_PROT_SEC_4
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_4: u32 = 0x00000010;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_4_BITN: u32 = 4;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_4_M: u32 = 0x00000010;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_4_S: u32 = 4;

        // Field:     [3] WRT_PROT_SEC_3
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_3: u32 = 0x00000008;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_3_BITN: u32 = 3;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_3_M: u32 = 0x00000008;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_3_S: u32 = 3;

        // Field:     [2] WRT_PROT_SEC_2
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_2: u32 = 0x00000004;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_2_BITN: u32 = 2;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_2_M: u32 = 0x00000004;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_2_S: u32 = 2;

        // Field:     [1] WRT_PROT_SEC_1
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_1: u32 = 0x00000002;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_1_BITN: u32 = 1;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_1_M: u32 = 0x00000002;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_1_S: u32 = 1;

        // Field:     [0] WRT_PROT_SEC_0
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_0: u32 = 0x00000001;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_0_BITN: u32 = 0;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_0_M: u32 = 0x00000001;
        pub(super) const CCFG_CCFG_PROT_31_0_WRT_PROT_SEC_0_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_CCFG_PROT_63_32
        //
        //*****************************************************************************
        // Field:    [31] WRT_PROT_SEC_63
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_63: u32 = 0x80000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_63_BITN: u32 = 31;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_63_M: u32 = 0x80000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_63_S: u32 = 31;

        // Field:    [30] WRT_PROT_SEC_62
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_62: u32 = 0x40000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_62_BITN: u32 = 30;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_62_M: u32 = 0x40000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_62_S: u32 = 30;

        // Field:    [29] WRT_PROT_SEC_61
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_61: u32 = 0x20000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_61_BITN: u32 = 29;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_61_M: u32 = 0x20000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_61_S: u32 = 29;

        // Field:    [28] WRT_PROT_SEC_60
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_60: u32 = 0x10000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_60_BITN: u32 = 28;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_60_M: u32 = 0x10000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_60_S: u32 = 28;

        // Field:    [27] WRT_PROT_SEC_59
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_59: u32 = 0x08000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_59_BITN: u32 = 27;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_59_M: u32 = 0x08000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_59_S: u32 = 27;

        // Field:    [26] WRT_PROT_SEC_58
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_58: u32 = 0x04000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_58_BITN: u32 = 26;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_58_M: u32 = 0x04000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_58_S: u32 = 26;

        // Field:    [25] WRT_PROT_SEC_57
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_57: u32 = 0x02000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_57_BITN: u32 = 25;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_57_M: u32 = 0x02000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_57_S: u32 = 25;

        // Field:    [24] WRT_PROT_SEC_56
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_56: u32 = 0x01000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_56_BITN: u32 = 24;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_56_M: u32 = 0x01000000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_56_S: u32 = 24;

        // Field:    [23] WRT_PROT_SEC_55
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_55: u32 = 0x00800000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_55_BITN: u32 = 23;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_55_M: u32 = 0x00800000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_55_S: u32 = 23;

        // Field:    [22] WRT_PROT_SEC_54
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_54: u32 = 0x00400000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_54_BITN: u32 = 22;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_54_M: u32 = 0x00400000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_54_S: u32 = 22;

        // Field:    [21] WRT_PROT_SEC_53
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_53: u32 = 0x00200000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_53_BITN: u32 = 21;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_53_M: u32 = 0x00200000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_53_S: u32 = 21;

        // Field:    [20] WRT_PROT_SEC_52
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_52: u32 = 0x00100000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_52_BITN: u32 = 20;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_52_M: u32 = 0x00100000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_52_S: u32 = 20;

        // Field:    [19] WRT_PROT_SEC_51
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_51: u32 = 0x00080000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_51_BITN: u32 = 19;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_51_M: u32 = 0x00080000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_51_S: u32 = 19;

        // Field:    [18] WRT_PROT_SEC_50
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_50: u32 = 0x00040000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_50_BITN: u32 = 18;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_50_M: u32 = 0x00040000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_50_S: u32 = 18;

        // Field:    [17] WRT_PROT_SEC_49
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_49: u32 = 0x00020000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_49_BITN: u32 = 17;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_49_M: u32 = 0x00020000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_49_S: u32 = 17;

        // Field:    [16] WRT_PROT_SEC_48
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_48: u32 = 0x00010000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_48_BITN: u32 = 16;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_48_M: u32 = 0x00010000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_48_S: u32 = 16;

        // Field:    [15] WRT_PROT_SEC_47
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_47: u32 = 0x00008000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_47_BITN: u32 = 15;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_47_M: u32 = 0x00008000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_47_S: u32 = 15;

        // Field:    [14] WRT_PROT_SEC_46
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_46: u32 = 0x00004000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_46_BITN: u32 = 14;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_46_M: u32 = 0x00004000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_46_S: u32 = 14;

        // Field:    [13] WRT_PROT_SEC_45
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_45: u32 = 0x00002000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_45_BITN: u32 = 13;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_45_M: u32 = 0x00002000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_45_S: u32 = 13;

        // Field:    [12] WRT_PROT_SEC_44
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_44: u32 = 0x00001000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_44_BITN: u32 = 12;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_44_M: u32 = 0x00001000;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_44_S: u32 = 12;

        // Field:    [11] WRT_PROT_SEC_43
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_43: u32 = 0x00000800;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_43_BITN: u32 = 11;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_43_M: u32 = 0x00000800;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_43_S: u32 = 11;

        // Field:    [10] WRT_PROT_SEC_42
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_42: u32 = 0x00000400;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_42_BITN: u32 = 10;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_42_M: u32 = 0x00000400;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_42_S: u32 = 10;

        // Field:     [9] WRT_PROT_SEC_41
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_41: u32 = 0x00000200;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_41_BITN: u32 = 9;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_41_M: u32 = 0x00000200;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_41_S: u32 = 9;

        // Field:     [8] WRT_PROT_SEC_40
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_40: u32 = 0x00000100;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_40_BITN: u32 = 8;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_40_M: u32 = 0x00000100;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_40_S: u32 = 8;

        // Field:     [7] WRT_PROT_SEC_39
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_39: u32 = 0x00000080;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_39_BITN: u32 = 7;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_39_M: u32 = 0x00000080;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_39_S: u32 = 7;

        // Field:     [6] WRT_PROT_SEC_38
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_38: u32 = 0x00000040;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_38_BITN: u32 = 6;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_38_M: u32 = 0x00000040;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_38_S: u32 = 6;

        // Field:     [5] WRT_PROT_SEC_37
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_37: u32 = 0x00000020;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_37_BITN: u32 = 5;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_37_M: u32 = 0x00000020;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_37_S: u32 = 5;

        // Field:     [4] WRT_PROT_SEC_36
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_36: u32 = 0x00000010;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_36_BITN: u32 = 4;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_36_M: u32 = 0x00000010;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_36_S: u32 = 4;

        // Field:     [3] WRT_PROT_SEC_35
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_35: u32 = 0x00000008;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_35_BITN: u32 = 3;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_35_M: u32 = 0x00000008;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_35_S: u32 = 3;

        // Field:     [2] WRT_PROT_SEC_34
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_34: u32 = 0x00000004;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_34_BITN: u32 = 2;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_34_M: u32 = 0x00000004;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_34_S: u32 = 2;

        // Field:     [1] WRT_PROT_SEC_33
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_33: u32 = 0x00000002;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_33_BITN: u32 = 1;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_33_M: u32 = 0x00000002;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_33_S: u32 = 1;

        // Field:     [0] WRT_PROT_SEC_32
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_32: u32 = 0x00000001;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_32_BITN: u32 = 0;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_32_M: u32 = 0x00000001;
        pub(super) const CCFG_CCFG_PROT_63_32_WRT_PROT_SEC_32_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_CCFG_PROT_95_64
        //
        //*****************************************************************************
        // Field:    [31] WRT_PROT_SEC_95
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_95: u32 = 0x80000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_95_BITN: u32 = 31;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_95_M: u32 = 0x80000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_95_S: u32 = 31;

        // Field:    [30] WRT_PROT_SEC_94
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_94: u32 = 0x40000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_94_BITN: u32 = 30;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_94_M: u32 = 0x40000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_94_S: u32 = 30;

        // Field:    [29] WRT_PROT_SEC_93
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_93: u32 = 0x20000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_93_BITN: u32 = 29;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_93_M: u32 = 0x20000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_93_S: u32 = 29;

        // Field:    [28] WRT_PROT_SEC_92
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_92: u32 = 0x10000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_92_BITN: u32 = 28;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_92_M: u32 = 0x10000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_92_S: u32 = 28;

        // Field:    [27] WRT_PROT_SEC_91
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_91: u32 = 0x08000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_91_BITN: u32 = 27;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_91_M: u32 = 0x08000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_91_S: u32 = 27;

        // Field:    [26] WRT_PROT_SEC_90
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_90: u32 = 0x04000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_90_BITN: u32 = 26;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_90_M: u32 = 0x04000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_90_S: u32 = 26;

        // Field:    [25] WRT_PROT_SEC_89
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_89: u32 = 0x02000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_89_BITN: u32 = 25;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_89_M: u32 = 0x02000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_89_S: u32 = 25;

        // Field:    [24] WRT_PROT_SEC_88
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_88: u32 = 0x01000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_88_BITN: u32 = 24;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_88_M: u32 = 0x01000000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_88_S: u32 = 24;

        // Field:    [23] WRT_PROT_SEC_87
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_87: u32 = 0x00800000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_87_BITN: u32 = 23;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_87_M: u32 = 0x00800000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_87_S: u32 = 23;

        // Field:    [22] WRT_PROT_SEC_86
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_86: u32 = 0x00400000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_86_BITN: u32 = 22;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_86_M: u32 = 0x00400000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_86_S: u32 = 22;

        // Field:    [21] WRT_PROT_SEC_85
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_85: u32 = 0x00200000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_85_BITN: u32 = 21;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_85_M: u32 = 0x00200000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_85_S: u32 = 21;

        // Field:    [20] WRT_PROT_SEC_84
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_84: u32 = 0x00100000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_84_BITN: u32 = 20;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_84_M: u32 = 0x00100000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_84_S: u32 = 20;

        // Field:    [19] WRT_PROT_SEC_83
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_83: u32 = 0x00080000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_83_BITN: u32 = 19;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_83_M: u32 = 0x00080000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_83_S: u32 = 19;

        // Field:    [18] WRT_PROT_SEC_82
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_82: u32 = 0x00040000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_82_BITN: u32 = 18;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_82_M: u32 = 0x00040000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_82_S: u32 = 18;

        // Field:    [17] WRT_PROT_SEC_81
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_81: u32 = 0x00020000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_81_BITN: u32 = 17;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_81_M: u32 = 0x00020000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_81_S: u32 = 17;

        // Field:    [16] WRT_PROT_SEC_80
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_80: u32 = 0x00010000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_80_BITN: u32 = 16;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_80_M: u32 = 0x00010000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_80_S: u32 = 16;

        // Field:    [15] WRT_PROT_SEC_79
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_79: u32 = 0x00008000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_79_BITN: u32 = 15;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_79_M: u32 = 0x00008000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_79_S: u32 = 15;

        // Field:    [14] WRT_PROT_SEC_78
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_78: u32 = 0x00004000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_78_BITN: u32 = 14;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_78_M: u32 = 0x00004000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_78_S: u32 = 14;

        // Field:    [13] WRT_PROT_SEC_77
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_77: u32 = 0x00002000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_77_BITN: u32 = 13;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_77_M: u32 = 0x00002000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_77_S: u32 = 13;

        // Field:    [12] WRT_PROT_SEC_76
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_76: u32 = 0x00001000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_76_BITN: u32 = 12;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_76_M: u32 = 0x00001000;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_76_S: u32 = 12;

        // Field:    [11] WRT_PROT_SEC_75
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_75: u32 = 0x00000800;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_75_BITN: u32 = 11;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_75_M: u32 = 0x00000800;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_75_S: u32 = 11;

        // Field:    [10] WRT_PROT_SEC_74
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_74: u32 = 0x00000400;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_74_BITN: u32 = 10;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_74_M: u32 = 0x00000400;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_74_S: u32 = 10;

        // Field:     [9] WRT_PROT_SEC_73
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_73: u32 = 0x00000200;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_73_BITN: u32 = 9;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_73_M: u32 = 0x00000200;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_73_S: u32 = 9;

        // Field:     [8] WRT_PROT_SEC_72
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_72: u32 = 0x00000100;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_72_BITN: u32 = 8;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_72_M: u32 = 0x00000100;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_72_S: u32 = 8;

        // Field:     [7] WRT_PROT_SEC_71
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_71: u32 = 0x00000080;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_71_BITN: u32 = 7;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_71_M: u32 = 0x00000080;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_71_S: u32 = 7;

        // Field:     [6] WRT_PROT_SEC_70
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_70: u32 = 0x00000040;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_70_BITN: u32 = 6;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_70_M: u32 = 0x00000040;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_70_S: u32 = 6;

        // Field:     [5] WRT_PROT_SEC_69
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_69: u32 = 0x00000020;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_69_BITN: u32 = 5;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_69_M: u32 = 0x00000020;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_69_S: u32 = 5;

        // Field:     [4] WRT_PROT_SEC_68
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_68: u32 = 0x00000010;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_68_BITN: u32 = 4;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_68_M: u32 = 0x00000010;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_68_S: u32 = 4;

        // Field:     [3] WRT_PROT_SEC_67
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_67: u32 = 0x00000008;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_67_BITN: u32 = 3;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_67_M: u32 = 0x00000008;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_67_S: u32 = 3;

        // Field:     [2] WRT_PROT_SEC_66
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_66: u32 = 0x00000004;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_66_BITN: u32 = 2;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_66_M: u32 = 0x00000004;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_66_S: u32 = 2;

        // Field:     [1] WRT_PROT_SEC_65
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_65: u32 = 0x00000002;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_65_BITN: u32 = 1;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_65_M: u32 = 0x00000002;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_65_S: u32 = 1;

        // Field:     [0] WRT_PROT_SEC_64
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_64: u32 = 0x00000001;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_64_BITN: u32 = 0;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_64_M: u32 = 0x00000001;
        pub(super) const CCFG_CCFG_PROT_95_64_WRT_PROT_SEC_64_S: u32 = 0;

        //*****************************************************************************
        //
        // Register: CCFG_O_CCFG_PROT_127_96
        //
        //*****************************************************************************
        // Field:    [31] WRT_PROT_SEC_127
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_127: u32 = 0x80000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_127_BITN: u32 = 31;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_127_M: u32 = 0x80000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_127_S: u32 = 31;

        // Field:    [30] WRT_PROT_SEC_126
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_126: u32 = 0x40000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_126_BITN: u32 = 30;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_126_M: u32 = 0x40000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_126_S: u32 = 30;

        // Field:    [29] WRT_PROT_SEC_125
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_125: u32 = 0x20000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_125_BITN: u32 = 29;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_125_M: u32 = 0x20000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_125_S: u32 = 29;

        // Field:    [28] WRT_PROT_SEC_124
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_124: u32 = 0x10000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_124_BITN: u32 = 28;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_124_M: u32 = 0x10000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_124_S: u32 = 28;

        // Field:    [27] WRT_PROT_SEC_123
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_123: u32 = 0x08000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_123_BITN: u32 = 27;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_123_M: u32 = 0x08000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_123_S: u32 = 27;

        // Field:    [26] WRT_PROT_SEC_122
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_122: u32 = 0x04000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_122_BITN: u32 = 26;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_122_M: u32 = 0x04000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_122_S: u32 = 26;

        // Field:    [25] WRT_PROT_SEC_121
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_121: u32 = 0x02000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_121_BITN: u32 = 25;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_121_M: u32 = 0x02000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_121_S: u32 = 25;

        // Field:    [24] WRT_PROT_SEC_120
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_120: u32 = 0x01000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_120_BITN: u32 = 24;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_120_M: u32 = 0x01000000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_120_S: u32 = 24;

        // Field:    [23] WRT_PROT_SEC_119
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_119: u32 = 0x00800000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_119_BITN: u32 = 23;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_119_M: u32 = 0x00800000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_119_S: u32 = 23;

        // Field:    [22] WRT_PROT_SEC_118
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_118: u32 = 0x00400000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_118_BITN: u32 = 22;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_118_M: u32 = 0x00400000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_118_S: u32 = 22;

        // Field:    [21] WRT_PROT_SEC_117
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_117: u32 = 0x00200000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_117_BITN: u32 = 21;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_117_M: u32 = 0x00200000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_117_S: u32 = 21;

        // Field:    [20] WRT_PROT_SEC_116
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_116: u32 = 0x00100000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_116_BITN: u32 = 20;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_116_M: u32 = 0x00100000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_116_S: u32 = 20;

        // Field:    [19] WRT_PROT_SEC_115
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_115: u32 = 0x00080000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_115_BITN: u32 = 19;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_115_M: u32 = 0x00080000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_115_S: u32 = 19;

        // Field:    [18] WRT_PROT_SEC_114
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_114: u32 = 0x00040000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_114_BITN: u32 = 18;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_114_M: u32 = 0x00040000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_114_S: u32 = 18;

        // Field:    [17] WRT_PROT_SEC_113
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_113: u32 = 0x00020000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_113_BITN: u32 = 17;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_113_M: u32 = 0x00020000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_113_S: u32 = 17;

        // Field:    [16] WRT_PROT_SEC_112
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_112: u32 = 0x00010000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_112_BITN: u32 = 16;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_112_M: u32 = 0x00010000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_112_S: u32 = 16;

        // Field:    [15] WRT_PROT_SEC_111
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_111: u32 = 0x00008000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_111_BITN: u32 = 15;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_111_M: u32 = 0x00008000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_111_S: u32 = 15;

        // Field:    [14] WRT_PROT_SEC_110
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_110: u32 = 0x00004000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_110_BITN: u32 = 14;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_110_M: u32 = 0x00004000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_110_S: u32 = 14;

        // Field:    [13] WRT_PROT_SEC_109
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_109: u32 = 0x00002000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_109_BITN: u32 = 13;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_109_M: u32 = 0x00002000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_109_S: u32 = 13;

        // Field:    [12] WRT_PROT_SEC_108
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_108: u32 = 0x00001000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_108_BITN: u32 = 12;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_108_M: u32 = 0x00001000;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_108_S: u32 = 12;

        // Field:    [11] WRT_PROT_SEC_107
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_107: u32 = 0x00000800;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_107_BITN: u32 = 11;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_107_M: u32 = 0x00000800;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_107_S: u32 = 11;

        // Field:    [10] WRT_PROT_SEC_106
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_106: u32 = 0x00000400;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_106_BITN: u32 = 10;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_106_M: u32 = 0x00000400;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_106_S: u32 = 10;

        // Field:     [9] WRT_PROT_SEC_105
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_105: u32 = 0x00000200;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_105_BITN: u32 = 9;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_105_M: u32 = 0x00000200;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_105_S: u32 = 9;

        // Field:     [8] WRT_PROT_SEC_104
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_104: u32 = 0x00000100;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_104_BITN: u32 = 8;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_104_M: u32 = 0x00000100;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_104_S: u32 = 8;

        // Field:     [7] WRT_PROT_SEC_103
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_103: u32 = 0x00000080;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_103_BITN: u32 = 7;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_103_M: u32 = 0x00000080;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_103_S: u32 = 7;

        // Field:     [6] WRT_PROT_SEC_102
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_102: u32 = 0x00000040;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_102_BITN: u32 = 6;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_102_M: u32 = 0x00000040;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_102_S: u32 = 6;

        // Field:     [5] WRT_PROT_SEC_101
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_101: u32 = 0x00000020;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_101_BITN: u32 = 5;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_101_M: u32 = 0x00000020;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_101_S: u32 = 5;

        // Field:     [4] WRT_PROT_SEC_100
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_100: u32 = 0x00000010;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_100_BITN: u32 = 4;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_100_M: u32 = 0x00000010;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_100_S: u32 = 4;

        // Field:     [3] WRT_PROT_SEC_99
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_99: u32 = 0x00000008;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_99_BITN: u32 = 3;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_99_M: u32 = 0x00000008;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_99_S: u32 = 3;

        // Field:     [2] WRT_PROT_SEC_98
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_98: u32 = 0x00000004;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_98_BITN: u32 = 2;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_98_M: u32 = 0x00000004;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_98_S: u32 = 2;

        // Field:     [1] WRT_PROT_SEC_97
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_97: u32 = 0x00000002;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_97_BITN: u32 = 1;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_97_M: u32 = 0x00000002;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_97_S: u32 = 1;

        // Field:     [0] WRT_PROT_SEC_96
        //
        // 0: Sector protected
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_96: u32 = 0x00000001;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_96_BITN: u32 = 0;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_96_M: u32 = 0x00000001;
        pub(super) const CCFG_CCFG_PROT_127_96_WRT_PROT_SEC_96_S: u32 = 0;
    }

    #[allow(unused)]
    mod defaults {
        use super::hw_ccfg::*;

        //*****************************************************************************
        //
        // Introduction
        //
        // This file contains fields used by Boot ROM, startup code, and SW radio
        // stacks to configure chip behavior.
        //
        // Fields are documented in more details in hw_ccfg.h and CCFG.html in
        // DriverLib documentation (doc_overview.html -> CPU Domain Memory Map -> CCFG).
        //
        //*****************************************************************************

        //*****************************************************************************
        //
        // Set the values of the individual bit fields.
        //
        //*****************************************************************************

        //#####################################
        // Alternate DC/DC settings
        //#####################################

        // #ifndef SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING
        // #define SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING    0x0    // Alternate DC/DC setting enabled
        // // #define SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING 0x1    // Alternate DC/DC setting disabled
        // #endif
        pub(super) const SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING: u32 = 0x0;

        // #ifndef SET_CCFG_MODE_CONF_1_ALT_DCDC_VMIN
        // #define SET_CCFG_MODE_CONF_1_ALT_DCDC_VMIN              0x8        // 2.25V
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_1_ALT_DCDC_VMIN: u32 = 0x8; // 2.25V

        // #ifndef SET_CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN
        // #define SET_CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN         0x0        // Disable
        // // #define SET_CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN      0x1        // Enable
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN: u32 = 0x0;

        // #ifndef SET_CCFG_MODE_CONF_1_ALT_DCDC_IPEAK
        // #define SET_CCFG_MODE_CONF_1_ALT_DCDC_IPEAK             0x2        // 39mA
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_1_ALT_DCDC_IPEAK: u32 = 0x2; // 39mA

        //#####################################
        // XOSC override settings
        //#####################################

        // #ifndef SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR
        // // #define SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR     0x0        // Enable override
        // #define SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR        0x1        // Disable override
        // #endif
        pub(super) const SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR: u32 = 0x1;

        // #ifndef SET_CCFG_MODE_CONF_1_DELTA_IBIAS_INIT
        // #define SET_CCFG_MODE_CONF_1_DELTA_IBIAS_INIT           0x0        // Delta = 0
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_1_DELTA_IBIAS_INIT: u32 = 0x0;

        // #ifndef SET_CCFG_MODE_CONF_1_DELTA_IBIAS_OFFSET
        // #define SET_CCFG_MODE_CONF_1_DELTA_IBIAS_OFFSET         0x0        // Delta = 0
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_1_DELTA_IBIAS_OFFSET: u32 = 0x0;

        // #ifndef SET_CCFG_MODE_CONF_1_XOSC_MAX_START
        // #define SET_CCFG_MODE_CONF_1_XOSC_MAX_START             0x10       // 1600us
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_1_XOSC_MAX_START: u32 = 0x10;

        //#####################################
        // Power settings
        //#####################################

        // #ifndef SET_CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA
        // #define SET_CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA        0xF        // Signed delta value +1 to apply to the VDDR_TRIM_SLEEP target (0xF=-1=default=no compensation)
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA: u32 = 0xF;

        // #ifndef SET_CCFG_MODE_CONF_DCDC_RECHARGE
        // #define SET_CCFG_MODE_CONF_DCDC_RECHARGE                0x0        // Use the DC/DC during recharge in powerdown
        // // #define SET_CCFG_MODE_CONF_DCDC_RECHARGE             0x1        // Do not use the DC/DC during recharge in powerdown
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_DCDC_RECHARGE: u32 = 0x0;

        // #ifndef SET_CCFG_MODE_CONF_DCDC_ACTIVE
        // #define SET_CCFG_MODE_CONF_DCDC_ACTIVE                  0x0        // Use the DC/DC during active mode
        // // #define SET_CCFG_MODE_CONF_DCDC_ACTIVE               0x1        // Do not use the DC/DC during active mode
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_DCDC_ACTIVE: u32 = 0x0;

        // #ifndef SET_CCFG_MODE_CONF_VDDS_BOD_LEVEL
        // // #define SET_CCFG_MODE_CONF_VDDS_BOD_LEVEL            0x0        // VDDS BOD level is 2.0V
        // #define SET_CCFG_MODE_CONF_VDDS_BOD_LEVEL               0x1        // VDDS BOD level is 1.8V (or 1.65V for external regulator mode)
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_VDDS_BOD_LEVEL: u32 = 0x1;

        // #ifndef SET_CCFG_MODE_CONF_VDDR_CAP
        // #define SET_CCFG_MODE_CONF_VDDR_CAP                     0x3A       // Unsigned 8-bit integer representing the min. decoupling capacitance on VDDR in units of 100nF
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_VDDR_CAP: u32 = 0x3A;

        // #ifndef SET_CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC
        // #define SET_CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC           0x1        // Temperature compensation on VDDR sleep trim disabled (default)
        // // #define SET_CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC        0x0        // Temperature compensation on VDDR sleep trim enabled
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC: u32 = 0x1;

        //#####################################
        // Clock settings
        //#####################################

        // #ifndef SET_CCFG_MODE_CONF_SCLK_LF_OPTION
        // // #define SET_CCFG_MODE_CONF_SCLK_LF_OPTION            0x0        // LF clock derived from High Frequency XOSC
        // // #define SET_CCFG_MODE_CONF_SCLK_LF_OPTION            0x1        // External LF clock
        // #define SET_CCFG_MODE_CONF_SCLK_LF_OPTION               0x2        // LF XOSC
        // // #define SET_CCFG_MODE_CONF_SCLK_LF_OPTION            0x3        // LF RCOSC
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_SCLK_LF_OPTION: u32 = 0x2;

        // #ifndef SET_CCFG_MODE_CONF_XOSC_CAP_MOD
        // // #define SET_CCFG_MODE_CONF_XOSC_CAP_MOD              0x0        // Apply cap-array delta
        // #define SET_CCFG_MODE_CONF_XOSC_CAP_MOD                 0x1        // Don't apply cap-array delta
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_XOSC_CAP_MOD: u32 = 0x1;

        // #ifndef SET_CCFG_MODE_CONF_XOSC_CAPARRAY_DELTA
        // #define SET_CCFG_MODE_CONF_XOSC_CAPARRAY_DELTA          0xFF       // Signed 8-bit value, directly modifying trimmed XOSC cap-array value
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_XOSC_CAPARRAY_DELTA: u32 = 0xFF;

        // #ifndef SET_CCFG_EXT_LF_CLK_DIO
        // #define SET_CCFG_EXT_LF_CLK_DIO                         0x01       // DIO number if using external LF clock
        // #endif
        pub(super) const SET_CCFG_EXT_LF_CLK_DIO: u32 = 0x01;

        // #ifndef SET_CCFG_EXT_LF_CLK_RTC_INCREMENT
        // #define SET_CCFG_EXT_LF_CLK_RTC_INCREMENT               0x800000   // RTC increment representing the external LF clock frequency
        // #endif
        pub(super) const SET_CCFG_EXT_LF_CLK_RTC_INCREMENT: u32 = 0x800000;

        //#####################################
        // Special HF clock source setting
        //#####################################
        // #ifndef SET_CCFG_MODE_CONF_XOSC_FREQ
        // // #define SET_CCFG_MODE_CONF_XOSC_FREQ                 0x1        // Use BAW oscillator as HF source (if executed on a BAW chip, otherwise using default (=3))
        // // #define SET_CCFG_MODE_CONF_XOSC_FREQ                 0x2        // HF source is a 48 MHz xtal
        // #define SET_CCFG_MODE_CONF_XOSC_FREQ                    0x3        // HF source is a 24 MHz xtal (default)
        // #endif
        pub(super) const SET_CCFG_MODE_CONF_XOSC_FREQ: u32 = 0x3;

        //#####################################
        // Bootloader settings
        //#####################################

        // #ifndef PLATFORM_CC26XX_BOOTLOADER_DIO
        // #define PLATFORM_CC26XX_BOOTLOADER_DIO -1
        // #endif
        // pub(super) const PLATFORM_CC26XX_BOOTLOADER_DIO: u32 = u32::MAX;
        pub(super) const PLATFORM_CC26XX_BOOTLOADER_DIO: u32 = 0x0B; // taken from valid .hex

        // #ifndef PLATFORM_CC26XX_BOOTLOADER_DIO_LEVEL
        // #define PLATFORM_CC26XX_BOOTLOADER_DIO_LEVEL 0
        // #endif
        pub(super) const PLATFORM_CC26XX_BOOTLOADER_DIO_LEVEL: u32 = 0;

        // #ifndef SET_CCFG_BL_CONFIG_BOOTLOADER_ENABLE
        // // #define SET_CCFG_BL_CONFIG_BOOTLOADER_ENABLE         0x00       // Disable ROM boot loader
        // #define SET_CCFG_BL_CONFIG_BOOTLOADER_ENABLE            0xC5       // Enable ROM boot loader
        // #endif
        pub(super) const SET_CCFG_BL_CONFIG_BOOTLOADER_ENABLE: u32 = 0xC5;

        // #ifndef SET_CCFG_BL_CONFIG_BL_LEVEL
        // #define SET_CCFG_BL_CONFIG_BL_LEVEL                  PLATFORM_CC26XX_BOOTLOADER_DIO_LEVEL        // Active low to open boot loader backdoor
        // //#define SET_CCFG_BL_CONFIG_BL_LEVEL                     0x1        // Active high to open boot loader backdoor
        // #endif
        pub(super) const SET_CCFG_BL_CONFIG_BL_LEVEL: u32 = PLATFORM_CC26XX_BOOTLOADER_DIO_LEVEL;

        // #ifndef SET_CCFG_BL_CONFIG_BL_PIN_NUMBER
        // #define SET_CCFG_BL_CONFIG_BL_PIN_NUMBER                PLATFORM_CC26XX_BOOTLOADER_DIO       // DIO number for boot loader backdoor
        // #endif
        pub(super) const SET_CCFG_BL_CONFIG_BL_PIN_NUMBER: u32 = PLATFORM_CC26XX_BOOTLOADER_DIO;

        // #ifndef SET_CCFG_BL_CONFIG_BL_ENABLE
        // #if PLATFORM_CC26XX_BOOTLOADER_DIO == -1
        // #define SET_CCFG_BL_CONFIG_BL_ENABLE                    0xFF       // Disabled boot loader backdoor
        // #else
        // #define SET_CCFG_BL_CONFIG_BL_ENABLE                 0xC5       // Enabled boot loader backdoor
        // #endif
        // #endif
        pub(super) const SET_CCFG_BL_CONFIG_BL_ENABLE: u32 = if PLATFORM_CC26XX_BOOTLOADER_DIO == u32::MAX {
            0xFF
        } else {
            0xC5
        };

        //#####################################
        // Debug access settings
        //#####################################

        // #ifndef SET_CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE
        // // #define SET_CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE        0x00       // Disable unlocking of TI FA option.
        // #define SET_CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE           0xC5       // Enable unlocking of TI FA option with the unlock code
        // #endif
        pub(super) const SET_CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE: u32 = 0xC5;

        // #ifndef SET_CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE
        // // #define SET_CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE       0x00       // Access disabled
        // #define SET_CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE          0xC5       // Access enabled if also enabled in FCFG
        // #endif
        pub(super) const SET_CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE: u32 = 0xC5;

        // #ifndef SET_CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE
        // // #define SET_CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE      0x00       // Access disabled
        // #define SET_CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE         0xC5       // Access enabled if also enabled in FCFG
        // #endif
        pub(super) const SET_CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE: u32 = 0xC5;

        // #ifndef SET_CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE
        // // #define SET_CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE      0x00       // Access disabled
        // #define SET_CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE         0xC5       // Access enabled if also enabled in FCFG
        // #endif
        pub(super) const SET_CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE: u32 = 0xC5;

        // #ifndef SET_CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE
        // // #define SET_CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE    0x00       // Access disabled
        // #define SET_CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE       0xC5       // Access enabled if also enabled in FCFG
        // #endif
        pub(super) const SET_CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE: u32 = 0xC5;

        // #ifndef SET_CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE
        // // #define SET_CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE    0x00       // Access disabled
        // #define SET_CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE       0xC5       // Access enabled if also enabled in FCFG
        // #endif
        pub(super) const SET_CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE: u32 = 0xC5;

        // #ifndef SET_CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE
        // // #define SET_CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE       0x00       // Access disabled
        // #define SET_CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE          0xC5       // Access enabled if also enabled in FCFG
        // #endif
        pub(super) const SET_CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE: u32 = 0xC5;

        //#####################################
        // Alternative IEEE 802.15.4 MAC address
        //#####################################
        // #ifndef SET_CCFG_IEEE_MAC_0
        // #define SET_CCFG_IEEE_MAC_0                             0xFFFFFFFF // Bits [31:0]
        // #endif
        pub(super) const SET_CCFG_IEEE_MAC_0: u32 = 0xFFFFFFFF;

        // #ifndef SET_CCFG_IEEE_MAC_1
        // #define SET_CCFG_IEEE_MAC_1                             0xFFFFFFFF // Bits [63:32]
        // #endif
        pub(super) const SET_CCFG_IEEE_MAC_1: u32 = 0xFFFFFFFF;

        //#####################################
        // Alternative BLE address
        //#####################################
        // #ifndef SET_CCFG_IEEE_BLE_0
        // #define SET_CCFG_IEEE_BLE_0                             0xFFFFFFFF // Bits [31:0]
        // #endif
        pub(super) const SET_CCFG_IEEE_BLE_0: u32 = 0xFFFFFFFF;

        // #ifndef SET_CCFG_IEEE_BLE_1
        // #define SET_CCFG_IEEE_BLE_1                             0xFFFFFFFF // Bits [63:32]
        // #endif
        pub(super) const SET_CCFG_IEEE_BLE_1: u32 = 0xFFFFFFFF;

        //#####################################
        // Flash erase settings
        //#####################################

        // #ifndef SET_CCFG_ERASE_CONF_CHIP_ERASE_DIS_N
        // // #define SET_CCFG_ERASE_CONF_CHIP_ERASE_DIS_N         0x0        // Any chip erase request detected during boot will be ignored
        // #define SET_CCFG_ERASE_CONF_CHIP_ERASE_DIS_N            0x1        // Any chip erase request detected during boot will be performed by the boot FW
        // #endif
        pub(super) const SET_CCFG_ERASE_CONF_CHIP_ERASE_DIS_N: u32 = 0x1;

        // #ifndef SET_CCFG_ERASE_CONF_BANK_ERASE_DIS_N
        // // #define SET_CCFG_ERASE_CONF_BANK_ERASE_DIS_N         0x0        // Disable the boot loader bank erase function
        // #define SET_CCFG_ERASE_CONF_BANK_ERASE_DIS_N            0x1        // Enable the boot loader bank erase function
        // #endif
        pub(super) const SET_CCFG_ERASE_CONF_BANK_ERASE_DIS_N: u32 = 0x1;

        //#####################################
        // Flash image valid
        //#####################################
        // #ifndef SET_CCFG_IMAGE_VALID_CONF_IMAGE_VALID
        // #define SET_CCFG_IMAGE_VALID_CONF_IMAGE_VALID           0x00000000 // Flash image is valid
        // // #define SET_CCFG_IMAGE_VALID_CONF_IMAGE_VALID        <non-zero> // Flash image is invalid, call bootloader
        // #endif
        pub(super) const SET_CCFG_IMAGE_VALID_CONF_IMAGE_VALID: u32 = 0x00000000;

        //#####################################
        // Flash sector write protection
        //#####################################
        // #ifndef SET_CCFG_CCFG_PROT_31_0
        // #define SET_CCFG_CCFG_PROT_31_0                         0xFFFFFFFF
        // #endif
        pub(super) const SET_CCFG_CCFG_PROT_31_0: u32 = 0xFFFFFFFF;

        // #ifndef SET_CCFG_CCFG_PROT_63_32
        // #define SET_CCFG_CCFG_PROT_63_32                        0xFFFFFFFF
        // #endif
        pub(super) const SET_CCFG_CCFG_PROT_63_32: u32 = 0xFFFFFFFF;

        // #ifndef SET_CCFG_CCFG_PROT_95_64
        // #define SET_CCFG_CCFG_PROT_95_64                        0xFFFFFFFF
        // #endif
        pub(super) const SET_CCFG_CCFG_PROT_95_64: u32 = 0xFFFFFFFF;

        // #ifndef SET_CCFG_CCFG_PROT_127_96
        // #define SET_CCFG_CCFG_PROT_127_96                       0xFFFFFFFF
        // #endif
        pub(super) const SET_CCFG_CCFG_PROT_127_96: u32 = 0xFFFFFFFF;

        //#####################################
        // Select between cache or GPRAM
        //#####################################
        // #ifndef SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM
        // // #define SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM        0x0        // Cache is disabled and GPRAM is available at 0x11000000-0x11001FFF
        // #define SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM           0x1        // Cache is enabled and GPRAM is disabled (unavailable)
        // #endif
        pub(super) const SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM: u32 = 0x1;

        //*****************************************************************************
        //
        // CCFG values that should not be modified.
        //
        //*****************************************************************************
        // #define SET_CCFG_SIZE_AND_DIS_FLAGS_SIZE_OF_CCFG        0x0058
        pub(super) const SET_CCFG_SIZE_AND_DIS_FLAGS_SIZE_OF_CCFG: u32 = 0x0058;
        // #define SET_CCFG_SIZE_AND_DIS_FLAGS_DISABLE_FLAGS       0x3FFF
        pub(super) const SET_CCFG_SIZE_AND_DIS_FLAGS_DISABLE_FLAGS: u32 = 0x3FFF;

        // #define SET_CCFG_MODE_CONF_VDDR_EXT_LOAD                0x1
        pub(super) const SET_CCFG_MODE_CONF_VDDR_EXT_LOAD: u32 = 0x1;
        // #define SET_CCFG_MODE_CONF_RTC_COMP                     0x1
        pub(super) const SET_CCFG_MODE_CONF_RTC_COMP: u32 = 0x1;
        // #define SET_CCFG_MODE_CONF_HF_COMP                      0x1
        pub(super) const SET_CCFG_MODE_CONF_HF_COMP: u32 = 0x1;

        // #define SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TP45              0xFF
        pub(super) const SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TP45: u32 = 0xFF;
        // #define SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TP25              0xFF
        pub(super) const SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TP25: u32 = 0xFF;
        // #define SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TP5               0xFF
        pub(super) const SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TP5: u32 = 0xFF;
        // #define SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TM15              0xFF
        pub(super) const SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TM15: u32 = 0xFF;

        // #define SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP125             0xFF
        pub(super) const SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP125: u32 = 0xFF;
        // #define SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP105             0xFF
        pub(super) const SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP105: u32 = 0xFF;
        // #define SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP85              0xFF
        pub(super) const SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP85: u32 = 0xFF;
        // #define SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP65              0xFF
        pub(super) const SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP65: u32 = 0xFF;

        // #define SET_CCFG_RTC_OFFSET_RTC_COMP_P0                 0xFFFF
        pub(super) const SET_CCFG_RTC_OFFSET_RTC_COMP_P0: u32 = 0xFFFF;
        // #define SET_CCFG_RTC_OFFSET_RTC_COMP_P1                 0xFF
        pub(super) const SET_CCFG_RTC_OFFSET_RTC_COMP_P1: u32 = 0xFF;
        // #define SET_CCFG_RTC_OFFSET_RTC_COMP_P2                 0xFF
        pub(super) const SET_CCFG_RTC_OFFSET_RTC_COMP_P2: u32 = 0xFF;

        // #define SET_CCFG_FREQ_OFFSET_HF_COMP_P0                 0xFFFF
        pub(super) const SET_CCFG_FREQ_OFFSET_HF_COMP_P0: u32 = 0xFFFF;
        // #define SET_CCFG_FREQ_OFFSET_HF_COMP_P1                 0xFF
        pub(super) const SET_CCFG_FREQ_OFFSET_HF_COMP_P1: u32 = 0xFF;
        // #define SET_CCFG_FREQ_OFFSET_HF_COMP_P2                 0xFF
        pub(super) const SET_CCFG_FREQ_OFFSET_HF_COMP_P2: u32 = 0xFF;

        //*****************************************************************************
        //
        // Concatenate bit fields to words.
        // DO NOT EDIT!
        //
        //*****************************************************************************
        pub(super) const DEFAULT_CCFG_O_EXT_LF_CLK: u32 = ((SET_CCFG_EXT_LF_CLK_DIO << CCFG_EXT_LF_CLK_DIO_S)
            | !CCFG_EXT_LF_CLK_DIO_M)
            & ((SET_CCFG_EXT_LF_CLK_RTC_INCREMENT << CCFG_EXT_LF_CLK_RTC_INCREMENT_S)
                | !CCFG_EXT_LF_CLK_RTC_INCREMENT_M);

        pub(super) const DEFAULT_CCFG_MODE_CONF_1: u32 = ((SET_CCFG_MODE_CONF_1_ALT_DCDC_VMIN
            << CCFG_MODE_CONF_1_ALT_DCDC_VMIN_S)
            | !CCFG_MODE_CONF_1_ALT_DCDC_VMIN_M)
            & ((SET_CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN << CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN_S)
                | !CCFG_MODE_CONF_1_ALT_DCDC_DITHER_EN_M)
            & ((SET_CCFG_MODE_CONF_1_ALT_DCDC_IPEAK << CCFG_MODE_CONF_1_ALT_DCDC_IPEAK_S)
                | !CCFG_MODE_CONF_1_ALT_DCDC_IPEAK_M)
            & ((SET_CCFG_MODE_CONF_1_DELTA_IBIAS_INIT << CCFG_MODE_CONF_1_DELTA_IBIAS_INIT_S)
                | !CCFG_MODE_CONF_1_DELTA_IBIAS_INIT_M)
            & ((SET_CCFG_MODE_CONF_1_DELTA_IBIAS_OFFSET << CCFG_MODE_CONF_1_DELTA_IBIAS_OFFSET_S)
                | !CCFG_MODE_CONF_1_DELTA_IBIAS_OFFSET_M)
            & ((SET_CCFG_MODE_CONF_1_XOSC_MAX_START << CCFG_MODE_CONF_1_XOSC_MAX_START_S)
                | !CCFG_MODE_CONF_1_XOSC_MAX_START_M);

        pub(super) const DEFAULT_CCFG_SIZE_AND_DIS_FLAGS: u32 = ((SET_CCFG_SIZE_AND_DIS_FLAGS_SIZE_OF_CCFG
            << CCFG_SIZE_AND_DIS_FLAGS_SIZE_OF_CCFG_S)
            | !CCFG_SIZE_AND_DIS_FLAGS_SIZE_OF_CCFG_M)
            & ((SET_CCFG_SIZE_AND_DIS_FLAGS_DISABLE_FLAGS << CCFG_SIZE_AND_DIS_FLAGS_DISABLE_FLAGS_S)
                | !CCFG_SIZE_AND_DIS_FLAGS_DISABLE_FLAGS_M)
            & ((SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM << CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM_S)
                | !CCFG_SIZE_AND_DIS_FLAGS_DIS_GPRAM_M)
            & ((SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING << CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING_S)
                | !CCFG_SIZE_AND_DIS_FLAGS_DIS_ALT_DCDC_SETTING_M)
            & ((SET_CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR << CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR_S)
                | !CCFG_SIZE_AND_DIS_FLAGS_DIS_XOSC_OVR_M);

        pub(super) const DEFAULT_CCFG_MODE_CONF: u32 = ((SET_CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA
            << CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA_S)
            | !CCFG_MODE_CONF_VDDR_TRIM_SLEEP_DELTA_M)
            & ((SET_CCFG_MODE_CONF_DCDC_RECHARGE << CCFG_MODE_CONF_DCDC_RECHARGE_S) | !CCFG_MODE_CONF_DCDC_RECHARGE_M)
            & ((SET_CCFG_MODE_CONF_DCDC_ACTIVE << CCFG_MODE_CONF_DCDC_ACTIVE_S) | !CCFG_MODE_CONF_DCDC_ACTIVE_M)
            & ((SET_CCFG_MODE_CONF_VDDR_EXT_LOAD << CCFG_MODE_CONF_VDDR_EXT_LOAD_S) | !CCFG_MODE_CONF_VDDR_EXT_LOAD_M)
            & ((SET_CCFG_MODE_CONF_VDDS_BOD_LEVEL << CCFG_MODE_CONF_VDDS_BOD_LEVEL_S)
                | !CCFG_MODE_CONF_VDDS_BOD_LEVEL_M)
            & ((SET_CCFG_MODE_CONF_SCLK_LF_OPTION << CCFG_MODE_CONF_SCLK_LF_OPTION_S)
                | !CCFG_MODE_CONF_SCLK_LF_OPTION_M)
            & ((SET_CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC << CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC_S)
                | !CCFG_MODE_CONF_VDDR_TRIM_SLEEP_TC_M)
            & ((SET_CCFG_MODE_CONF_RTC_COMP << CCFG_MODE_CONF_RTC_COMP_S) | !CCFG_MODE_CONF_RTC_COMP_M)
            & ((SET_CCFG_MODE_CONF_XOSC_FREQ << CCFG_MODE_CONF_XOSC_FREQ_S) | !CCFG_MODE_CONF_XOSC_FREQ_M)
            & ((SET_CCFG_MODE_CONF_XOSC_CAP_MOD << CCFG_MODE_CONF_XOSC_CAP_MOD_S) | !CCFG_MODE_CONF_XOSC_CAP_MOD_M)
            & ((SET_CCFG_MODE_CONF_HF_COMP << CCFG_MODE_CONF_HF_COMP_S) | !CCFG_MODE_CONF_HF_COMP_M)
            & ((SET_CCFG_MODE_CONF_XOSC_CAPARRAY_DELTA << CCFG_MODE_CONF_XOSC_CAPARRAY_DELTA_S)
                | !CCFG_MODE_CONF_XOSC_CAPARRAY_DELTA_M)
            & ((SET_CCFG_MODE_CONF_VDDR_CAP << CCFG_MODE_CONF_VDDR_CAP_S) | !CCFG_MODE_CONF_VDDR_CAP_M);

        pub(super) const DEFAULT_CCFG_VOLT_LOAD_0: u32 = ((SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TP45
            << CCFG_VOLT_LOAD_0_VDDR_EXT_TP45_S)
            | !CCFG_VOLT_LOAD_0_VDDR_EXT_TP45_M)
            & ((SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TP25 << CCFG_VOLT_LOAD_0_VDDR_EXT_TP25_S)
                | !CCFG_VOLT_LOAD_0_VDDR_EXT_TP25_M)
            & ((SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TP5 << CCFG_VOLT_LOAD_0_VDDR_EXT_TP5_S)
                | !CCFG_VOLT_LOAD_0_VDDR_EXT_TP5_M)
            & ((SET_CCFG_VOLT_LOAD_0_VDDR_EXT_TM15 << CCFG_VOLT_LOAD_0_VDDR_EXT_TM15_S)
                | !CCFG_VOLT_LOAD_0_VDDR_EXT_TM15_M);

        pub(super) const DEFAULT_CCFG_VOLT_LOAD_1: u32 = ((SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP125
            << CCFG_VOLT_LOAD_1_VDDR_EXT_TP125_S)
            | !CCFG_VOLT_LOAD_1_VDDR_EXT_TP125_M)
            & ((SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP105 << CCFG_VOLT_LOAD_1_VDDR_EXT_TP105_S)
                | !CCFG_VOLT_LOAD_1_VDDR_EXT_TP105_M)
            & ((SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP85 << CCFG_VOLT_LOAD_1_VDDR_EXT_TP85_S)
                | !CCFG_VOLT_LOAD_1_VDDR_EXT_TP85_M)
            & ((SET_CCFG_VOLT_LOAD_1_VDDR_EXT_TP65 << CCFG_VOLT_LOAD_1_VDDR_EXT_TP65_S)
                | !CCFG_VOLT_LOAD_1_VDDR_EXT_TP65_M);

        pub(super) const DEFAULT_CCFG_RTC_OFFSET: u32 =
            ((SET_CCFG_RTC_OFFSET_RTC_COMP_P0 << CCFG_RTC_OFFSET_RTC_COMP_P0_S) | !CCFG_RTC_OFFSET_RTC_COMP_P0_M)
                & ((SET_CCFG_RTC_OFFSET_RTC_COMP_P1 << CCFG_RTC_OFFSET_RTC_COMP_P1_S) | !CCFG_RTC_OFFSET_RTC_COMP_P1_M)
                & ((SET_CCFG_RTC_OFFSET_RTC_COMP_P2 << CCFG_RTC_OFFSET_RTC_COMP_P2_S) | !CCFG_RTC_OFFSET_RTC_COMP_P2_M);

        pub(super) const DEFAULT_CCFG_FREQ_OFFSET: u32 =
            ((SET_CCFG_FREQ_OFFSET_HF_COMP_P0 << CCFG_FREQ_OFFSET_HF_COMP_P0_S) | !CCFG_FREQ_OFFSET_HF_COMP_P0_M)
                & ((SET_CCFG_FREQ_OFFSET_HF_COMP_P1 << CCFG_FREQ_OFFSET_HF_COMP_P1_S) | !CCFG_FREQ_OFFSET_HF_COMP_P1_M)
                & ((SET_CCFG_FREQ_OFFSET_HF_COMP_P2 << CCFG_FREQ_OFFSET_HF_COMP_P2_S) | !CCFG_FREQ_OFFSET_HF_COMP_P2_M);

        pub(super) const DEFAULT_CCFG_IEEE_MAC_0: u32 = SET_CCFG_IEEE_MAC_0;
        pub(super) const DEFAULT_CCFG_IEEE_MAC_1: u32 = SET_CCFG_IEEE_MAC_1;
        pub(super) const DEFAULT_CCFG_IEEE_BLE_0: u32 = SET_CCFG_IEEE_BLE_0;
        pub(super) const DEFAULT_CCFG_IEEE_BLE_1: u32 = SET_CCFG_IEEE_BLE_1;

        pub(super) const DEFAULT_CCFG_BL_CONFIG: u32 = ((SET_CCFG_BL_CONFIG_BOOTLOADER_ENABLE
            << CCFG_BL_CONFIG_BOOTLOADER_ENABLE_S)
            | !CCFG_BL_CONFIG_BOOTLOADER_ENABLE_M)
            & ((SET_CCFG_BL_CONFIG_BL_LEVEL << CCFG_BL_CONFIG_BL_LEVEL_S) | !CCFG_BL_CONFIG_BL_LEVEL_M)
            & ((SET_CCFG_BL_CONFIG_BL_PIN_NUMBER << CCFG_BL_CONFIG_BL_PIN_NUMBER_S) | !CCFG_BL_CONFIG_BL_PIN_NUMBER_M)
            & ((SET_CCFG_BL_CONFIG_BL_ENABLE << CCFG_BL_CONFIG_BL_ENABLE_S) | !CCFG_BL_CONFIG_BL_ENABLE_M);

        pub(super) const DEFAULT_CCFG_ERASE_CONF: u32 = ((SET_CCFG_ERASE_CONF_CHIP_ERASE_DIS_N
            << CCFG_ERASE_CONF_CHIP_ERASE_DIS_N_S)
            | !CCFG_ERASE_CONF_CHIP_ERASE_DIS_N_M)
            & ((SET_CCFG_ERASE_CONF_BANK_ERASE_DIS_N << CCFG_ERASE_CONF_BANK_ERASE_DIS_N_S)
                | !CCFG_ERASE_CONF_BANK_ERASE_DIS_N_M);

        pub(super) const DEFAULT_CCFG_CCFG_TI_OPTIONS: u32 = ((SET_CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE
            << CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE_S)
            | !CCFG_CCFG_TI_OPTIONS_TI_FA_ENABLE_M);

        pub(super) const DEFAULT_CCFG_CCFG_TAP_DAP_0: u32 = ((SET_CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE
            << CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE_S)
            | !CCFG_CCFG_TAP_DAP_0_CPU_DAP_ENABLE_M)
            & ((SET_CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE << CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE_S)
                | !CCFG_CCFG_TAP_DAP_0_PRCM_TAP_ENABLE_M)
            & ((SET_CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE << CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE_S)
                | !CCFG_CCFG_TAP_DAP_0_TEST_TAP_ENABLE_M);

        pub(super) const DEFAULT_CCFG_CCFG_TAP_DAP_1: u32 = ((SET_CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE
            << CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE_S)
            | !CCFG_CCFG_TAP_DAP_1_PBIST2_TAP_ENABLE_M)
            & ((SET_CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE << CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE_S)
                | !CCFG_CCFG_TAP_DAP_1_PBIST1_TAP_ENABLE_M)
            & ((SET_CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE << CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE_S)
                | !CCFG_CCFG_TAP_DAP_1_WUC_TAP_ENABLE_M);

        pub(super) const DEFAULT_CCFG_IMAGE_VALID_CONF: u32 = ((SET_CCFG_IMAGE_VALID_CONF_IMAGE_VALID
            << CCFG_IMAGE_VALID_CONF_IMAGE_VALID_S)
            | !CCFG_IMAGE_VALID_CONF_IMAGE_VALID_M);

        pub(super) const DEFAULT_CCFG_CCFG_PROT_31_0: u32 = SET_CCFG_CCFG_PROT_31_0;
        pub(super) const DEFAULT_CCFG_CCFG_PROT_63_32: u32 = SET_CCFG_CCFG_PROT_63_32;
        pub(super) const DEFAULT_CCFG_CCFG_PROT_95_64: u32 = SET_CCFG_CCFG_PROT_95_64;
        pub(super) const DEFAULT_CCFG_CCFG_PROT_127_96: u32 = SET_CCFG_CCFG_PROT_127_96;
    }
}

#[allow(unused)]
pub(crate) struct Ccfg {
    ccfg: pac::CCFG::CCFG,
}

impl Ccfg {
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self { ccfg: pac::CCFG }
    }

    #[allow(unused)]
    pub(crate) fn ieee_mac(&self) -> Option<u64> {
        let mac = ((self.ccfg.IEEE_MAC_1().read().ADDR() as u64) << 32) | self.ccfg.IEEE_MAC_0().read().ADDR() as u64;

        // If mac has all bits set to 1, then it's unset.
        (mac != (!0)).then_some(mac)
    }
}
