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
    // ================= CONFIGURATION DESCRIPTOR =================
    0x09,       // bLength (size of this descriptor in bytes)
    0x02,       // bDescriptorType (CONFIGURATION)

    0x19, 0x00, // wTotalLength (total size of all descriptors = 25 bytes)
    0x01,       // bNumInterfaces (only 1 interface)
    0x01,       // bConfigurationValue (ID of this configuration)
    0x00,       // iConfiguration (no string descriptor)

    0x80,       // bmAttributes:
                //  bit7 = 1 (required)
                //  bit6 = 0 (bus-powered device)
                //  bit5 = 0 (no remote wakeup support)

    0x32,       // bMaxPower (100 mA → value * 2 mA)

    // ================= INTERFACE DESCRIPTOR =================
    0x09,       // bLength
    0x04,       // bDescriptorType (INTERFACE)

    0x00,       // bInterfaceNumber (Interface 0)
    0x00,       // bAlternateSetting

    0x01,       // bNumEndpoints (1 endpoint used)

    0xFF,       // bInterfaceClass (Vendor-specific class)
    0x00,       // bInterfaceSubClass
    0x00,       // bInterfaceProtocol

    0x00,       // iInterface (no string descriptor)

    // ================= ENDPOINT DESCRIPTOR =================
    0x07,       // bLength
    0x05,       // bDescriptorType (ENDPOINT)

    0x81,       // bEndpointAddress:
                //  bit7 = 1 → IN direction (device → host)
                //  bits3..0 = endpoint number 1

    0x02,       // bmAttributes (Bulk transfer type)

    0x40, 0x00, // wMaxPacketSize (64 bytes)

    0x00        // bInterval (ignored for bulk endpoints)
];
*/

const CDC_STRING0_LANG: [u8; 4] =
[
    0x04, 0x03,
    0x09, 0x04,
];

const CDC_STRING1_MANUF: [u8; 10] =
[
    10, 0x03,
    b'A', 0, b'C', 0, b'M', 0, b'E', 0
];

const CDC_STRING2_PRODUCT: [u8; 16] =
[
    16, 0x03,
    b'U',0, b'S',0, b'B',0, b' ',0,
    b'D',0, b'e',0, b'v',0
];

const CDC_STRING3_SERIAL: [u8; 10] =
[
    10, 0x03,
    b'1',0, b'2',0, b'3',0, b'4',0
];


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
    0x00, 0x02,   // bcdDevice = 1.00
    0x01,         // Index of string descriptor describing manufacturer (iManufacturer)
    0x02,         // Index of string descriptor describing product (iProduct)
    0x03,         // Index of string descriptor describing device serial number (iSerialNumber)
    0x01          // bNumConfigurations
];

