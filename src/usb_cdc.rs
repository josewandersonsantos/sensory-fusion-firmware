//! Implementation USB CDC (Communications Device Class)

#![allow(non_snake_case)]
#![allow(unused_variables)]

use crate::mcu;
use crate::utils;
use crate::usb_types;
use crate::usb_driver;
use crate::usb_control;
use crate::usb_endpoint;

const DEVICE_DESCRIPTOR: [u8; 18] =
[
    0x12,         // bLength
    1,            // bDescriptorType = DEVICE
    0x00, 0x02,   // bcdUSB = 2.00
    0x00,         // bDeviceClass
    0x00,         // bDeviceSubClass
    0x00,         // bDeviceProtocol
    0x40,         // bMaxPacketSize0 = 64 bytes
    // 0x34, 0x12,   // idVendor  (0x1234)
    0x83, 0x04,   // idVendor  (0x0483 is STMicroelectronics' VID for testing)
    0x78, 0x56,   // idProduct (0x5678)
    0x00, 0x01,   // bcdDevice
    0x01,         // iManufacturer
    0x2,          // iProduct
    0x3,          // iSerialNumber
    0x1           // bNumConfigurations
];

const CONFIG_DESCRIPTOR: [u8; 25] =
[
    // CONFIG
    0x09, 0x02,
    0x19, 0x00, // total length = 25
    0x01,       // 1 interface
    0x01,
    0x00,
    0x80,
    0x32,

    // INTERFACE
    0x09, 0x04,
    0x00, // interface 0
    0x00,
    0x01, // 1 endpoint
    0xFF, // vendor specific
    0x00,
    0x00,
    0x00,

    // ENDPOINT IN
    0x07, 0x05,
    0x81, // IN EP1
    0x02, // bulk
    0x40, 0x00,
    0x00
];

const CDC_STRING0: [u8; 4] =
[
    0x04, 0x03,
    0x09, 0x04,
];

const CDC_STRING1: [u8; 10] =
[
    10, 0x03,
    b'S', 0, b'T', 0, b'M', 0, b' ', 0
];

const CDC_STRING2: [u8; 16] =
[
    16, 0x03,
    b'U',0, b'S',0, b'B',0, b' ',0,
    b'D',0, b'e',0, b'v',0
];

const CDC_STRING3: [u8; 10] =
[
    10, 0x03,
    b'1',0, b'2',0, b'3',0, b'4',0
];

const STRING_DESCRIPTORS: [&'static [u8]; 4] =
[
    &CDC_STRING0, &CDC_STRING1, &CDC_STRING2, &CDC_STRING3,
];

const DESCRIPTORS: usb_types::Descriptors = usb_types::Descriptors
{
    device_descriptor: DEVICE_DESCRIPTOR,
    config_descriptor: CONFIG_DESCRIPTOR,
    string0: CDC_STRING0,
    string1: CDC_STRING1,
    string2: CDC_STRING2,
    string3: CDC_STRING3,
};

static mut EP0_CONTROL: usb_endpoint::Endpoint = usb_endpoint::Endpoint
{
    number: usb_types::Endpoints::EP0,
    address: 0x00,
    ep_type: usb_types::EndpointType::CONTROL,

    state: usb_endpoint::EndpointState::Idle,
    data_buffer: [0; 64],
    length: 0,
    position: 0,
    tx_addr: 0x40,    // ADDR_TX field in BTABLE
    rx_addr: 0x80,    // ADDR_RX field in BTABLE
    tx_count: 0x00,   // COUNT_TX field in BTABLE
    rx_count: 0x8400, // COUNT_RX field in BTABLE (for OUT endpoints, this is set by hardware to the number of bytes received)
};

static mut EP1_BULK_IN: usb_endpoint::Endpoint = usb_endpoint::Endpoint
{
    number: usb_types::Endpoints::EP1,
    address: 0x81, // IN endpoint 1
    ep_type: usb_types::EndpointType::BULK,

    state: usb_endpoint::EndpointState::Idle,
    data_buffer: [0; 64],
    length: 0,
    position: 0,
    tx_addr: 0x40,  // ADDR_TX field in BTABLE
    rx_addr: 0,     // Not used for IN endpoint
    tx_count: 0,          // COUNT_TX field in BTABLE
    rx_count: 0,          // Not used for IN endpoint
};

static mut EP2_BULK_OUT: usb_endpoint::Endpoint = usb_endpoint::Endpoint
{
    number: usb_types::Endpoints::EP2,
    address: 0x02, // OUT endpoint 2
    ep_type: usb_types::EndpointType::BULK,

    state: usb_endpoint::EndpointState::Idle,
    data_buffer: [0; 64],
    length: 0,
    position: 0,
    tx_addr: 0,     // Not used for OUT endpoint
    rx_addr: 0x80,  // ADDR_RX field in BTABLE
    tx_count: 0,          // Not used for OUT endpoint
    rx_count: 0,          // COUNT_RX field in BTABLE (set by hardware)
};

pub fn handle_usb_interrupt()
{
    handler_endpoint_interrupt();
}

/// USB Low Priority Interrupt Handler
pub fn handler_endpoint_interrupt()
{
    unsafe
    {
        let usb_istr = mcu::USB_ISTR as *mut u16;
        let mut istr = utils::read_register16(usb_istr);
        // Extract endpoint number
        let ep_id = (istr & 0x0F) as usize;

        // ESOF
        if istr & (1 << usb_types::USBISTR::ESOF as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::ESOF as u16);
        }

        // SOF
        if istr & (1 << usb_types::USBISTR::SOF as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::SOF as u16);
        }        

        // RESET
        if istr & (1 << usb_types::USBISTR::RESET as u16) != 0
        {
            // Handle EP0 interrupt on USB reset
            handler_reset(ep_id as usize);
            istr &= !(1 << usb_types::USBISTR::RESET as u16);
        }

        // SUSP (Suspend)
        if istr & (1 << usb_types::USBISTR::SUSP as u16) != 0
        {
            // entra em low power mode
            // let usb_cntr = mcu::USB_CNTR as *mut u16;
            // utils::set_bit16(usb_cntr, 1); // LP_MODE = 1
            istr &= !(1 << usb_types::USBISTR::SUSP as u16);
        }
        
        // WKP (Wakeup)
        if istr & (1 << usb_types::USBISTR::WKUP as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::WKUP as u16);
        }

        // ERR (Error)
        if istr & (1 << usb_types::USBISTR::ERR as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::ERR as u16);
        }

        // PMAOVR (PMA Over/underrun)
        if istr & (1 << usb_types::USBISTR::PMAOVR as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::PMAOVR as u16);
        }

        // Correct Transfer (CTR) interrupt
        if istr & (1 << usb_types::USBISTR::CTR as u16) != 0
        {
            handler_endpoint(ep_id as usize);
            istr &= !(1 << usb_types::USBISTR::CTR as u16);
        }
        utils::write_register16(usb_istr, istr);
    }
}

