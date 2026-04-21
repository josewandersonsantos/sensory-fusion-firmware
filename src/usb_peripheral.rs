//! USB peripheral initialization and management for STM32F103C8T6 (Blue Pill)
//! This module sets up the USB peripheral, configures endpoints, and handles USB interrupts.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::irq;
use crate::utils;
use crate::rcc;
use crate::mcu;
use crate::gpio;
use crate::usb_types;

/// Enables the Buffer Table (BTABLE) and clears EP0 register
fn enable_btable()
{
    // Set BTABLE address to 0x0000 (start of PMA)
    let usb_btable = mcu::USB_BTABLE as *mut u16;
    unsafe { core::ptr::write_volatile(usb_btable, 0x0000); }
}

pub fn reconnect()
{
    // Disconnect (Set PA15 low to power USB pull-down)
    gpio::clear_pin(mcu::GPIOB_BASE, mcu::GPIO15);
    
    // Small delay to ensure host detects disconnection
    utils::delay_ms(500);
    // Connect (Set PA15 high to power USB pull-up)
    gpio::set_pin(mcu::GPIOB_BASE, mcu::GPIO15);
}

/// Initializes the USB peripheral on STM32F103 (BluePill)
pub fn init()
{
    // Enable USB clock
    rcc::apb1::enable(rcc::apb1::Apb1Peripheral::Usb);
    // Reset USB peripheral
    rcc::apb1::reset(rcc::apb1::Apb1Peripheral::Usb);

    // Disconnect to USB host by setting high on PA15
    gpio::clear_pin(mcu::GPIOB_BASE, mcu::GPIO15);
    
    // Clear Power Down
    let usb_cntr = mcu::USB_CNTR as *mut u16;
    utils::clear_bit16(usb_cntr, usb_types::USBCNTR::PDWN as u8); // PDWN = 0
    
    // Small delay after waking up the peripheral
    utils::delay_ms(100);

    // Force Reset bits
    utils::clear_bit16(usb_cntr, usb_types::USBCNTR::FRES as u8); // FRES = 0

    unsafe 
    {
        let usb_istr = mcu::USB_ISTR as *mut u16;
        utils::write_register16(usb_istr, 0x0000);
    }

    // Setup BTABLE
    enable_btable();

    // Setup Endpoint 0
    // usb_endpoint::configure_ep(usb_types::Endpoints::EP0, usb_types::EndpointType::CONTROL);
    
    // Enable Correct Transfer interrupt
    utils::set_bit16(usb_cntr, usb_types::USBCNTR::CTRM as u8); // CTRM
    // Enable Reset interrupt
    utils::set_bit16(usb_cntr, usb_types::USBCNTR::RESETM as u8); // RESETM

    // Enable USB Low Priority interrupt in NVIC
    irq::enable_irq(irq::IRQn::USB_LP_CAN1_RX0 as u32);
    //irq::set_irq_priority(irq::IRQn::USB_LP_CAN1_RX0 as u32, 6);

    //utils::delay_ms(50);
    // Connect (Set PA15 high to power USB pull-up)
    gpio::set_pin(mcu::GPIOB_BASE, mcu::GPIO15);
    
    //reconnect();

}