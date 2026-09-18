use crate::chip::interrupt;
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
        unsafe { core::ptr::addr_of!($place).as_ref().unwrap_unchecked() }
    };
}
pub(crate) use safe_packed_ref;

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

    /// AUX RAM image word array
    pub(crate) aux_ram_image: &'static [u16],

    /// Look-up table that converts from AUX I/O index to MCU IOCFG offset
    pub(crate) task_data_struct_info_lut: &'static [u32],
    /// Look-up table of data structure information for each task
    pub(crate) aux_io_index_to_mcu_iocfg_offset_lut: &'static [u8],

    /// Pointer to the project-specific hardware initialization function
    pub(crate) fptr_task_resource_init: SCIFVfptr,
}

unsafe impl Sync for SCIFData {}

/// I/O pin mode: Output
pub(crate) const AUXIOMODE_OUTPUT: u32 = 0x00000000;
/// I/O pin mode: Input, active
pub(crate) const AUXIOMODE_INPUT: u32 = 0x00010001;
/// Task data structure buffer control: Size (in bytes)
const SCIF_TASK_STRUCT_CTRL_SIZE: u32 = 3 * core::mem::size_of::<u16>() as u32;

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
    scif_data: Cell<Option<SCIFData>>,
    /// Bit-vector indicating tasks with potentially modified input/output/state data structures
    bv_dirty_tasks: Cell<u16>,
    last_aux_ram_image: Cell<Option<&'static [u16]>>,
}

impl Scif {
    fn scif_data(&self) -> SCIFData {
        self.scif_data.get().unwrap()
    }

    pub(crate) const fn new() -> Self {
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

    pub(crate) unsafe fn scif_init(&self, scif_driver_setup: SCIFData, dirty_tasks: u16) -> SCIFResult {
        // Perform sanity checks: The Sensor Controller cannot already be active
        if self.aon_wuc.AUXCTL().read().SCE_RUN_EN() {
            return SCIFResult::IllegalOperation;
        }

        // Copy the driver setup
        self.scif_data.set(Some(scif_driver_setup));
        self.bv_dirty_tasks.set(dirty_tasks);

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
        self.aux_evctl.VECCFG0().write(|w| {
            w.set_VEC0_EV(vals::VEC0_EV::AON_SW);
            w.set_VEC0_EN(vals::VEC0_EN::EN);
            w.set_VEC1_EV(vals::VEC1_EV::AON_RTC_CH2);
            w.set_VEC1_EN(vals::VEC1_EN::EN);
        });

        self.aux_evctl.VECCFG1().write(|w| {
            w.set_VEC2_EV(vals::VEC2_EV::AON_SW);
            w.set_VEC3_EV(vals::VEC3_EV::AON_SW);
            w.set_VEC3_EN(vals::VEC3_EN::EN);
        });

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

    fn scif_clear_ready_int_source() {
        let aux_evctl = pac::AUX_EVCTL;
        // Clear the source
        aux_evctl.EVTOAONFLAGSCLR().write(|w| w.set_SWEV0(true));

        // Ensure that the source clearing has taken effect
        while aux_evctl.EVTOAONFLAGS().read().SWEV0() {}

        SCIF_READY.store(true, Ordering::Relaxed);
    }

    fn scif_clear_alert_int_source() {
        let aux_evctl = pac::AUX_EVCTL;
        // Clear the source
        aux_evctl.EVTOAONFLAGSCLR().write(|w| w.set_SWEV1(true));

        // Ensure that the source clearing has taken effect
        while aux_evctl.EVTOAONFLAGS().read().SWEV1() {}
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

    unsafe fn scif_ctrl_tasks_nbl(&self, bv_task_ids: u32, bv_task_req: u32) -> SCIFResult {
        // Prevent interruptions by concurrent scifCtrlTasksNbl() calls
        if !Self::osal_lock_ctrl_task_nbl() {
            return SCIFResult::NotReady;
        }

        // Perform sanity checks: Starting already active or dirty tasks is illegal
        if bv_task_req & 0x01 != 0 {
            let task_ctrl = self.scif_data().task_ctrl;
            if (safe_packed_ref!(task_ctrl.bv_active_tasks).get() | self.bv_dirty_tasks.get()) & (bv_task_ids as u16)
                != 0
            {
                Self::osal_unlock_ctrl_task_nbl();
                return SCIFResult::IllegalOperation;
            }
        }

        // Verify that the control interface is ready
        if !SCIF_READY.swap(false, Ordering::Relaxed) {
            Self::osal_unlock_ctrl_task_nbl();
            return SCIFResult::NotReady;
        }

        let task_ctl_data = self.scif_data().task_ctrl;
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
        unsafe {
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
        Self::scif_clear_ready_int_source();
        cortex_m::peripheral::NVIC::mask(INT_SCIF_CTRL_READY);
    }

    pub(crate) unsafe extern "C" fn alert_handler() {
        Self::scif_clear_alert_int_source();
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
}

#[interrupt]
fn UART1() {
    unsafe {
        Scif::ready_handler();
    };
}

#[interrupt]
fn AON_EVENT() {
    unsafe {
        Scif::alert_handler();
    };
}