pub fn handler_reset(epn: usize)
{
    unsafe 
    {
        match epn
        {
            0 =>
            {
                usb_driver::configure_ep(&mut EP0_CONTROL, usb_types::EndpointType::CONTROL);
            }
            1 =>
            {
                usb_driver::configure_ep(&mut EP1_BULK_IN, usb_types::EndpointType::BULK);
            }
            2 =>
            {
                usb_driver::configure_ep(&mut EP2_BULK_OUT, usb_types::EndpointType::BULK);
            }
            _=> {return;}
        }
    }
}
/// Main handler for Endpoint 0 (Control Endpoint)
pub fn handler_endpoint(epn: usize)
{
    unsafe 
    {
        let epr     = usb_driver::get_ep_register(epn);
        let epv          = core::ptr::read_volatile(epr);
        let ep = match epn
        {
            0 => &mut EP0_CONTROL,
            1 => &mut EP1_BULK_IN,
            2 => &mut EP2_BULK_OUT,
            _ => { return; }
        };

        // ========================
        // RX Side (SETUP or OUT packet received)
        // ========================
        if epv & (1 << usb_types::USBEPnR::CTR_RX as u16) != 0 // CTR_RX flag set
        {
            usb_driver::clear_ctr_rx(epn);
            if epv & (1 << usb_types::USBEPnR::SETUP as u16) != 0 // SETUP bit set
            {
                usb_control::handle_setup(ep, &DESCRIPTORS);
            }
            else
            {
                // Regular OUT data packet
                usb_control::handle_out(ep);
            }
        }
        
        // ========================
        // TX Side (IN packet transmission completed)
        // ========================
        if epv & (1 << usb_types::USBEPnR::CTR_TX as u16) != 0
        {
            usb_driver::clear_ctr_tx(epn);
            usb_control::handle_in(ep);
        }
    
    }

}

fn handle_class_request(epn: usize, setup: &[u8; 8]) -> bool
{
    let brequest = setup[1];
    match brequest {
        0x20 => { /* SET_LINE_CODING */ true }
        0x21 => { /* GET_LINE_CODING */ true }
        // ...
        _ => false,
    }
}

pub fn init()
{
    // Initialize USB peripheral
    crate::usb_peripheral::init();

    // Configure endpoints for CDC class
    unsafe 
    {
        usb_driver::configure_ep(&mut EP0_CONTROL, usb_types::EndpointType::CONTROL);
        usb_driver::configure_ep(&mut EP1_BULK_IN, usb_types::EndpointType::BULK);
        usb_driver::configure_ep(&mut EP2_BULK_OUT, usb_types::EndpointType::BULK);

        usb_driver::enable_usb_peripheral();
    }
}

// fn configure_class_endpoints() {
//     // EP1 IN (bulk), EP2 OUT (bulk), EP3 IN (interrupt) etc.
//     usb::hw::configure_ep(1, 0x81, usb_types::EndpointType::BULK, 0xC0, 0x100);
//     usb::hw::configure_ep(2, 0x02, usb_types::EndpointType::BULK, 0x140, 0x180);
//     // ...
// }

fn on_data_out(epn: usize, data: &[u8]) { /* log ou buffer */ }
fn on_data_in(epn: usize) {}