// ==================== CONFIGURATION DESCRIPTOR ====================
const CONFIG_DESCRIPTOR: [u8; 67] =
[
    // ================= CONFIGURATION DESCRIPTOR =================
    0x09,       // bLength (size of this descriptor in bytes)
    0x02,       // bDescriptorType (CONFIGURATION)

    0x43, 0x00, // wTotalLength (total size of all descriptors = 67 bytes)
    0x02,       // bNumInterfaces (2 interfaces: CDC Control + CDC Data)
    0x01,       // bConfigurationValue (ID of this configuration)
    0x00,       // iConfiguration (no string descriptor)
    0xC0,       // bmAttributes:
                //  bit7 = 1 (required)
                //  bit6 = 0 (bus-powered)
                //  bit5 = 0 (no remote wakeup)
    0x32,       // bMaxPower (100 mA → value * 2 mA)

    // ================= INTERFACE 0 (CDC CONTROL) =================
    0x09,       // bLength
    0x04,       // bDescriptorType (INTERFACE)

    0x00,       // bInterfaceNumber (Interface 0)
    0x00,       // bAlternateSetting
    0x01,       // bNumEndpoints (1 interrupt endpoint)
    0x02,       // bInterfaceClass (Communications and CDC Control)
    0x02,       // bInterfaceSubClass (Abstract Control Model - ACM)
    0x01,       // bInterfaceProtocol (AT commands / V.250)
    0x00,       // iInterface (no string)

    // -------- CDC HEADER FUNCTIONAL DESCRIPTOR --------
    0x05,       // bFunctionLength
    0x24,       // bDescriptorType (CS_INTERFACE)
    0x00,       // bDescriptorSubType (Header)
    0x10, 0x01, // bcdCDC (CDC spec version 1.10)

    // -------- CDC CALL MANAGEMENT FUNCTIONAL DESCRIPTOR --------
    0x05,       // bFunctionLength
    0x24,       // bDescriptorType (CS_INTERFACE)
    0x01,       // bDescriptorSubType (Call Management)
    0x00,       // bmCapabilities:
                //  bit1 = device handles call management
                //  bit0 = uses data interface for call management
    0x01,       // bDataInterface (Interface 1 is the data interface)

    // -------- CDC ABSTRACT CONTROL MANAGEMENT (ACM) --------
    0x04,       // bFunctionLength
    0x24,       // bDescriptorType (CS_INTERFACE)
    0x02,       // bDescriptorSubType (ACM)
    0x02,       // bmCapabilities:
                //  supports Set_Line_Coding, Get_Line_Coding, etc.

    // -------- CDC UNION FUNCTIONAL DESCRIPTOR --------
    0x05,       // bFunctionLength
    0x24,       // bDescriptorType (CS_INTERFACE)
    0x06,       // bDescriptorSubType (Union)
    0x00,       // bControlInterface (Interface 0 = control)
    0x01,       // bSubordinateInterface (Interface 1 = data)

    // -------- ENDPOINT 1 (INTERRUPT IN - NOTIFICATION) --------
    0x07,       // bLength
    0x05,       // bDescriptorType (ENDPOINT)

    0x81,       // bEndpointAddress:
                //  bit7 = 1 (IN direction)
                //  bits3..0 = endpoint number 1
    0x03,       // bmAttributes (Interrupt transfer)
    0x08, 0x00, // wMaxPacketSize (8 bytes)
    0xFF,       // bInterval (polling interval = 16 ms)

    // ================= INTERFACE 1 (CDC DATA) =================
    0x09,       // bLength
    0x04,       // bDescriptorType (INTERFACE)

    0x01,       // bInterfaceNumber (Interface 1)
    0x00,       // bAlternateSetting
    0x02,       // bNumEndpoints (2 bulk endpoints)
    0x0A,       // bInterfaceClass (CDC Data)
    0x00,       // bInterfaceSubClass
    0x00,       // bInterfaceProtocol
    0x00,       // iInterface (no string)

    // -------- ENDPOINT 2 (BULK OUT - HOST → DEVICE) --------
    0x07,       // bLength
    0x05,       // bDescriptorType (ENDPOINT)

    0x02,       // bEndpointAddress:
                //  bit7 = 0 (OUT direction)
                //  endpoint number 2
    0x02,       // bmAttributes (Bulk transfer)
    0x40, 0x00, // wMaxPacketSize (64 bytes)
    0x00,       // bInterval (ignored for bulk)

    // -------- ENDPOINT 3 (BULK IN - DEVICE → HOST) --------
    0x07,       // bLength
    0x05,       // bDescriptorType (ENDPOINT)

    0x83,       // bEndpointAddress:
                //  bit7 = 1 (IN direction)
                //  endpoint number 3
    0x02,       // bmAttributes (Bulk transfer)
    0x40, 0x00, // wMaxPacketSize (64 bytes)
    0x00,       // bInterval (ignored for bulk)
];

// */

