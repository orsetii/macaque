pub mod uart_16550;

use uart_16550::SerialPort;

lazy_static::lazy_static! {
    pub static ref SERIAL: SerialPort = SerialPort::new(0x1000_0000);
}

#[macro_export]
macro_rules! print {
     	($($args:tt)+) => ({
 			use core::fmt::Write;
 			let _ = write!(crate::drivers::serial::SERIAL.lock(), $($args)+);
 	});
 }

#[macro_export]
macro_rules! println
 {
 	() => ({
 		crate::print!("\r\n")
 	});
 	($fmt:expr) => ({
 		crate::print!(concat!($fmt, "\r\n"))
 	});
 	($fmt:expr, $($args:tt)+) => ({
 		crate::print!(concat!($fmt, "\r\n"), $($args)+)
 	});
 }
