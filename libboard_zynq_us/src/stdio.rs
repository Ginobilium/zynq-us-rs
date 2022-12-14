use core::ops::{Deref, DerefMut};

use crate::uart::Uart;

const UART_RATE: u32 = 115_200;
static mut UART: LazyUart = LazyUart::Uninitialized;

pub enum LazyUart {
    Uninitialized,
    Initialized(Uart),
}

pub fn get_uart() -> &'static mut LazyUart {
    unsafe { &mut UART }
}

/// Deinitialize so that the Uart will be reinitialized on next
/// output.
///
/// Delays so that an outstanding transmission can finish.
pub fn drop_uart() {
    unsafe {
        if let LazyUart::Initialized(uart) = &UART {
            while !uart.tx_idle() {}
        }
        UART = LazyUart::Uninitialized;
    }
}

/// Initializes the UART on first use through `.deref_mut()` for debug
/// output through the `print!` and `println!` macros.
impl Deref for LazyUart {
    type Target = Uart;
    fn deref(&self) -> &Uart {
        match self {
            LazyUart::Uninitialized => panic!("UART not initialized!"),
            LazyUart::Initialized(uart) => uart,
        }
    }
}

impl DerefMut for LazyUart {
    fn deref_mut(&mut self) -> &mut Uart {
        match self {
            LazyUart::Uninitialized => {
                #[cfg(feature = "target_zcu111")]
                let uart = Uart::uart0(UART_RATE);
                *self = LazyUart::Initialized(uart);
                self
            }
            LazyUart::Initialized(uart) => uart,
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut uart = $crate::stdio::get_uart();
        let _ = write!(uart, $($arg)*);
    })
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut uart = $crate::stdio::get_uart();
        let _ = write!(uart, $($arg)*);
        let _ = write!(uart, "\r\n");
        // flush after the newline
        while !uart.tx_idle() {}
    })
}