/*
// ==================== STRING DESCRIPTORS ====================
const CDC_STRING0_LANG: [u8; 4] = [0x04, 0x03, 0x09, 0x04]; // Language ID: English (US)

const CDC_STRING1_MANUF: [u8; 38] = [  // "STMicroelectronics"
    38, 0x03,
    b'S',0, b'T',0, b'M',0, b'i',0, b'c',0, b'r',0, b'o',0,
    b'e',0, b'l',0, b'e',0, b'c',0, b't',0, b'r',0, b'o',0, b'n',0, b'i',0, b'c',0, b's',0
];

const CDC_STRING2_PRODUCT: [u8; 30] = [  // "USB CDC Device"
    30, 0x03,
    b'U',0, b'S',0, b'B',0, b' ',0,
    b'C',0, b'D',0, b'C',0, b' ',0,
    b'D',0, b'e',0, b'v',0, b'i',0, b'c',0, b'e',0
];

const CDC_STRING3_SERIAL: [u8; 18] = [  // "12345678"
    18, 0x03,
    b'1',0, b'2',0, b'3',0, b'4',0, b'5',0, b'6',0, b'7',0, b'8',0
];

const STRING_DESCRIPTORS: [&'static [u8]; 4] =
[
    &CDC_STRING0_LANG, &CDC_STRING1_MANUF, &CDC_STRING2_PRODUCT, &CDC_STRING3_SERIAL,
];

*/

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
    string0: CDC_STRING0_LANG,
    string1: CDC_STRING1_MANUF,
    string2: CDC_STRING2_PRODUCT,
    string3: CDC_STRING3_SERIAL,
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
    configure_epns();
    unsafe
    {
        usb_driver::configure_ep(&mut EP0_CONTROL, usb_types::EndpointType::CONTROL);
        // match epn
        // {
        //     0 =>
        //     {
        //         usb_driver::configure_ep(&mut EP0_CONTROL, EP0_CONTROL.ep_type);
        //     }
        //     1 =>
        //     {
        //         usb_driver::configure_ep(&mut EP1_INTERRUPT_IN, EP1_INTERRUPT_IN.ep_type);
        //     }
        //     2 =>
        //     {
        //         usb_driver::configure_ep(&mut EP2_BULK_OUT, EP2_BULK_OUT.ep_type);
        //     }
        //     3 =>
        //     {
        //         usb_driver::configure_ep(&mut EP3_BULK_IN, EP3_BULK_IN.ep_type);
        //     }
        //     _=> {return;}
        // }
    }
    usb_driver::set_address(0); // Ensure device address is reset to 0
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
                // Return 1 to indicate setup not handled, so class-specific handler can try to process it
                if let Some(setup) = usb_control::handle_setup(ep, &DESCRIPTORS)
                {
                    handle_class_request(ep, setup);
                }
            }
            else
            {
                // Regular OUT data packet
                // usb_control::handle_out(ep);
                handle_out(ep);
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

fn handle_class_request(ep: &mut usb_endpoint::Endpoint, setup: [u8; 8])
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
        // CLEAR_FEATURE (HOST → DEVICE)
        // =========================
        0x01 =>
        {
            if wvalue == 0x00 // ENDPOINT_HALT
            {
                let ep_addr = windex as u8;
                let ep_num = ep_addr & 0x0F;
                let is_in = (ep_addr & 0x80) != 0;

                if is_in
                {
                    usb_driver::set_stat_tx_nak(ep_num as usize);
                }
                else
                {
                    usb_driver::set_stat_rx_valid(ep_num as usize);
                }
            }

            usb_driver::send_zero_length_packet(0);
        }
        // =========================
        // SET_CONFIGURATION (HOST → DEVICE)
        // =========================
        0x09 =>
        {
            // configure_epns();
            // usb_driver::set_stat_tx_nak(0);
            usb_driver::send_zero_length_packet(0);
        }
        // =========================
        // SET_LINE_CODING (HOST → DEVICE)
        // =========================
        0x20 =>
        {
            if wlength == 7
            {
                ep.state = usb_endpoint::EndpointState::DataOut;
                // Set RX to get 7 bytes for line coding
                usb_driver::set_stat_rx_valid(0);
                usb_driver::send_zero_length_packet(0);
            }
            else
            {
                usb_driver::stall_ep(0);
            }
        }
        // =========================
        // GET_LINE_CODING (DEVICE → HOST)
        // =========================
        0x21 =>
        {
            //usb_driver::send_zero_length_packet(0);

            unsafe
            {
                ep.data_buffer[..8].copy_from_slice(&LINE_CODING);
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
            ep.state = usb_endpoint::EndpointState::StatusIn;
            usb_driver::send_zero_length_packet(ep.number as usize);
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