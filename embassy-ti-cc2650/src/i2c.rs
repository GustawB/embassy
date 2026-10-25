#![macro_use]

use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Poll;

use crate::interrupt::typelevel::Interrupt;
use crate::{chip::interrupt, driverlib};
use embassy_hal_internal::drop::OnDrop;
use embassy_hal_internal::{Peri, PeripheralType};

/// Interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        unsafe {
            let master_irq_status = driverlib::I2CMasterIntStatus(driverlib::I2C0_BASE, true);
            driverlib::I2CMasterIntClear(driverlib::I2C0_BASE);
            let slave_irq_status = driverlib::I2CSlaveIntStatus(driverlib::I2C0_BASE, true);
            driverlib::I2CSlaveIntClear(driverlib::I2C0_BASE, slave_irq_status);
            let s = T::state();
            if master_irq_status {
                s.master_waker.wake();
            }
            if slave_irq_status != 0 {
                if slave_irq_status & driverlib::I2C_SLAVE_INT_START != 0 {
                    s.start_condition.store(true, Ordering::Release);
                }
                if slave_irq_status & driverlib::I2C_SLAVE_INT_STOP != 0 {
                    s.end_condition.store(true, Ordering::Release);
                }
                s.slave_waker.wake();
            }
        }
    }
}

/// I2C peripheral instance
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    /// Interrupt for this peripheral.
    type Interrupt: interrupt::typelevel::Interrupt;
}

pub(crate) struct State {
    pub(crate) master_waker: AtomicWaker,
    pub(crate) slave_waker: AtomicWaker,
    pub(crate) start_condition: AtomicBool,
    pub(crate) end_condition: AtomicBool,
}

impl State {
    pub(crate) const fn new() -> Self {
        Self {
            master_waker: AtomicWaker::new(),
            slave_waker: AtomicWaker::new(),
            start_condition: AtomicBool::new(false),
            end_condition: AtomicBool::new(false),
        }
    }
}

pub(crate) trait SealedInstance {
    fn state() -> &'static State;
}

macro_rules! impl_i2c {
    ($type:ident, $pac_type:ident, $irq:ident) => {
        impl crate::i2c::SealedInstance for peripherals::$type {
            fn state() -> &'static crate::i2c::State {
                static STATE: crate::i2c::State = crate::i2c::State::new();
                &STATE
            }
        }
        impl crate::i2c::Instance for peripherals::$type {
            type Interrupt = crate::interrupt::typelevel::$irq;
        }
    };
}

#[derive(PartialEq)]
pub enum MasterResult {
    /// Transaction succeeded
    Ok,
    /// Address was not acknowledged by the receiver.
    AddrAck,
    /// Data was not acknowledged by the receiver.
    DataAck,
    /// Master lost arbitration.
    ArLost,
}

pub struct I2CMaster<'a, T: Instance> {
    state: &'static State,
    _p: PhantomData<(&'a (), T)>,
}

impl<'a, T: Instance> I2CMaster<'a, T> {
    pub(crate) fn new(
        //_i2c_master: Peri<'a, T>,
        _irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
    ) -> Self {
        unsafe {
            // Setup pins.
            driverlib::IOCPinTypeI2c(driverlib::IOC_BASE, driverlib::IOID_18, driverlib::IOID_19);

            // Initialize I2C master.
            let freq = driverlib::SysCtrlClockGet();
            driverlib::I2CMasterInitExpClk(driverlib::I2C0_BASE, freq, true);
            driverlib::I2CMasterIntEnable(driverlib::I2C0_BASE);
        };

        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };

        Self {
            state: T::state(),
            _p: PhantomData,
        }
    }

    async fn await_master_completion(&self) {
        let _ = poll_fn(|cx| {
            self.state.master_waker.register(cx.waker());
            unsafe {
                if driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {
                    return Poll::Pending;
                }
            }
            Poll::Ready(())
        })
        .await;
    }

    fn handle_transaction_result(&self) -> MasterResult {
        let error_flags = unsafe { driverlib::I2CMasterErr(driverlib::I2C0_BASE) };
        if error_flags == driverlib::I2C_MASTER_ERR_NONE {
            MasterResult::Ok
        } else if error_flags & driverlib::I2C_MASTER_ERR_ADDR_ACK != 0 {
            MasterResult::AddrAck
        } else if error_flags & driverlib::I2C_MASTER_ERR_ARB_LOST != 0 {
            MasterResult::ArLost
        } else {
            MasterResult::DataAck
        }
    }

    fn get_drop_guard(flag: u32) -> OnDrop<impl FnOnce() -> ()> {
        OnDrop::new(move || {
            unsafe {
                // MCTRL must not be written while a command is in flight.
                while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}
                // After lost arbitration the bus isn't ours, so no STOP.
                if driverlib::I2CMasterErr(driverlib::I2C0_BASE) & driverlib::I2C_MASTER_ERR_ARB_LOST == 0 {
                    driverlib::I2CMasterControl(driverlib::I2C0_BASE, flag);
                }
            };
        })
    }

    fn first_send_cmd(len: usize) -> u32 {
        if len == 1 {
            driverlib::I2C_MASTER_CMD_SINGLE_SEND
        } else {
            driverlib::I2C_MASTER_CMD_BURST_SEND_START
        }
    }

    fn next_send_cmd(i: usize, len: usize) -> u32 {
        if i == len - 1 {
            driverlib::I2C_MASTER_CMD_BURST_SEND_FINISH
        } else {
            driverlib::I2C_MASTER_CMD_BURST_SEND_CONT
        }
    }

    fn first_recv_cmd(len: usize) -> u32 {
        if len == 1 {
            driverlib::I2C_MASTER_CMD_SINGLE_RECEIVE
        } else {
            driverlib::I2C_MASTER_CMD_BURST_RECEIVE_START
        }
    }

    fn next_recv_cmd(i: usize, len: usize) -> u32 {
        if i == len - 1 {
            driverlib::I2C_MASTER_CMD_BURST_RECEIVE_FINISH
        } else {
            driverlib::I2C_MASTER_CMD_BURST_RECEIVE_CONT
        }
    }

    pub fn read_byte_blocking(&mut self, slave_addr: u8, buf: &mut u8) -> MasterResult {
        unsafe {
            // Set the receiving slave address.
            driverlib::I2CMasterSlaveAddrSet(driverlib::I2C0_BASE, slave_addr, true);
            // Wait until master controller is not busy.
            while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}
            // Write 0-111 to MCTRL.
            driverlib::I2CMasterControl(driverlib::I2C0_BASE, driverlib::I2C_MASTER_CMD_SINGLE_RECEIVE);
            // Wait until master controller is not busy.
            while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}

            match self.handle_transaction_result() {
                MasterResult::Ok => *buf = driverlib::I2CMasterDataGet(driverlib::I2C0_BASE) as u8,
                err => return err,
            }
        }
        MasterResult::Ok
    }

    pub fn write_byte_blocking(&mut self, slave_addr: u8, buf: u8) -> MasterResult {
        unsafe {
            // Set the receiving slave address.
            driverlib::I2CMasterSlaveAddrSet(driverlib::I2C0_BASE, slave_addr, false);
            // Transmit one byte.
            driverlib::I2CMasterDataPut(driverlib::I2C0_BASE, buf);
            // Wait until master controller is not busy.
            while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}
            // Write 0-111 to MCTRL.
            driverlib::I2CMasterControl(driverlib::I2C0_BASE, driverlib::I2C_MASTER_CMD_SINGLE_SEND);
            // Wait until master controller is not busy.
            while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}
            self.handle_transaction_result()
        }
    }

    pub async fn read_byte(&mut self, slave_addr: u8, buf: &mut u8) -> MasterResult {
        unsafe {
            // Set the receiving slave address.
            driverlib::I2CMasterSlaveAddrSet(driverlib::I2C0_BASE, slave_addr, true);
            // Wait until master controller is not busy.
            self.await_master_completion().await;
            // Write 0-111 to MCTRL.
            driverlib::I2CMasterControl(driverlib::I2C0_BASE, driverlib::I2C_MASTER_CMD_SINGLE_RECEIVE);
            // Wait until master controller is not busy.
            self.await_master_completion().await;

            match self.handle_transaction_result() {
                MasterResult::Ok => *buf = driverlib::I2CMasterDataGet(driverlib::I2C0_BASE) as u8,
                err => return err,
            }
        }
        MasterResult::Ok
    }

    pub async fn write_byte(&mut self, slave_addr: u8, buf: u8) -> MasterResult {
        unsafe {
            // Set the receiving slave address.
            driverlib::I2CMasterSlaveAddrSet(driverlib::I2C0_BASE, slave_addr, false);
            // Transmit one byte.
            driverlib::I2CMasterDataPut(driverlib::I2C0_BASE, buf);
            // Wait until master controller is not busy.
            self.await_master_completion().await;
            // Write 0-111 to MCTRL.
            driverlib::I2CMasterControl(driverlib::I2C0_BASE, driverlib::I2C_MASTER_CMD_SINGLE_SEND);
            // Wait until master controller is not busy.
            self.await_master_completion().await;
            self.handle_transaction_result()
        }
    }

    /// Tries to read the data one byte at a time into the provided buf.
    /// buf size determines the amount of bytes to be read.
    /// Returns the number of bytes read, and in case of any errors,
    /// the error itself.
    pub async fn read(&mut self, slave_addr: u8, buf: &mut [u8]) -> (usize, MasterResult) {
        if buf.is_empty() {
            return (0, MasterResult::Ok);
        }

        unsafe {

            // Set the receiving slave address.
            driverlib::I2CMasterSlaveAddrSet(driverlib::I2C0_BASE, slave_addr, true);
            // Wait until master controller is not busy.
            self.await_master_completion().await;
            // Write 0-111 (single byte) or 0-1011 (burst start) to MCTRL.
            driverlib::I2CMasterControl(driverlib::I2C0_BASE, Self::first_recv_cmd(buf.len()));
            let drop = Self::get_drop_guard(driverlib::I2C_MASTER_CMD_BURST_RECEIVE_ERROR_STOP);

            for i in 1usize..buf.len() {
                // Wait until master controller is not busy.
                self.await_master_completion().await;

                // The status refers to buf[i - 1].
                let transaction_result = self.handle_transaction_result();
                if transaction_result != MasterResult::Ok {
                    return (i - 1, transaction_result);
                }
                // Read one byte.
                buf[i - 1] = driverlib::I2CMasterDataGet(driverlib::I2C0_BASE) as u8;

                // Write 0-101 (last byte, NACK) or 0-1001 to MCTRL.
                driverlib::I2CMasterControl(driverlib::I2C0_BASE, Self::next_recv_cmd(i, buf.len()));
            }
            drop.defuse();

            // Wait until master controller is not busy.
            self.await_master_completion().await;

            let transaction_result = self.handle_transaction_result();
            if transaction_result != MasterResult::Ok {
                return (buf.len() - 1, transaction_result);
            }
            // Read the last byte.
            buf[buf.len() - 1] = driverlib::I2CMasterDataGet(driverlib::I2C0_BASE) as u8;

            // Idle
            (buf.len(), MasterResult::Ok)
        }
    }

    pub fn read_blocking(&mut self, slave_addr: u8, buf: &mut [u8]) -> (usize, MasterResult) {
        if buf.is_empty() {
            return (0, MasterResult::Ok);
        }

        unsafe {
            // Set the receiving slave address.
            driverlib::I2CMasterSlaveAddrSet(driverlib::I2C0_BASE, slave_addr, true);
            // Wait until master controller is not busy.
            while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}
            // Write 0-111 (single byte) or 0-1011 (burst start) to MCTRL.
            driverlib::I2CMasterControl(driverlib::I2C0_BASE, Self::first_recv_cmd(buf.len()));
            let drop = Self::get_drop_guard(driverlib::I2C_MASTER_CMD_BURST_RECEIVE_ERROR_STOP);

            for i in 1usize..buf.len() {
                // Wait until master controller is not busy.
                while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}

                // The status refers to buf[i - 1].
                let transaction_result = self.handle_transaction_result();
                if transaction_result != MasterResult::Ok {
                    return (i - 1, transaction_result);
                }

                // Transfer one byte.
                buf[i - 1] = driverlib::I2CMasterDataGet(driverlib::I2C0_BASE) as u8;

                // Write 0-101 (last byte, NACK) or 0-1001 to MCTRL.
                driverlib::I2CMasterControl(driverlib::I2C0_BASE, Self::next_recv_cmd(i, buf.len()));
            }
            drop.defuse();

            // Wait until master controller is not busy.
            while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}

            let transaction_result = self.handle_transaction_result();
            if transaction_result != MasterResult::Ok {
                return (buf.len() - 1, transaction_result);
            }
            // Read the last byte.
            buf[buf.len() - 1] = driverlib::I2CMasterDataGet(driverlib::I2C0_BASE) as u8;

            // Idle
            (buf.len(), MasterResult::Ok)
        }
    }

    /// Tries to write the contents of the buf over I2C, one byte at a time.
    /// Returns the number of bytes sent, and in case of any errors,
    /// the error itself.
    pub async fn write(&mut self, slave_addr: u8, buf: &[u8]) -> (usize, MasterResult) {
        if buf.is_empty() {
            return (0, MasterResult::Ok);
        }

        unsafe {

            // Set the receiving slave address.
            driverlib::I2CMasterSlaveAddrSet(driverlib::I2C0_BASE, slave_addr, false);
            // Wait until master controller is not busy.
            self.await_master_completion().await;
            // START sends the address followed by MDR, so MDR must hold the first byte.
            driverlib::I2CMasterDataPut(driverlib::I2C0_BASE, buf[0]);
            // Write 0-111 (single byte) or 0-011 (burst start) to MCTRL.
            driverlib::I2CMasterControl(driverlib::I2C0_BASE, Self::first_send_cmd(buf.len()));
            let drop = Self::get_drop_guard(driverlib::I2C_MASTER_CMD_BURST_SEND_ERROR_STOP);

            for i in 1usize..buf.len() {
                // Wait until master controller is not busy.
                self.await_master_completion().await;

                // The status refers to buf[i - 1].
                let transaction_result = self.handle_transaction_result();
                if transaction_result != MasterResult::Ok {
                    return (i - 1, transaction_result);
                }

                // Transfer one byte.
                driverlib::I2CMasterDataPut(driverlib::I2C0_BASE, buf[i]);
                // Write 0-101 (last byte) or 0-001 to MCTRL.
                driverlib::I2CMasterControl(driverlib::I2C0_BASE, Self::next_send_cmd(i, buf.len()));
            }
            drop.defuse();

            // Wait until master controller is not busy.
            self.await_master_completion().await;

            let transaction_result = self.handle_transaction_result();
            if transaction_result != MasterResult::Ok {
                return (buf.len() - 1, transaction_result);
            }

            // Idle
            (buf.len(), MasterResult::Ok)
        }
    }

    // Same as write, but blocking.
    pub fn write_blocking(&mut self, slave_addr: u8, buf: &[u8]) -> (usize, MasterResult) {
        if buf.is_empty() {
            return (0, MasterResult::Ok);
        }

        unsafe {
            // Set the receiving slave address.
            driverlib::I2CMasterSlaveAddrSet(driverlib::I2C0_BASE, slave_addr, false);
            // Wait until master controller is not busy.
            while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}
            // START sends the address followed by MDR, so MDR must hold the first byte.
            driverlib::I2CMasterDataPut(driverlib::I2C0_BASE, buf[0]);
            // Write 0-111 (single byte) or 0-011 (burst start) to MCTRL.
            driverlib::I2CMasterControl(driverlib::I2C0_BASE, Self::first_send_cmd(buf.len()));
            let drop = Self::get_drop_guard(driverlib::I2C_MASTER_CMD_BURST_SEND_ERROR_STOP);

            for i in 1usize..buf.len() {
                // Wait until master controller is not busy.
                while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}

                // The status refers to buf[i - 1].
                let transaction_result = self.handle_transaction_result();
                if transaction_result != MasterResult::Ok {
                    return (i - 1, transaction_result);
                }

                // Transfer one byte.
                driverlib::I2CMasterDataPut(driverlib::I2C0_BASE, buf[i]);
                // Write 0-101 (last byte) or 0-001 to MCTRL.
                driverlib::I2CMasterControl(driverlib::I2C0_BASE, Self::next_send_cmd(i, buf.len()));
            }
            drop.defuse();
            // Wait until master controller is not busy.
            while driverlib::I2CMasterBusy(driverlib::I2C0_BASE) {}

            let transaction_result = self.handle_transaction_result();
            if transaction_result != MasterResult::Ok {
                return (buf.len() - 1, transaction_result);
            }

            // Idle
            (buf.len(), MasterResult::Ok)
        }
    }
}

