//! Implementation USB CDC (Communications Device Class)

#![allow(non_snake_case)]
#![allow(unused_variables)]

use crate::mcu;
use crate::utils;
use crate::usb_types;
use crate::usb_driver;
use crate::usb_control;
use crate::usb_endpoint;

/*
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
    b'A', 0, b'C', 0, b'M', 0, b'E', 0
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
*/

// /*
// ==================== DEVICE DESCRIPTOR ====================
const DEVICE_DESCRIPTOR: [u8; 18] =
[
    0x12,         // bLength
    0x01,         // bDescriptorType = DEVICE
    0x00, 0x02,   // bcdUSB = 2.00
    0x02,         // bDeviceClass = CDC
    0x00,         // bDeviceSubClass
    0x00,         // bDeviceProtocol
    0x40,         // bMaxPacketSize0 = 64
    0x83, 0x04,   // idVendor  = 0x0483 (ST)
    0x78, 0x56,   // idProduct = 0x5678
    0x00, 0x01,   // bcdDevice = 1.00
    0x01,         // iManufacturer
    0x02,         // iProduct
    0x03,         // iSerialNumber
    0x01          // bNumConfigurations
];

// ==================== CONFIGURATION DESCRIPTOR ====================
const CONFIG_DESCRIPTOR: [u8; 67] =
[
    // CONFIG
    0x09, 0x02,
    0x43, 0x00, // total length = 67
    0x02,       // 2 interfaces
    0x01,
    0x00,
    0x80,
    0x32,

    // INTERFACE 0
    0x09, 0x04,
    0x00, // interface 0
    0x00,
    0x01, // 1 endpoint
    0x02, // CDC
    0x02, // ACM
    0x01,
    0x00,

    // HEADER
    0x05, 0x24, 0x00, 0x10, 0x01,

    // CALL MANAGEMENT
    0x05, 0x24, 0x01, 0x03, 0x01,

    // ACM
    0x04, 0x24, 0x02, 0x02,

    // UNION
    0x05, 0x24, 0x06, 0x00, 0x01,

    // ENDPOINT IN (CONTROL)
    0x07, 0x05,
    0x81, // IN EP1
    0x03, // interrupt
    0x08, 0x00,
    0x10,
    
    // INTERFACE 1 DATA
    0x09, 0x04,
    0x01, // interface 1
    0x00,
    0x02, // 2 endpoints
    0x0A, // DATA CLASS
    0x00,
    0x00,
    0x00,
    
    // BULK OUT
    0x07, 0x05,
    0x02, // EP2 OUT
    0x02, // BULK
    0x40, 0x00,
    0x00,
    
    // BULK IN
    0x07, 0x05,
    0x83, // EP3 IN
    0x02,
    0x40, 0x00,
    0x00,
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
// */

const STRING_DESCRIPTORS: [&'static [u8]; 4] =
[
    &CDC_STRING0, &CDC_STRING1, &CDC_STRING2, &CDC_STRING3,
];

static mut LINE_CODING: [u8; 8] =
[
    0x00, 0xC2, 0x01, 0x00, // 115200
    0x00, // stop bits
    0x00, // parity
    0x08,  // data bits
    0x00
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
    data_buffer: [0; 128],
    length: 0,
    position: 0,
    tx_addr: 0x40,    // ADDR_TX field in BTABLE
    rx_addr: 0x80,    // ADDR_RX field in BTABLE
    tx_count: 0x00,   // COUNT_TX field in BTABLE
    rx_count: 0x8400, // COUNT_RX field in BTABLE (for OUT endpoints, this is set by hardware to the number of bytes received)
};

static mut EP1_INTERRUPT_IN: usb_endpoint::Endpoint = usb_endpoint::Endpoint
{
    number: usb_types::Endpoints::EP1,
    address: 0x81, // IN endpoint 1
    ep_type: usb_types::EndpointType::INTERRUPT,

    state: usb_endpoint::EndpointState::Idle,
    data_buffer: [0; 128],
    length: 0,
    position: 0,
    tx_addr: 0xC0,  // ADDR_TX field in BTABLE
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
    data_buffer: [0; 128],
    length: 0,
    position: 0,
    tx_addr: 0,     // Not used for OUT endpoint
    rx_addr: 0x100,  // ADDR_RX field in BTABLE
    tx_count: 0,          // Not used for OUT endpoint
    rx_count: 0,          // COUNT_RX field in BTABLE (set by hardware)
};

static mut EP3_BULK_IN: usb_endpoint::Endpoint = usb_endpoint::Endpoint
{
    number: usb_types::Endpoints::EP3,
    address: 0x83, // IN endpoint 3
    ep_type: usb_types::EndpointType::BULK,

    state: usb_endpoint::EndpointState::Idle,
    data_buffer: [0; 128],
    length: 0,
    position: 0,
    tx_addr: 0x140,     // Not used for OUT endpoint
    rx_addr: 0,  // ADDR_RX field in BTABLE
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
                usb_driver::configure_ep(&mut EP0_CONTROL, EP0_CONTROL.ep_type);
            }
            1 =>
            {
                usb_driver::configure_ep(&mut EP1_INTERRUPT_IN, EP1_INTERRUPT_IN.ep_type);
            }
            2 =>
            {
                usb_driver::configure_ep(&mut EP2_BULK_OUT, EP2_BULK_OUT.ep_type);
            }
            3 =>
            {
                usb_driver::configure_ep(&mut EP3_BULK_IN, EP3_BULK_IN.ep_type);
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
            1 => &mut EP1_INTERRUPT_IN,
            2 => &mut EP2_BULK_OUT,
            3 => &mut EP3_BULK_IN,
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
                if usb_control::handle_setup(ep, &DESCRIPTORS) == 1
                {
                    handle_class_request(ep);
                }
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

fn handle_class_request(ep: &mut usb_endpoint::Endpoint)
{
    let mut setup = [0u8; 8];

    usb_driver::pma_read(ep.rx_addr, &mut setup, 8);
    ep.state = usb_endpoint::EndpointState::Setup;

    // bRequest
    let brequesttype = setup[0];
    let brequest     = setup[1];
    let wvalue      = ((setup[3] as u16) << 8) | (setup[2] as u16);
    let windex      = ((setup[5] as u16) << 8) | (setup[4] as u16);
    let wlength     = ((setup[7] as u16) << 8) | (setup[6] as u16);
    
    match brequest
    {
        // =========================
        // SET_LINE_CODING (HOST → DEVICE)
        // =========================
        0x20 =>
        {
            // host will send 7 bytes later
            ep.state = usb_endpoint::EndpointState::DataOut;
        }

        // =========================
        // GET_LINE_CODING (DEVICE → HOST)
        // =========================
        0x21 =>
        {
            unsafe
            {
                ep.data_buffer[..7].copy_from_slice(&LINE_CODING);
            }

            ep.length = 7;
            ep.position = 0;
            ep.state = usb_endpoint::EndpointState::DataIn;

            usb_driver::send_next_packet(ep.number as usize, ep.tx_addr, ep.length, &mut ep.position, &ep.data_buffer);
        }

        // =========================
        // SET_CONTROL_LINE_STATE
        // =========================
        0x22 =>
        {
            // só precisa responder ZLP
            usb_driver::write_count_tx(ep.number as usize, 0);
            usb_driver::set_stat_tx_valid(ep.number as usize);
        }

        _ =>
        {
            usb_driver::stall_ep(ep.number as usize);
        }
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
        usb_driver::configure_ep(&mut EP1_INTERRUPT_IN, usb_types::EndpointType::INTERRUPT);
        usb_driver::configure_ep(&mut EP2_BULK_OUT, usb_types::EndpointType::BULK);
        usb_driver::configure_ep(&mut EP3_BULK_IN, usb_types::EndpointType::BULK);

        usb_driver::enable_usb_peripheral();
    }
}

fn on_data_out(epn: usize, data: &[u8]) { /* log or buffer */ }
fn on_data_in(epn: usize) {}