//! This module defines an abstraction for the UARTE instance.
//! This abstraction initalizes the peripheral with the passed configuration,
//! sets up a DMA read buffer and configures interrupts. It uses
//! a timer and a PPI channel to time out RX tasks, so the device can react to messages
//! before the buffer is full. It also defines a method for setting up a TX transaction.

use crate::hal;
use core::marker::PhantomData;

pub use hal::uarte::{Baudrate, Instance as UarteInstance, Parity, Pins, Uarte as HalUarte};
use hal::{
    ppi::ConfigurablePpi,
    timer::{Instance as TimerInstance, Periodic},
    Timer,
};

use self::rx_buffer::UarteRxBuffer;
pub enum UarteEvent {
    EndRx, // End of RX transaction
    EndTx, // End of TX transaction
           // Add more variants as you expect more to occur
}

/// Wrapper for UARTE that uses a timer and the PPI
/// to time out RX transactions. This type
/// contains two PhantomData fields,
/// which are used only to notify the compiler
/// of the timer and PPI channel used to initialize
/// the TimeoutUarte.
pub struct TimeoutUarte<U, T, P> {
    uarte: U,
    buffer: UarteRxBuffer,
    endtx_raised: bool,
    timer: PhantomData<T>,
    ppi_channel: PhantomData<P>,
}

impl<U, T, P> TimeoutUarte<U, T, P>
where
    U: UarteInstance,
    T: TimerInstance,
    P: ConfigurablePpi,
{
    /// Initialize the UARTE peripheral. Set up interrupts, DMA,
    /// and connect the timer interrupt to the PPI channel.
    pub fn init(
        uarte: U,
        pins: Pins,
        parity: Parity,
        baudrate: Baudrate,
        timer: Timer<T, Periodic>,
        mut ppi_channel: P,
    ) -> Self {
        let mut buffer = UarteRxBuffer::take().expect("UarteRxBuffer is already taken");

        // We want to use advanced features that the HAL sadly does not implement.
        // Therefore, we destruct the Uarte object just created, regaining the UARTE0 peripheral
        // This way, we can still use the HAL for the initalization code.
        let (uarte, pins) = HalUarte::new(uarte, pins, parity, baudrate).free();

        // We don't want the pins to be de-initialized on drop,
        // so we just forget about them.
        core::mem::forget(pins);

        // Now we set up the uarte0 peripheral.
        let buffer_slice = buffer.as_mut_slice();
        let slice_len = buffer_slice.len() as u16;

        // Set up DMA for RX transactions
        // Point to RX buffer
        uarte
            .rxd
            .ptr
            .write(|w| unsafe { w.ptr().bits(buffer_slice.as_ptr() as u32) });

        // Specify RX buffer size
        uarte
            .rxd
            .maxcnt
            .write(|w| unsafe { w.maxcnt().bits(slice_len) });

        // Setup interrupts, listen for RX and TX transactions done events
        uarte
            .intenset
            .write(|w| w.endrx().set_bit().endtx().set_bit());
        uarte.tasks_startrx.write(|w| unsafe { w.bits(0x01) });

        // Free the timer, so we can use its advanced features
        let timer = timer.free();
        let timer_block = timer.as_timer0();

        // Setup PPI channel, connecting the timer event to the STOPRX task.
        // The STOPRX task finishes the current RX transaction,
        // flushing its FIFO to the DMA buffer. In combination with the ENDRX event,
        // this allows us to time out RX transactions and handle any data in the buffer
        // early.
        ppi_channel.set_task_endpoint(&uarte.tasks_stoprx);
        ppi_channel.set_event_endpoint(&timer_block.events_compare[0]);
        ppi_channel.enable();

        Self {
            uarte,
            buffer,
            endtx_raised: false,
            timer: PhantomData,
            ppi_channel: PhantomData,
        }
    }

    // *Try* to start a TX transaction. This method returns an Err variant
    // if there is already a TX transaction going on.
    // In that case, this method should be called another time.
    pub fn try_start_tx(&mut self, bytes: &[u8]) -> Result<(), ()> {
        // Check whether a TX transaction has started.
        if self.uarte.events_txstarted.read().bits() == 0x01 {
            // Check whether the last TX transaction has finished
            if !self.endtx_raised {
                // If not, there's a write transaction started, and it's not done yet.
                // Return Err variant
                return Err(());
            }
            // If the last transaction has finished, clear the flag
            self.endtx_raised = false;
            // Clear TX started flag
            self.uarte.events_txstarted.reset();
        }

        let slice_len = bytes.len() as u16;

        // Setup transaction parameters for DMA
        // Point to TX buffer
        self.uarte
            .txd
            .ptr // Where to find the data
            .write(|w| unsafe { w.ptr().bits(bytes.as_ptr() as u32) });
        // Specify TX buffer size
        self.uarte
            .txd
            .maxcnt // The length of the data
            .write(|w| unsafe { w.maxcnt().bits(slice_len) });
        // Start read transaction
        self.uarte.tasks_starttx.write(|w| unsafe { w.bits(0x01) });
        Ok(())
    }

    // Get and clear UARTE events. To be used in interrupt handlers.
    pub fn get_clear_event(&mut self) -> Option<UarteEvent> {
        if self.uarte.events_endrx.read().bits() == 0x01 {
            // Start a new read transaction
            self.uarte.tasks_startrx.write(|w| unsafe { w.bits(0x01) });
            // Clear interrupt flag
            self.uarte.events_endrx.reset();
            return Some(UarteEvent::EndRx);
        }
        if self.uarte.events_endtx.read().bits() == 0x01 {
            self.uarte.events_endtx.reset();
            // Set flag
            self.endtx_raised = true;
            return Some(UarteEvent::EndTx);
        }

        None
    }

    /// Get received chunk, that exists in DMA
    pub fn get_rx_chunk(&mut self) -> &'static [u8] {
        let chunk_len = self.uarte.rxd.amount.read().amount().bits() as usize;

        &self.buffer.as_mut_slice()[0..chunk_len]
    }
}

mod rx_buffer {
    use core::{
        marker::PhantomData,
        sync::atomic::{AtomicBool, Ordering},
    };

    // Size of the buffer in bytes
    // Don't use a buffer bigger than 255 bytes,
    // as the nRF52832 can't handle them
    const BUFFER_SIZE: usize = 255;

    // A static, mutable buffer for use in DMA transactions.
    // Static mutables are very scary, so we make sure only one
    // reference to it exists at any moment.
    static mut UARTE_RX_BUFFER: [u8; BUFFER_SIZE] = [0u8; BUFFER_SIZE];
    // Atomic flag that keeps track of whether the buffer was already
    // taken.
    static BUFFER_TAKEN: AtomicBool = AtomicBool::new(false);

    // This type is used to keep track of the RX buffer ownership.
    pub struct UarteRxBuffer {
        _marker: PhantomData<bool>,
    }

    impl UarteRxBuffer {
        // This methos will only return Some the first time it's called.
        pub fn take() -> Option<Self> {
            if BUFFER_TAKEN.swap(true, Ordering::Relaxed) {
                return None;
            }
            Some(Self {
                _marker: PhantomData,
            })
        }
        // Get a unique reference to the buffer
        pub fn as_mut_slice(&mut self) -> &'static mut [u8] {
            // Note(unsafe) `self` can only be instantiated once
            // therefore, there is only one mutable reference to it at
            // any time.
            unsafe { &mut UARTE_RX_BUFFER }
        }
    }
}
