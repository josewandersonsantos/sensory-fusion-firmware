#![allow(dead_code)]
#![allow(non_camel_case_types)]

/// Packet Memory Area (PMA) base address in the USB peripheral
pub const PMA_BASE: u32 = 0x40006000;

#[repr(u8)]

#[derive(Clone, Copy)]
pub enum Endpoints
{
    EP0 = 0,
    EP1 = 1,
    EP2 = 2,
    EP3 = 3,
    EP4 = 4,
    EP5 = 5,
    EP6 = 6,
    EP7 = 7,
}

#[repr(u8)]
pub enum UsbRequest
{
    GET_STATUS        = 0,
    CLEAR_FEATURE     = 1,
    RESERVED0         = 2,
    SET_FEATURE       = 3,
    RESERVED1         = 4,
    SET_ADDRESS       = 5,
    GET_DESCRIPTOR    = 6,
    SET_DESCRIPTOR    = 7,
    GET_CONFIGURATION = 8,
    SET_CONFIGURATION = 9,
    GET_INTERFACE     = 10,
    SET_INTERFACE     = 11,
    SYNCH_FRAME       = 12,
}

pub enum USBCNTR
{
    FRES = 0,   // Force Reset
    PDWN = 1,   // Power Down
    LPMODE = 2, // Low-power mode
    FSUSP = 3,  // Force Suspend
    RESUME = 4, // Resume request
    ESOFM = 8,  // Start of Frame interrupt mask
    SOFM = 9,   // Start of Frame interrupt mask
    RESETM = 10, // USB Reset interrupt mask
    SUSPM = 11,  // Suspend mode interrupt mask
    WKUPM = 12,  // Wakeup interrupt mask
    ERRM = 13,   // Error interrupt mask
    PMAOVRM = 14, // Packet Memory Area Over/underrun interrupt mask
    CTRM = 15,  // Correct Transfer interrupt mask
}

pub enum USBFNR
{
    FN = 0,     // Frame Number (11 bits)
    LSOFL = 11, // Lost SOF HIGH
    LSOFH = 12, // Lost SOF HIGH
    LCK = 13,   // Locked
    RXDM = 14,  // Receive Data Minus (1 bit)
    RXDP = 15,  // Receive Data Plus (1 bit)
}

#[repr(u8)]
pub enum USBDADDR
{
    ADD = 0,    // Device Address (7 bits)
    EF = 7,     // Enable Function
}

pub enum USBBTABLE
{
    BTABLE = 3, // Base address of the buffer table (in 512-byte units)
}

pub enum USBISTR
{
    ESOF = 8,   // Start of Frame
    SOF = 9,    // Start of Frame
    RESET = 10, // USB Reset
    SUSP = 11,  // Suspend mode
    WKUP = 12,  // Wakeup
    ERR = 13,   // Error
    PMAOVR = 14, // Packet Memory Area Over/underrun
    CTR = 15,   // Correct Transfer
}

pub enum USBBCDR
{
    DPPU = 15, // D+ Pull-up
}

pub enum USBEPnR
{
    EA      = 0,  // Endpoint Address (4 bits)
    STAT_TX = 4,  // Status bits for transmission
    DTOG_TX = 6,  // Data Toggle for transmission
    CTR_TX  = 7,  // Correct Transfer for transmission
    EP_KIND = 8,  // Endpoint Kind
    EP_TYPE = 9,  // Endpoint Type (2 bits) 
    SETUP   = 11, // Setup transaction completed
    STAT_RX = 12, // Status bits for reception
    DTOG_RX = 14, // Data Toggle for reception
    CTR_RX  = 15, // Correct Transfer for reception
}

pub enum STATRX_Status
{
    VALID    = 0b11,  // Valid
    NAK      = 0b10,  // NAK
    STALL    = 0b01,  // STALL
    DISABLED = 0b00,  // Disabled
}

pub enum STATTX_Status
{
    VALID    = 0b11,  // Valid
    NAK      = 0b10,  // NAK
    STALL    = 0b01,  // STALL
    DISABLED = 0b00,  // Disabled
}

#[repr(u8)]
pub enum BTABLE_ADDRESS
{
    EP0_ADDR_TX = 0x00,   // Endpoint 0 Address of the TX buffer for the endpoint
    EP0_COUNT_TX = 0x02,  // Endpoint 0 Number of bytes to transmit (for IN endpoints)
    EP0_ADDR_RX = 0x04,   // Endpoint 0 Address of the RX buffer for the endpoint
    EP0_COUNT_RX = 0x06,  // Endpoint 0 Number of bytes received (for OUT endpoints)
    
    EP1_ADDR_TX = 0x08,   // Endpoint 1 Address of the TX buffer for the endpoint
    EP1_COUNT_TX = 0x0A,  // Endpoint 1 Number of bytes to transmit (for IN endpoints)
    EP1_ADDR_RX = 0x0C,   // Endpoint 1 Address of the RX buffer for the endpoint
    EP1_COUNT_RX = 0x0E,  // Endpoint 1 Number of bytes received (for OUT endpoints)

    EP2_ADDR_TX = 0x10,   // Endpoint 2 Address of the TX buffer for the endpoint
    EP2_COUNT_TX = 0x12,  // Endpoint 2 Number of bytes to transmit (for IN endpoints)
    EP2_ADDR_RX = 0x14,   // Endpoint 2 Address of the RX buffer for the endpoint
    EP2_COUNT_RX = 0x16,  // Endpoint 2 Number of bytes received (for OUT endpoints)
    
    /*
     *
     */
}

#[derive(Clone, Copy)]
pub enum EndpointType
{
    BULK        = 0,
    CONTROL     = 1,
    ISOCHRONOUS = 2,
    INTERRUPT   = 3,
}

#[repr(C)]
pub struct SetupPacket
{
    pub bm_request: u8,
    pub b_request: u8,
    pub w_value: u16,
    pub w_index: u16,
    pub w_length: u16,
}