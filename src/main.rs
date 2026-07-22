// https://github.com/niekiran/embedded-rust
// cargo flash --chip STM32F103C8T6
#![no_std]
#![no_main]

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::{panic::PanicInfo, sync::atomic::AtomicBool, sync::atomic::Ordering};

mod startup_stm32f103;
mod debug;
mod utils;
mod checksum;

mod mcu;
mod rcc;
mod exti;
mod gpio;
mod usart;
mod crc;
mod watchdog;
mod irq;
mod led;
mod i2c;

mod usb_cdc;
mod usb_core;
mod usb_class;
mod usb_control;
mod usb_driver;
mod usb_endpoint;
mod usb_types;
mod usb_peripheral;

mod gps_neo6m;
mod icm20948;
mod fusion;

mod bridge;
mod ccb;
mod kalman_filter;

/* 
mod mpu6050;
mod mpu9250;
*/

/*
 * PANIC HANDLER
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {}
}

/*
 * CALLBACKS IT
 */
#[no_mangle]
pub extern "C" fn SysTick_Handler()
{
    loop {}
}

#[no_mangle]
pub extern "C" fn USB_LP_CAN_RX0_Handler()
{
    // usb_peripheral::handle_usb_interrupt();
    usb_cdc::handle_usb_interrupt();
}

#[no_mangle]
pub extern "C" fn EXTI4_Handler()
{
    send_mpu_data();
    exti::clear_pending_interrupt(mcu::GPIO04);
}

#[no_mangle]
pub extern "C" fn USART1_Handler()
{
    unsafe
    {
        let usart1_sr = (mcu::USART1_BASE + mcu::USART_SR) as *const u32;
        let usart1_dr = (mcu::USART1_BASE + mcu::USART_DR) as *mut u32;

        let sr = utils::read_register32(usart1_sr);
        if (sr & mcu::USART_SR_RXNE) != 0
        {
            let data = utils::read_register32(usart1_dr) as u8;
            gps_neo6m::push_byte(data);

            //while (utils::read_register32(usart1_sr) & mcu::USART_SR_TXE) == 0 {}
            // utils::write_register32(usart1_dr, data as u32);
            //usart::write(usart::Usart::Usart2, data);
        }
    }
}

/*
 * BRIDGE TX
 */
static MTX_SEND_FRAME: AtomicBool = AtomicBool::new(false);
fn send_frame(frame: &bridge::FrameTx)
{
    if MTX_SEND_FRAME.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err()
    {
        return;
    }
    
    usart::write(usart::Usart::Usart2, 0x7E);
    usart::write_bytes(usart::Usart::Usart2, utils::as_bytes(&frame.header));
    usart::write_bytes(usart::Usart::Usart2, frame.payload);
    usart::write_bytes(usart::Usart::Usart2, utils::as_bytes(&frame.crc));
    
    MTX_SEND_FRAME.store(false, Ordering::Release);
}

fn send_mpu_data()
{
    // Read ICM20948 data
    let (acc_x, acc_y, acc_z) = icm20948::accel_g(&i2c::I2C::I2C1, icm20948::AccelRange::G2);
    let (gyr_x, gyr_y, gyr_z) = icm20948::gyro_dps(&i2c::I2C::I2C1, icm20948::GyroRange::D500);
    let temp_c                          = icm20948::temperature_c(&i2c::I2C::I2C1);
    // let (mag_x, mag_y, mag_z) = icm20948::mag_raw(&i2c::I2C::I2C1);

    icm20948::clear_data_ready(&i2c::I2C::I2C1);

    let mut payload = [0u8; 28];
    let frame = bridge::get_package_mpu_data(&mut payload, acc_x, acc_y, acc_z, gyr_x, gyr_y, gyr_z, temp_c);
    send_frame(&frame);
}

fn send_acc_data(x: f32, y: f32, z: f32)
{
    let mut payload = [0u8; 12];
    let frame = bridge::get_package_acc_data(&mut payload, x, y, z);
    send_frame(&frame);
}

fn send_gyr_data(x: f32, y: f32, z: f32)
{
    let mut payload = [0u8; 12];
    let frame = bridge::get_package_gyr_data(&mut payload, x, y, z);
    send_frame(&frame);
}

/*
 * CALLBACKS DATA
 */
fn cb_line_from_gps(line: &str)
{
    let frame: bridge::FrameTx = bridge::get_gps_data(line.as_bytes());
    send_frame(&frame);
}

fn cb_coords_from_gps(lat: f32, lng: f32, height : f32)
{
    let mut payload = [0u8; 12];
    let frame: bridge::FrameTx = bridge::get_gps_coords(&mut payload, lat, lng, height);
    send_frame(&frame);
}

/*
 * MAIN
 */
