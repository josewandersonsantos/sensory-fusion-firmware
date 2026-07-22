use core::fmt::{self, Write};

use crate::mcu;
use crate::gpio;
use crate::usart;

#[macro_export]
macro_rules! debug
{
    ($lvl:expr, $($arg:tt)*) =>
    {
        $crate::debug::write($lvl, format_args!($($arg)*));
    };
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugLevel
{
    Error,
    Warning,
    Info,
    Verbose
}

static mut OUT_UART : usart::Usart = usart::Usart::Usart1;
static mut OUT_LEVEL : DebugLevel = DebugLevel::Verbose;

struct DebugWriter;

impl Write for DebugWriter
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        unsafe
        {
            usart::write_string(OUT_UART, s);
        }

        Ok(())
    }
}

pub fn write(level: DebugLevel, args: fmt::Arguments)
{
    unsafe
    {
        if level > OUT_LEVEL {
            return;
        }

        match level
        {
            DebugLevel::Error   => usart::write_string(OUT_UART, "[ERROR] "),
            DebugLevel::Warning => usart::write_string(OUT_UART, "[WARN ] "),
            DebugLevel::Info    => usart::write_string(OUT_UART, "[INFO ] "),
            DebugLevel::Verbose => usart::write_string(OUT_UART, "[VERB ] "),
        }
    }

    let mut writer = DebugWriter;

    let _ = writer.write_fmt(args);
    let _ = writer.write_str("\r\n");
}

pub fn init(usart :usart::Usart, level: DebugLevel)
{
    unsafe 
    {
        OUT_LEVEL = level;
        OUT_UART = usart;
    }

    // USART2 (DEBUG)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO02, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO03, gpio::GpioMode::Input, gpio::GpioConfig::Floating, None);
    usart::start(usart::Usart::Usart2, usart::UsartMode::Tx, usart::UsartInterrupt::None, usart::UsartBaudRate::B115200, usart::UsartWordLength::Length8Bits, usart::UsartStopBits::Stop1Bit, usart::UsartParity::None);
}