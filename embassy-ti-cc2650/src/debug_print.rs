use core::{
    fmt::{self, Write},
    sync::atomic::{AtomicU8, AtomicU32, Ordering},
};

use crate::scif_uart_emulator::{LOST_BUFFER_SIZE, SCIF_UART_TX_FIFO_MAX_COUNT, ScifUart};
const SC_UART_FREE_THRESHOLD: u32 = 2 * SCIF_UART_TX_FIFO_MAX_COUNT / 4;

// Based on: https://stackoverflow.com/a/39491059
pub(crate) struct LostBytesWriter<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> LostBytesWriter<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        LostBytesWriter { buf, offset: 0 }
    }

    fn written_so_far(&self) -> usize {
        self.offset
    }
}

impl<'a> fmt::Write for LostBytesWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Check if there is space remaining (return error instead of panicking)
        if remainder.len() < bytes.len() {
            return Err(core::fmt::Error);
        }
        // Make the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // Copy
        remainder.copy_from_slice(bytes);

        // Update offset to avoid overwriting
        self.offset += bytes.len();

        Ok(())
    }
}

pub fn debug_print(debug_message: &'static [u8]) {
    // This function should not be called from ISR.
    debug_assert!(crate::pac::CPU_SCS.ICSR().read().VECTACTIVE() == 0);

    static BYTES_LOST: AtomicU32 = AtomicU32::new(0);
    static REMAINING_BYTE: AtomicU8 = AtomicU8::new(0);

    let mut lost_buffer = [0_u8; LOST_BUFFER_SIZE];
    let is_remaining = (REMAINING_BYTE.load(Ordering::Relaxed) != 0) as u32;
    let bytes_lost = BYTES_LOST.load(Ordering::Relaxed);
    let mut debug_msg_len = debug_message.len() as u32;
    let mut debug_msg_iter = 0;

    if bytes_lost > 0 {
        debug_assert!(is_remaining == 0);

        let free = 2 * SCIF_UART_TX_FIFO_MAX_COUNT - (ScifUart::scif_uart_get_tx_fifo_free_slots() as u32);
        if free < SC_UART_FREE_THRESHOLD {
            BYTES_LOST.store(bytes_lost + debug_msg_len, Ordering::Relaxed);
            return;
        }

        let mut writer = LostBytesWriter::new(&mut lost_buffer);
        write!(&mut writer, "\nLOST:{}\n", bytes_lost).unwrap();
        let msg_len = writer.written_so_far();

        if free < msg_len as u32 {
            BYTES_LOST.store(bytes_lost + debug_msg_len, Ordering::Relaxed);
            return;
        }
    }

    if debug_msg_len == 0 {
        return;
    }

    let free = 2 * SCIF_UART_TX_FIFO_MAX_COUNT - (ScifUart::scif_uart_get_tx_fifo_free_slots() as u32);
    if free < debug_msg_len + is_remaining as u32 {
        BYTES_LOST.store(bytes_lost + debug_msg_len + is_remaining - free, Ordering::Relaxed);
        debug_msg_len = free - is_remaining;

        if free == 0 {
            REMAINING_BYTE.store(0, Ordering::Relaxed);
        }
    }

    debug_assert!(free >= 2);
    debug_assert!(debug_msg_len > 0);

    if is_remaining != 0 {
        unsafe {
            ScifUart::scif_uart_tx_put_two_chars(
                REMAINING_BYTE.swap(0, Ordering::Relaxed),
                debug_message[debug_msg_iter],
            );
        };
        debug_msg_iter += 1;
        debug_msg_len -= 1;
    }

    // SAFETY: unwrap happens only if there is at least one element in the debug_message.
    if debug_msg_len % 2 == 1 && *(debug_message.last().unwrap()) != '\n' as u8 {
        REMAINING_BYTE.store(*(debug_message.last().unwrap()), Ordering::Relaxed);
        debug_msg_len -= 1;
    }

    if debug_msg_len > 0 {
        unsafe {
            ScifUart::scif_uart_tx_put_chars(&debug_message[debug_msg_iter..], debug_msg_len as u32);
        };
    }
}