#[no_mangle]
fn main() -> !
{
    // Set system clock
    mcu::init_clock(mcu::SysClock::HSE48MHz);

    // RCC peripheral clocks
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::Afio);
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPc);
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPb);
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPa);
    // rcc::ahb::enable(rcc::ahb::AHBPeripheral::Crc);
    
    // IWDG
    // watchdog::iwdg::init(500);
    
    // Debug
    debug::init(usart::Usart::Usart2, debug::DebugLevel::Verbose);

    debug!(debug::DebugLevel::Info, "=== SENSOR FUSION ===");
    debug!(debug::DebugLevel::Info, "v{}.{}.{}.{}", 0,0,0,1);

    let mut cb: ccb::CircularBuffer<u8, 8> = ccb::CircularBuffer::new();
    /*
     * Test Circular buffer
    cb.push(10).unwrap();
    cb.push(20).unwrap();
    cb.push(30).unwrap();

    assert_eq!(cb.len(), 3);

    assert_eq!(cb.pop(), Some(10));
    assert_eq!(cb.pop(), Some(20));

    cb.push(40).unwrap();
    cb.push(50).unwrap();

    assert_eq!(cb.peek(), Some(&30));
     */

    // PC13 (LED)
    gpio::configure_pin(mcu::GPIOC_BASE, mcu::GPIO13, gpio::GpioMode::Output, gpio::GpioConfig::PushPull, Some(gpio::GpioSpeed::Speed2MHz));
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO00, gpio::GpioMode::Output, gpio::GpioConfig::PushPull, Some(gpio::GpioSpeed::Speed2MHz));
    //
    // PA8 (MCO)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO08, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));

    // USB
    // PA11 (D-)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO11, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz)); // DM
    // PA12 (D+)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO12, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz)); // DP
    // PA15 (USB Pull-up)
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO15, gpio::GpioMode::Output, gpio::GpioConfig::PushPull, Some(gpio::GpioSpeed::Speed2MHz));
    // usb_peripheral::init();
    usb_cdc::init();

    loop
    {
        unsafe
        {
            let istr = utils::read_register16(mcu::USB_ISTR as *const u16);
            let ep0 = utils::read_register16(mcu::USB_EP0R as *const u16);
            let cfgr = utils::read_register32(mcu::RCC_CFGR as *const u32);
            
            led::led_toggle(mcu::GPIOC_BASE, mcu::GPIO13);
            utils::delay_ms(100);
        }
    }

    
    // USART1 (GPS)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO09, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO10, gpio::GpioMode::Input, gpio::GpioConfig::Floating, None);
    usart::start(usart::Usart::Usart1, usart::UsartMode::TxRx, usart::UsartInterrupt::RxInterrupt, usart::UsartBaudRate::B9600, usart::UsartWordLength::Length8Bits, usart::UsartStopBits::Stop1Bit, usart::UsartParity::None);
    gps_neo6m::init(usart::Usart::Usart1, gps_neo6m::GPSProtocol::NMEA, gps_neo6m::GPSBaudRate::B9600, gps_neo6m::GPSUpdateRate::R20Hz, gps_neo6m::GPSOperationMode::Normal, &[gps_neo6m::GPSNmeaSentence::GGA, gps_neo6m::GPSNmeaSentence::RMC], cb_line_from_gps, cb_coords_from_gps);
    irq::enable_irq(irq::IRQn::USART1 as u32);

    // I2C1 (ICM20948)
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO06, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfOpenDrain, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO07, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfOpenDrain, Some(gpio::GpioSpeed::Speed50MHz));
    i2c::start(i2c::I2C::I2C1, i2c::I2CClockSpeed::Standard100kHz);
    icm20948::init(&i2c::I2C::I2C1, icm20948::AccelRange::G2, icm20948::GyroRange::D250);
    // ICM20948 Pin IT
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO04, gpio::GpioMode::Input, gpio::GpioConfig::PullUpDown, None);
    gpio::write_pin(mcu::GPIOB_BASE, mcu::GPIO04, true);
    exti::gpio::set_edge(mcu::GPIO04, exti::gpio::EdgeTrigger::Rising);
    exti::configure_afio(exti::cfg_by_port(mcu::GPIOB_BASE), mcu::GPIO04);
    exti::enable_interrupt(mcu::GPIO04);
    exti::clear_pending_interrupt(mcu::GPIO04);
    // irq::enable_irq(irq::IRQn::EXTI4 as u32);
    
    loop
    {
        // Refresh IWDG
        watchdog::iwdg::refresh();
        // Toggle LED on PC13
        led::led_toggle(mcu::GPIOC_BASE, mcu::GPIO13);
        // Process GPS data
        gps_neo6m::process_gps();
        // Send ICM20948 data
        send_mpu_data();
        // Delay
        utils::delay_ms(50);
    }
}