pub struct Runner<'d> {
    state: &'d State,
}

impl<'d> Runner<'d> {
    pub(crate) fn new(state: &'static State) -> Self {
        Self { state: state }
    }

    /// Not cancel-safe: dropping the future mid-transaction loses the bytes in flight.
    /// The bus stays stretched until the next call resumes servicing it.
    pub async fn do_transaction(&mut self, read_callback: impl Fn(u8), write_callback: impl Fn() -> u8) {
        // 1. Wait for the start condition, or resume a transaction a dropped call left pending.
        let _ = poll_fn(|cx| {
            self.state.slave_waker.register(cx.waker());
            let slave_status = unsafe { driverlib::I2CSlaveStatus(driverlib::I2C0_BASE) };
            if slave_status & (driverlib::I2C_SLAVE_ACT_RREQ | driverlib::I2C_SLAVE_ACT_TREQ) != 0 {
                return Poll::Ready(());
            }
            if self
                .state
                .start_condition
                .compare_exchange(true, false, Ordering::Acquire, Ordering::SeqCst)
                .is_err()
            {
                return Poll::Pending;
            }
            Poll::Ready(())
        })
        .await;

        // 2. Try to read data as long as there is no STOP condition.
        let mut stop_condition = false;
        while !stop_condition {
            stop_condition = poll_fn(|cx| {
                self.state.slave_waker.register(cx.waker());
                let slave_status = unsafe { driverlib::I2CSlaveStatus(driverlib::I2C0_BASE) };
                if slave_status & driverlib::I2C_SLAVE_ACT_RREQ != 0 {
                    let data = unsafe { driverlib::I2CSlaveDataGet(driverlib::I2C0_BASE) };
                    read_callback(data as u8);
                    return Poll::Ready(false);
                } else if slave_status & driverlib::I2C_SLAVE_ACT_TREQ != 0 {
                    let data = write_callback();
                    unsafe {
                        driverlib::I2CSlaveDataPut(driverlib::I2C0_BASE, data);
                    };
                    return Poll::Ready(false);
                } else if self
                    .state
                    .end_condition
                    .compare_exchange(true, false, Ordering::Acquire, Ordering::SeqCst)
                    .is_ok()
                {
                    return Poll::Ready(true);
                }
                Poll::Pending
            })
            .await;
        }
    }
}

