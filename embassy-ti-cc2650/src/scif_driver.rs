use crate::driverlib;
use crate::pac;
use core::cell::Cell;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use ti_cc2650_pac::AUX_EVCTL::regs::VECFLAGS;
use ti_cc2650_pac::AUX_EVCTL::vals;
use vcell::VolatileCell;

macro_rules! safe_packed_ref {
    ($place:expr) => {
        core::ptr::addr_of!($place).as_ref().unwrap_unchecked()
    };
}

/// Sensor Controller Interface function call result
#[derive(Debug)]
#[repr(u32)]
pub(crate) enum SCIFResult {
    /// Call succeeded
    Success = 0,
    /// Not ready (previous non-blocking call is still running)
    NotReady = 1,
    /// Illegal operation
    IllegalOperation = 2,
}

/// Task data structure types
#[repr(u32)]
pub(crate) enum SCIFTaskStructType {
    /// Task configuration data structure (Sensor Controller read-only)
    SCIFStructCfg = 0,
    /// Task input data structure
    SCIFStructInput = 1,
    /// Task output data structure
    SCIFStructOutput = 2,
    /// Task state data structure
    SCIFStructState = 3,
}

///  function pointer type: " func(Scif)"
type SCIFVfptr = unsafe fn(&Scif);

/// Sensor Controller internal data (located in AUX RAM)
#[repr(packed)]
pub(crate) struct SCIFIntData {
    /// ID of currently executed Sensor Controller task
    #[allow(unused)]
    task_id: VolatileCell<u16>,
    /// Pending input/output data alert (LSB = normal exchange, MSB = overflow or underflow)
    #[allow(unused)]
    bv_task_io_alert: VolatileCell<u16>,
    /// ALERT interrupt generation mask
    #[allow(unused)]
    alert_gen_mask: VolatileCell<u16>,
}

/// Sensor Controller generic task control (located in AUX RAM)
#[repr(packed)]
pub(crate) struct SCIFTaskCtrl {
    /// Indicates which tasks are currently active (only valid while ready)
    bv_active_tasks: VolatileCell<u16>,
    /// Input/output data alert (LSB = normal exchange, MSB = overflow or underflow)
    #[allow(unused)]
    bv_task_io_alert: VolatileCell<u16>,
    /// Requests tasks to start
    bv_task_initialize_req: VolatileCell<u16>,
    /// Requests tasks to execute once immediately
    bv_task_execute_req: VolatileCell<u16>,
    /// Requests tasks to stop
    bv_task_terminate_req: VolatileCell<u16>,
}

/// Driver internal data (located in main RAM, not shared with the Sensor Controller)
#[derive(Clone, Copy)]
pub(crate) struct SCIFData {
    /// Sensor Controller internal data (located in AUX RAM)
    #[allow(unused)]
    pub(crate) int_data: &'static SCIFIntData,
    /// Sensor Controller task generic control (located in AUX RAM)
    pub(crate) task_ctrl: &'static SCIFTaskCtrl,
    /// Pointer to the task execution scheduling table
    pub(crate) task_execute_schedule: *mut u16,

    /// AUX RAM image word array
    pub(crate) aux_ram_image: &'static [u16],

    /// Look-up table that converts from AUX I/O index to MCU IOCFG offset
    pub(crate) task_data_struct_info_lut: &'static [u32],
    /// Look-up table of data structure information for each task
    pub(crate) aux_io_index_to_mcu_iocfg_offset_lut: &'static [u8],

    /// Pointer to the project-specific hardware initialization function
    pub(crate) fptr_task_resource_init: SCIFVfptr,
    /// Pointer to the project-specific hardware uninitialization function
    pub(crate) fptr_task_resource_uninit: SCIFVfptr,
}

