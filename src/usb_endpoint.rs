//! Endpoint definition
use crate::usb_types;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndpointState
{
    Idle,       // Waiting for SETUP packet
    Setup,      // SETUP packet received
    DataIn,     // Sending data to host (IN)
    DataOut,    // Receiving data from host (OUT) - not used in this minimal version
    StatusIn,   // Status stage (IN)
    StatusOut,  // Status stage (OUT)
}

#[derive(Clone, Copy, Debug)]
pub struct Endpoint
{
    pub number: usb_types::Endpoints,
    pub address: u8,
    pub ep_type: usb_types::EndpointType,
    pub state: EndpointState,
    pub data_buffer: [u8; 128],
    pub length: usize,
    pub position: usize,
    pub tx_addr: u16,
    pub rx_addr: u16,
    pub tx_count: u16,
    pub rx_count: u16,
}

// DEFAULT_EP sem os descritores (eles vão pro trait)
const DEFAULT_EP: Endpoint = Endpoint
{
    number: usb_types::Endpoints::EP0,
    address: 0x00,
    ep_type: usb_types::EndpointType::CONTROL,

    state: EndpointState::Idle,
    data_buffer: [0; 128],
    length: 0,
    position: 0,
    tx_addr: 0,  // ADDR_TX field in BTABLE
    rx_addr: 0,  // ADDR_RX field in BTABLE
    tx_count: 0, // COUNT_TX field in BTABLE
    rx_count: 0, // COUNT_RX field in BTABLE (for OUT endpoints, this is set by hardware to the number of bytes received)
};