pub struct I2CSlave<'a, T: Instance> {
    runner: Runner<'a>,
    _p: PhantomData<(&'a (), T)>,
}

impl<'a, T: Instance> I2CSlave<'a, T> {
    pub(crate) fn new(
        //_i2c: Peri<'a, T>,
        slave_addr: u8,
        _irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
    ) -> Self {
        unsafe {
            // Setup pins.
            driverlib::IOCPinTypeI2c(driverlib::IOC_BASE, driverlib::IOID_18, driverlib::IOID_19);

            // Initialize I2C slave.
            driverlib::I2CSlaveInit(driverlib::I2C0_BASE, slave_addr);
            driverlib::I2CSlaveIntEnable(
                driverlib::I2C0_BASE,
                driverlib::I2C_SLAVE_INT_STOP | driverlib::I2C_SLAVE_INT_START | driverlib::I2C_SLAVE_INT_DATA,
            );
        };

        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };

        Self {
            runner: Runner::new(T::state()),
            _p: PhantomData,
        }
    }

    pub fn into_runner(self) -> Runner<'a> {
        self.runner
    }
}

pub fn new_master<'a, T: Instance>(
    _i2c: Peri<'a, T>,
    irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
) -> I2CMaster<'a, T> {
    I2CMaster::new(irq)
}

pub fn new_slave<'a, T: Instance>(
    _i2c: Peri<'a, T>,
    slave_addr: u8,
    irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
) -> I2CSlave<'a, T> {
    I2CSlave::new(slave_addr, irq)
}

pub fn new_master_slave<'a, T: Instance>(
    _i2c: Peri<'a, T>,
    slave_addr: u8,
    irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
) -> (I2CMaster<'a, T>, I2CSlave<'a, T>) {
    (I2CMaster::new(irq), I2CSlave::new(slave_addr, irq))
}

use embassy_sync::waitqueue::AtomicWaker;
pub(crate) use impl_i2c;