/// I/O pin mode: Output
pub(crate) const AUXIOMODE_OUTPUT: u32 = 0x00000000;
/// I/O pin mode: Input, active
pub(crate) const AUXIOMODE_INPUT: u32 = 0x00010001;
/// I/O pin mode: Input, inactive
pub(crate) const AUXIOMODE_INPUT_IDLE: u32 = 0x00000001;
/// I/O pin mode: Open drain (driven low, pulled high)
pub(crate) const AUXIOMODE_OPEN_DRAIN: u32 = 0x00000002;
/// I/O pin mode: Open drain + input (driven low, pulled high, input buffer enabled)
pub(crate) const AUXIOMODE_OPEN_DRAIN_WITH_INPUT: u32 = 0x00010002;
/// I/O pin mode: Open source (driven high, pulled low)
pub(crate) const AUXIOMODE_OPEN_SOURCE: u32 = 0x00000003;
/// I/O pin mode: Open source + input (driven high, pulled low, input buffer enabled)
pub(crate) const AUXIOMODE_OPEN_SOURCE_WITH_INPUT: u32 = 0x00010003;
/// I/O pin mode: Analog
pub(crate) const AUXIOMODE_ANALOG: u32 = 0x00000001;

/// Task data structure buffer control: Size (in bytes)
const SCIF_TASK_STRUCT_CTRL_SIZE: u32 = 3 * core::mem::size_of::<u16>() as u32;
/// Task data structure buffer control: Sensor Controller Engine's pointer negative offset (ref. struct start)
const SCIF_TASK_STRUCT_CTRL_SCE_ADDR_BACK_OFFSET: u32 = 3 * core::mem::size_of::<u16>() as u32;
/// Task data structure buffer control: Driver/MCU's pointer negative offset (ref. struct start)
const SCIF_TASK_STRUCT_CTRL_MCU_ADDR_BACK_OFFSET: u32 = 2 * core::mem::size_of::<u16>() as u32;

static SCIF_READY: AtomicBool = AtomicBool::new(false);

/// The READY interrupt is implemented using INT_AON_AUX_SWEV0
const INT_SCIF_CTRL_READY: pac::Interrupt = pac::Interrupt::UART1;
/// The ALERT interrupt is implemented using INT_AON_AUX_SWEV1
const INT_SCIF_TASK_ALERT: pac::Interrupt = pac::Interrupt::AON_EVENT;

pub(crate) struct Scif {
    pub(crate) aon_wuc: pac::AON_WUC::AON_WUC,
    #[allow(unused)]
    pub(crate) aux_aiodio0: pac::AUX_AIODIO0::AUX_AIODIO0,
    #[allow(unused)]
    pub(crate) aux_aiodio1: pac::AUX_AIODIO1::AUX_AIODIO1,
    pub(crate) aux_evctl: pac::AUX_EVCTL::AUX_EVCTL,
    pub(crate) aux_sce: pac::AUX_SCE::AUX_SCE,
    pub(crate) aux_timer: pac::AUX_TIMER::AUX_TIMER,
    pub(crate) aux_wuc: pac::AUX_WUC::AUX_WUC,

    /// Driver internal data (located in MCU domain RAM, not shared with the Sensor Controller)
    scif_data: Cell<Option<&'static SCIFData>>,
    /// Bit-vector indicating tasks with potentially modified input/output/state data structures
    bv_dirty_tasks: Cell<u16>,
    last_aux_ram_image: Cell<Option<&'static [u16]>>,
}

