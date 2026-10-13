//! Compile-only board integration template. Adapt pin routing, clock setup,
//! interrupt lines and the application's executor/startup for the actual board.
#![no_std]

use embassy_hc32::{
    clocks::Clocks,
    gpio::AlternatePin,
    intc::{Line, Route},
    pac,
    usart::{self, Config, Uart, Unit},
};
use embedded_io_async::Write;
// The cortex-m-rt attribute checks the vector against the PAC's interrupt enum.
use pac::interrupt;

#[interrupt]
fn INT000() {
    usart::on_interrupt(Unit::Usart1);
}
#[interrupt]
fn INT001() {
    usart::on_interrupt(Unit::Usart1);
}
#[interrupt]
fn INT002() {
    usart::on_interrupt(Unit::Usart1);
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("UART setup failed: {0}")]
    Uart(#[from] usart::Error),
    #[error("interrupt route failed: {0:?}")]
    Interrupt(embassy_hc32::intc::IrqError),
}

pub struct Console {
    // Field drop order stops UART sources before the routes are released.
    uart: Uart<pac::Usart1>,
    _pins: [AlternatePin; 2],
    _routes: [Route; 3],
}

impl Console {
    /// # Safety
    /// The board has configured clocks, power/SRAM timing and PWC write access.
    /// `pins` are valid USART1 TX/RX functions. This example exclusively owns
    /// INT000..002, whose handlers are defined above. No inherited UART/DMA use
    /// remains. Do not link another template defining these same vectors.
    pub unsafe fn new(
        peripheral: pac::Usart1,
        clocks: Clocks,
        pins: [AlternatePin; 2],
    ) -> Result<Self, Error> {
        // These are chosen unrestricted lines, not hardware-fixed assignments.
        // Invariant checked at compile time: all three vectors exist on this MCU.
        const LINES: [Line; 3] = [
            Line::new(0).expect("INT000 is an implemented vector"),
            Line::new(1).expect("INT001 is an implemented vector"),
            Line::new(2).expect("INT002 is an implemented vector"),
        ];
        // Mid-scale priority on the HC32F460's four-bit NVIC scale.
        const PRIORITY: u8 = 8;
        // SAFETY: the caller provides the configured peripheral/pins/clocks.
        let uart = unsafe { Uart::new(peripheral, clocks.pclk[1], Config::default()) }?;
        // SAFETY: the caller gives this console its three installed vectors;
        // partial initialization drops earlier routes on any failure.
        let routes = unsafe {
            [
                Route::new(Unit::Usart1.receive_event().id(), LINES[0], PRIORITY)
                    .map_err(Error::Interrupt)?,
                Route::new(Unit::Usart1.error_event().id(), LINES[1], PRIORITY)
                    .map_err(Error::Interrupt)?,
                Route::new(Unit::Usart1.complete_event().id(), LINES[2], PRIORITY)
                    .map_err(Error::Interrupt)?,
            ]
        };
        Ok(Self {
            uart,
            _pins: pins,
            _routes: routes,
        })
    }

    /// Write the whole message and wait for the final stop bit on the wire.
    pub async fn send(&mut self, bytes: &[u8]) -> Result<(), usart::Error> {
        self.uart.write_all(bytes).await?;
        self.uart.flush().await
    }
}