impl Scif {
    fn scif_data(&self) -> &'static SCIFData {
        self.scif_data.get().unwrap()
    }

    pub(crate) fn new() -> Self {
        Self {
            aon_wuc: pac::AON_WUC,
            aux_aiodio0: pac::AUX_AIODIO0,
            aux_aiodio1: pac::AUX_AIODIO1,
            aux_evctl: pac::AUX_EVCTL,
            aux_sce: pac::AUX_SCE,
            aux_timer: pac::AUX_TIMER,
            aux_wuc: pac::AUX_WUC,
            scif_data: Cell::new(Option::None),
            bv_dirty_tasks: Cell::new(0x0000),
            last_aux_ram_image: Cell::new(Option::None),
        }
    }

    pub(crate) unsafe fn scif_init_io(&self, aux_io_index: u32, io_mode: u32, pull_level: i32, output_value: u32) {
        // Calculate access parameters from the AUX I/O index
        let (aux_aiodio_base, aux_aiodio_pin) = if aux_io_index >= 8 {
            (driverlib::AUX_AIODIO1_BASE, aux_io_index - 8)
        } else {
            (driverlib::AUX_AIODIO0_BASE, aux_io_index)
        };

        unsafe {
            // Setup the AUX I/O controller
            Self::modify_reg(aux_aiodio_base + Self::AUX_AIODIO_O_IOMODE, |read| {
                read & !(0x03 << (2 * aux_aiodio_pin)) | (io_mode << (2 * aux_aiodio_pin))
            });
            Self::modify_reg(aux_aiodio_base + Self::AUX_AIODIO_O_GPIODOUT, |read| {
                read & !(0x01 << (aux_aiodio_pin)) | (output_value << aux_aiodio_pin)
            });
            Self::modify_reg(aux_aiodio_base + Self::AUX_AIODIO_O_GPIODIE, |read| {
                read & !(0x01 << (aux_aiodio_pin)) | ((io_mode >> 16) << aux_aiodio_pin)
            });
            // Ensure that the settings have taken effect
            Self::read_reg(aux_aiodio_base + Self::AUX_AIODIO_O_GPIODIE);

            self.scif_reinit_io(aux_io_index, pull_level);
        };
    }

    pub(crate) unsafe fn scif_reinit_io(&self, aux_io_index: u32, pull_level: i32) {
        // Calculate access parameters from the AUX I/O index
        let mcu_iocfg_offset: u32 = self.scif_data().aux_io_index_to_mcu_iocfg_offset_lut[aux_io_index as usize] as u32;

        // Setup the MCU I/O controller, making the AUX I/O setup effective
        let iocfg: u32 = driverlib::IOC_IOCFG0_PORT_ID_AUX_IO
            | match pull_level {
                -1 => driverlib::IOC_IOCFG0_PULL_CTL_DIS,
                0 => driverlib::IOC_IOCFG0_PULL_CTL_DWN,
                1 => driverlib::IOC_IOCFG0_PULL_CTL_UP,
                _ => unreachable!(), // TODO: use enum instead of int
            };
        unsafe {
            ((driverlib::IOC_BASE + driverlib::IOC_O_IOCFG0 + mcu_iocfg_offset) as *mut u32).write_volatile(iocfg);
        };
    }

    pub(crate) unsafe fn scif_uninit_io(&self, aux_io_index: u32, pull_level: i32) {
        // Calculate access parameters from the AUX I/O index
        let mcu_iocfg_offset: u32 = self.scif_data().aux_io_index_to_mcu_iocfg_offset_lut[aux_io_index as usize] as u32;

        // Unconfigure the MCU I/O controller (revert to GPIO with input/output disabled and desired pull
        // level)
        let iocfg: u32 = driverlib::IOC_IOCFG0_PORT_ID_AUX_IO
            | match pull_level {
                -1 => driverlib::IOC_IOCFG0_PULL_CTL_DIS,
                0 => driverlib::IOC_IOCFG0_PULL_CTL_DWN,
                1 => driverlib::IOC_IOCFG0_PULL_CTL_UP,
                _ => unreachable!(), // TODO: use enum instead of int
            };
        unsafe {
            ((driverlib::IOC_BASE + driverlib::IOC_O_IOCFG0 + mcu_iocfg_offset) as *mut u32).write_volatile(iocfg);
        };
    }

    pub(crate) unsafe fn scif_init(&self, scif_driver_setup: &'static SCIFData) -> SCIFResult {
        // Perform sanity checks: The Sensor Controller cannot already be active
        if self.aon_wuc.AUXCTL().read().SCE_RUN_EN() {
            return SCIFResult::IllegalOperation;
        }

        // Copy the driver setup
        self.scif_data.set(Some(scif_driver_setup));

        // Enable clock for required AUX modules
        unsafe {
            driverlib::AUXWUCClockEnable(
                driverlib::AUX_WUC_SMPH_CLOCK
                    | driverlib::AUX_WUC_AIODIO0_CLOCK
                    | driverlib::AUX_WUC_AIODIO1_CLOCK
                    | driverlib::AUX_WUC_TIMER_CLOCK
                    | driverlib::AUX_WUC_ANAIF_CLOCK
                    | driverlib::AUX_WUC_TDCIF_CLOCK
                    | driverlib::AUX_WUC_ADI_CLOCK
                    | driverlib::AUX_WUC_OSCCTRL_CLOCK,
            );

            // Open the AUX I/O latches, which have undefined value after power-up. AUX_AIODIO will by default
            // drive '0' on all I/O pins, so AUX_AIODIO must be configured before IOC
            // FIXME: static inline fn
            driverlib::AUXWUCFreezeDisable();
        }

        let scif_data = self.scif_data();

        // Upload the AUX RAM image
        if self.last_aux_ram_image.get() != Some(scif_data.aux_ram_image) {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    scif_data.aux_ram_image.as_ptr() as *const u8,
                    driverlib::AUX_RAM_BASE as *mut u8,
                    scif_data.aux_ram_image.len() * core::mem::size_of::<u16>(),
                );
            };
            self.last_aux_ram_image.set(Some(scif_data.aux_ram_image));
        }

        // Perform task resource initialization
        unsafe {
            (self.scif_data().fptr_task_resource_init)(self);
        };

        // Map events to the Sensor Controller's vector table, and set reset vector = AON wakeup
        self.aux_evctl.VECCFG0().write(|w| w.set_VEC0_EV(vals::VEC0_EV::AON_SW));
        self.aux_evctl.VECCFG0().write(|w| w.set_VEC0_EN(vals::VEC0_EN::EN));
        self.aux_evctl
            .VECCFG0()
            .write(|w| w.set_VEC1_EV(vals::VEC1_EV::AON_RTC_CH2));
        self.aux_evctl.VECCFG0().write(|w| w.set_VEC1_EN(vals::VEC1_EN::EN));

        self.aux_evctl.VECCFG1().write(|w| w.set_VEC2_EV(vals::VEC2_EV::AON_SW));
        self.aux_evctl.VECCFG1().write(|w| w.set_VEC3_EV(vals::VEC3_EV::AON_SW));
        self.aux_evctl.VECCFG1().write(|w| w.set_VEC3_EN(vals::VEC3_EN::EN));

        self.aux_sce.CTL().write(|w| w.set_RESET_VECTOR(0x1));
        // Clear any vector flags currently set (due to previous hardware or SCIF driver operation)
        unsafe {
            self.aux_evctl.VECFLAGS().as_ptr().write_volatile(VECFLAGS::default());
        };

        // Set the READY event
        self.aux_evctl.SWEVSET().write(|w| w.set_SWEV0(true));

        unsafe {
            // Let AUX be powered down (clocks disabled, full retention) and the bus connection between the AUX
            // and MCU domains be disconnected by default. This may have been done already by the operating
            // system to be able to a framework dependencies on whether or not the Sensor Controller is used.
            driverlib::AUXWUCPowerCtrl(driverlib::AUX_WUC_POWER_DOWN);

            // Start the Sensor Controller, but first read a random register from the AUX domain to ensure
            // that the last write accesses have been completed
            self.aux_wuc.MCUBUSCTL().read();
            driverlib::AONWUCAuxImageValid();
            driverlib::SysCtrlAonSync();
        }

        // Register and enable the interrupts. If warm, we probably have task ALERT event(s) pending, which
        // will be triggered immediately. We need to clear the interrupts because they might have been used
        // previously
        unsafe {
            //osalRegisterCtrlReadyInt();
            Self::osal_clear_ctrl_ready_int();
            Self::osal_enable_ctrl_ready_int();
            //osalRegisterTaskAlertInt();
            Self::osal_clear_task_alert_int();
            Self::osal_enable_task_alert_int();
        }

        SCIFResult::Success
    }

    // General Purpose Input Output Data Out
    const AUX_AIODIO_O_GPIODOUT: u32 = 0x00000000;

    // Input Output Mode
    const AUX_AIODIO_O_IOMODE: u32 = 0x00000004;

    // General Purpose Input Output Digital Input Enable
    const AUX_AIODIO_O_GPIODIE: u32 = 0x00000018;

    unsafe fn read_reg(addr: u32) -> u32 {
        let ptr = addr as *mut u32;
        let read = unsafe { ptr.read_volatile() };
        read
    }

    unsafe fn modify_reg(addr: u32, modifier: impl FnOnce(u32) -> u32) {
        let ptr = addr as *mut u32;
        let read = unsafe { ptr.read_volatile() };
        let modified = modifier(read);
        unsafe {
            ptr.write_volatile(modified);
        };
    }

    fn scif_clear_alert_int_source() {
        let aux_evctl = pac::AUX_EVCTL;
        // Clear the source
        aux_evctl.EVTOAONFLAGSCLR().write(|w| w.set_SWEV1(true));

        // Ensure that the source clearing has taken effect
        while aux_evctl.EVTOAONFLAGS().read().SWEV1() {}
    }

    unsafe fn scif_get_alert_events(&self) -> u32 {
        unsafe { safe_packed_ref!(self.scif_data().task_ctrl.bv_task_io_alert).get() as u32 }
    }

    unsafe fn scif_ack_alert_events(&self) {
        unsafe {
            // Clear the events that have been handled now. This is needed for subsequent ALERT interrupts
            // generated by fwGenQuickAlertInterrupt(), since that procedure does not update bvTaskIoAlert.
            safe_packed_ref!(self.scif_data().task_ctrl.bv_task_io_alert).set(0x0000);

            // Make sure that the CPU interrupt has been cleared before reenabling it
            Self::osal_clear_task_alert_int();
            let key = Self::scif_osal_enter_critical_section();
            Self::osal_enable_task_alert_int();

            // Set the ACK event to the Sensor Controller
            self.aux_evctl.VECCFG1().write(|w| w.set_VEC3_EV(vals::VEC3_EV::AON_SW));
            self.aux_evctl.VECCFG1().write(|w| w.set_VEC3_EN(vals::VEC3_EN::EN));
            self.aux_evctl.VECCFG1().write(|w| w.set_VEC3_POL(vals::VEC3_POL::RISE));

            self.aux_evctl.VECCFG1().write(|w| w.set_VEC3_EV(vals::VEC3_EV::AON_SW));
            self.aux_evctl.VECCFG1().write(|w| w.set_VEC3_EN(vals::VEC3_EN::EN));

            Self::scif_osal_leave_critical_section(key);
        }
    }

    unsafe fn scif_set_task_startup_delay(&self, task_id: u32, ticks: u16) {
        unsafe {
            self.scif_data()
                .task_execute_schedule
                .add(task_id as usize)
                .write_volatile(ticks);
        }
    }

    pub(crate) unsafe fn scif_reset_task_structs(&self, mut bv_task_ids: u32, mut bv_task_structs: u32) {
        // Indicate that the data structure has been cleared
        self.bv_dirty_tasks
            .set(self.bv_dirty_tasks.get() & (!bv_task_ids as u16));

        // Always clean the state data structure
        bv_task_structs |= 1 << SCIFTaskStructType::SCIFStructState as u32;

        // As long as there are more tasks to reset ...
        while bv_task_ids != 0 {
            let task_id: u32 = bv_task_ids.trailing_zeros();

            bv_task_ids &= !(1 << task_id);

            // For each data structure to be reset ...
            while bv_task_structs != 0 {
                let n: u32 = bv_task_structs.trailing_zeros();
                bv_task_structs &= !(1 << n);

                let task_struct_info: u32 = self.scif_data().task_data_struct_info_lut[(task_id * 4 + n) as usize];

                // If it exists ...
                if task_struct_info != 0 {
                    // Split the information
                    let mut addr: u16 = (task_struct_info >> 0) as u16 & 0x0FFF; // 11:0
                    let count: u16 = (task_struct_info >> 12) as u16 & 0x00FF; // 19:12
                    let size: u16 = (task_struct_info >> 20) as u16 & 0x0FFF; // 31:20
                    let mut length: u16 = core::mem::size_of::<u16>() as u16 * size * count;

                    // If multiple-buffered, include the control variables
                    if count > 1 {
                        addr -= SCIF_TASK_STRUCT_CTRL_SIZE as u16;
                        length += SCIF_TASK_STRUCT_CTRL_SIZE as u16;
                    }

                    // Reset the data structure
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            (self.scif_data().aux_ram_image.as_ptr() as *const u8).add(addr as usize),
                            (driverlib::AUX_RAM_BASE as *mut u8).add(addr as usize),
                            length as usize,
                        );
                    };
                }
            }
        }
    }

    unsafe fn scif_get_task_io_struct_avail_count(&self, task_id: u32, task_struct_type: SCIFTaskStructType) -> u32 {
        // Fetch the information about the data structure
        let task_struct_info: u32 =
            self.scif_data().task_data_struct_info_lut[(task_id * 4 + task_struct_type as u32) as usize];
        let base_addr: u16 = (task_struct_info >> 0) as u16 & 0x0FFF; // 11:0
        let count: u16 = (task_struct_info >> 12) as u16 & 0x00FF; // 19:12
        let size: u16 = (task_struct_info >> 20) as u16 & 0x0FFF; // 31:20

        // If single-buffered, it's always 0
        if count < 2 {
            return 0;
        }

        // Fetch the current memory addresses used by SCE and MCU
        let mut sce_addr: u16 = unsafe {
            ((driverlib::AUX_RAM_BASE + base_addr as u32 - SCIF_TASK_STRUCT_CTRL_SCE_ADDR_BACK_OFFSET) as *const u16)
                .read_volatile()
        };
        let mut mcu_addr: u16 = unsafe {
            ((driverlib::AUX_RAM_BASE + base_addr as u32 - SCIF_TASK_STRUCT_CTRL_MCU_ADDR_BACK_OFFSET) as *const u16)
                .read_volatile()
        };

        // Buffer overflow or underflow can occur in the background if the Sensor Controller produces or
        // consumes data too fast for the System CPU application. If this happens, return 0 so that the
        // application can detect the error by calling scifGetAlertEvents() in the next ALERT interrupt
        // before starting to process potentially corrupted or out-of-sync buffers.
        unsafe {
            if safe_packed_ref!(self.scif_data().int_data.bv_task_io_alert).get() & (0x0100 << task_id) != 0 {
                return 0;
            }
        }

        // Detect all buffers available
        // LSBs are different when none are available -> handled in the calculation further down
        if mcu_addr == sce_addr {
            return count as u32;
        }

        // Calculate the number of buffers available
        mcu_addr &= !0x0001;
        sce_addr &= !0x0001;
        if sce_addr < mcu_addr {
            sce_addr += size * core::mem::size_of::<u16>() as u16 * count;
        }

        ((sce_addr - mcu_addr) / (size * core::mem::size_of::<u16>() as u16)) as u32
    }

    unsafe fn scif_get_task_struct(&self, task_id: u32, task_struct_type: SCIFTaskStructType) -> *mut () {
        // Fetch the information about the data structure
        let task_struct_info: u32 =
            self.scif_data().task_data_struct_info_lut[(task_id * 4 + task_struct_type as u32) as usize];
        let base_addr: u16 = (task_struct_info >> 0) as u16 & 0x0FFF; // 11:0
        let count: u16 = (task_struct_info >> 12) as u16 & 0x00FF; // 19:12

        // If single-buffered, just return the base address
        if count < 2 {
            unsafe { (driverlib::AUX_RAM_BASE as *mut ()).add(base_addr as usize) }

        // If multiple-buffered, return the MCU address
        } else {
            unsafe {
                let mcu_addr: u16 = (driverlib::AUX_RAM_BASE as *const u16)
                    .add(base_addr as usize)
                    .sub(SCIF_TASK_STRUCT_CTRL_MCU_ADDR_BACK_OFFSET as usize)
                    .read_volatile();
                (driverlib::AUX_RAM_BASE as *mut ()).add(mcu_addr as usize & !0x0001)
            }
        }
    }

    unsafe fn scif_handoff_task_struct(&self, task_id: u32, task_struct_type: SCIFTaskStructType) {
        // Fetch the information about the data structure
        let task_struct_info: u32 =
            self.scif_data().task_data_struct_info_lut[(task_id * 4 + task_struct_type as u32) as usize];
        let base_addr: u16 = (task_struct_info >> 0) as u16 & 0x0FFF; // 11:0
        let count: u16 = (task_struct_info >> 12) as u16 & 0x00FF; // 19:12
        let size: u16 = (task_struct_info >> 20) as u16 & 0x0FFF; // 31:20

        // If multiple-buffered, move on the MCU address to the next buffer
        if count >= 2 {
            // Move on the address
            let p_mcu_addr: *mut u16 =
                (driverlib::AUX_RAM_BASE + base_addr as u32 - SCIF_TASK_STRUCT_CTRL_MCU_ADDR_BACK_OFFSET) as *mut u16;
            let mut new_mcu_addr: u16 = unsafe { *p_mcu_addr } + size * core::mem::size_of::<u16>() as u16;

            // If it has wrapped, move it back to the start and invert LSB
            if new_mcu_addr & !0x0001 > (base_addr + (size * core::mem::size_of::<u16>() as u16 * (count - 1))) {
                new_mcu_addr = base_addr | ((new_mcu_addr & 0x0001) ^ 0x0001);
            }

            // Write back the new address
            unsafe {
                p_mcu_addr.write_volatile(new_mcu_addr);
            };
        }
    }

    unsafe fn scif_ctrl_tasks_nbl(&self, bv_task_ids: u32, bv_task_req: u32) -> SCIFResult {
        // Prevent interruptions by concurrent scifCtrlTasksNbl() calls
        if !Self::osal_lock_ctrl_task_nbl() {
            return SCIFResult::NotReady;
        }

        // Perform sanity checks: Starting already active or dirty tasks is illegal
        if bv_task_req & 0x01 != 0 {
            let task_ctrl = self.scif_data().task_ctrl;
            unsafe {
                if (safe_packed_ref!(task_ctrl.bv_active_tasks).get() | self.bv_dirty_tasks.get())
                    & (bv_task_ids as u16)
                    != 0
                {
                    Self::osal_unlock_ctrl_task_nbl();
                    return SCIFResult::IllegalOperation;
                }
            }
        }

        // Verify that the control interface is ready
        if !SCIF_READY.swap(false, Ordering::Relaxed) {
            Self::osal_unlock_ctrl_task_nbl();
            return SCIFResult::NotReady;
        }

        let task_ctl_data = self.scif_data().task_ctrl;
        unsafe {
            // Initialize tasks?
            safe_packed_ref!(task_ctl_data.bv_task_initialize_req).set(if bv_task_req & 0x01 != 0 {
                self.bv_dirty_tasks
                    .set(self.bv_dirty_tasks.get() | (bv_task_ids as u16));
                bv_task_ids as u16
            } else {
                0x0000
            });

            // Execute tasks?
            safe_packed_ref!(task_ctl_data.bv_task_execute_req).set(if bv_task_req & 0x02 != 0 {
                bv_task_ids as u16
            } else {
                0x0000
            });

            // Terminate tasks? Terminating already inactive tasks is allowed, because tasks may stop
            // spontaneously, and there's no way to know this for sure (it may for instance happen at any moment
            // while calling this function)
            safe_packed_ref!(task_ctl_data.bv_task_terminate_req).set(if (bv_task_req & 0x04) != 0 {
                bv_task_ids as u16
            } else {
                0x0000
            });

            // Make sure that the CPU interrupt has been cleared before reenabling it
            Self::osal_clear_ctrl_ready_int();
            Self::osal_enable_ctrl_ready_int();
        }

        // Set the REQ event to hand over the request to the Sensor Controller

        self.aux_evctl
            .VECCFG0()
            .modify(|w| w.set_VEC0_POL(vals::VEC0_POL::RISE));
        self.aux_evctl
            .VECCFG0()
            .modify(|w| w.set_VEC0_POL(vals::VEC0_POL::FALL));
        Self::osal_unlock_ctrl_task_nbl();

        SCIFResult::Success
    }

    pub(crate) unsafe fn scif_execute_tasks_once_nbl(&self, bv_task_ids: u16) -> SCIFResult {
        unsafe { self.scif_ctrl_tasks_nbl(bv_task_ids as u32, 0x07) }
    }

    unsafe fn scif_start_tasks_nbl(&self, bv_task_ids: u16) -> SCIFResult {
        unsafe { self.scif_ctrl_tasks_nbl(bv_task_ids as u32, 0x01) }
    }

    unsafe fn scif_stop_tasks_nbl(&self, bv_task_ids: u16) -> SCIFResult {
        unsafe { self.scif_ctrl_tasks_nbl(bv_task_ids as u32, 0x04) }
    }

    unsafe fn scif_wait_on_nbl(&self, timeout_us: u32) -> SCIFResult {
        unsafe {
            if self.aux_evctl.EVTOAONFLAGS().read().SWEV0() || self.osal_wait_on_ctrl_ready(timeout_us) {
                SCIFResult::Success
            } else {
                SCIFResult::NotReady
            }
        }
    }

    unsafe fn scif_get_active_task_ids(&self) -> u16 {
        unsafe { safe_packed_ref!(self.scif_data().task_ctrl.bv_active_tasks).get() }
    }

    unsafe fn scif_osal_enter_critical_section() -> bool {
        unsafe { driverlib::CPUcpsid() == 0 }
    }

    unsafe fn scif_osal_leave_critical_section(key: bool) {
        if key {
            unsafe {
                driverlib::CPUcpsie();
            };
        }
    }

    fn osal_lock_ctrl_task_nbl() -> bool {
        /*uint32_t key = !CPUcpsid();
        if (osalCtrlTaskNblLocked) {
            if (key) CPUcpsie();
            return false;
        } else {
            osalCtrlTaskNblLocked = true;
            if (key) CPUcpsie();
            return true;
        }*/
        return true;
    }

    fn osal_unlock_ctrl_task_nbl() {
        //osalCtrlTaskNblLocked = false;
    }

    pub(crate) unsafe extern "C" fn ready_handler() {
        let aux_evctl = pac::AUX_EVCTL;
        // HWREG(driverlib::NVIC_DIS0 + NVIC_OFFSET(INT_SCIF_CTRL_READY)) = NVIC_BV(INT_SCIF_CTRL_READY);
        cortex_m::peripheral::NVIC::mask(INT_SCIF_CTRL_READY);
    }

    pub(crate) unsafe extern "C" fn alert_handler() {
        Self::scif_clear_alert_int_source();
        // HWREG(driverlib::NVIC_DIS0 + NVIC_OFFSET(INT_SCIF_TASK_ALERT)) = NVIC_BV(INT_SCIF_TASK_ALERT);
        cortex_m::peripheral::NVIC::mask(INT_SCIF_TASK_ALERT);
    }

    unsafe fn osal_enable_ctrl_ready_int() {
        // FIXME: interrupt.c brings GBs to the bin file...
        // driverlib::IntRegister(driverlib::INT_AUX_SWEV0, Some(Self::ready_handler));
        unsafe {
            cortex_m::peripheral::NVIC::unmask(INT_SCIF_CTRL_READY);
        };
    }

    unsafe fn osal_clear_ctrl_ready_int() {
        cortex_m::peripheral::NVIC::unpend(INT_SCIF_CTRL_READY);
    }

    unsafe fn osal_clear_task_alert_int() {
        cortex_m::peripheral::NVIC::unpend(INT_SCIF_TASK_ALERT);
    }

    unsafe fn osal_enable_task_alert_int() {
        unsafe {
            cortex_m::peripheral::NVIC::unmask(INT_SCIF_TASK_ALERT);
        };
        // FIXME: interrupt.c brings GBs to the bin file...
        // driverlib::IntRegister(driverlib::INT_AUX_SWEV1, Some(Self::alert_handler));
    }

    unsafe fn osal_wait_on_ctrl_ready(&self, timeout_us: u32) -> bool {
        if timeout_us > 0 {
            while self.aux_evctl.EVTOAONFLAGS().read().SWEV0() {}
            true
        } else {
            self.aux_evctl.EVTOAONFLAGS().read().SWEV0()
        }
    }
}
