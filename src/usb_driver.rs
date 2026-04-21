//! Low-level USB driver: EPn registers, PMA, BTABLE

#![allow(non_snake_case)]
#![allow(unused_variables)]

use crate::mcu;
use crate::utils;
use crate::usb_types;
use crate::usb_endpoint;

// ========================
// ENDPOINT REGISTERS
// ========================
pub const EP_CTR_RX: u16  = 1 << usb_types::USBEPnR::CTR_RX as u8;
pub const EP_DTOG_RX: u16 = 1 << usb_types::USBEPnR::DTOG_RX as u8;
pub const EP_STAT_RX: u16 = (usb_types::STATRX_Status::VALID as u16) << usb_types::USBEPnR::STAT_RX as u8;
pub const EP_SETUP:   u16 = 1 << usb_types::USBEPnR::SETUP as u8;

pub const EP_CTR_TX: u16  = 1 << usb_types::USBEPnR::CTR_TX as u8;
pub const EP_DTOG_TX: u16 = 1 << usb_types::USBEPnR::DTOG_TX as u8;
pub const EP_STAT_TX: u16 = (usb_types::STATTX_Status::VALID as u16) << usb_types::USBEPnR::STAT_TX as u8;

pub const EP_W0_BITS:    u16 = EP_CTR_RX | EP_CTR_TX;
pub const EP_TOGGLE_TX:  u16 = EP_DTOG_RX | EP_DTOG_TX | EP_STAT_RX;
pub const EP_TOGGLE_RX:  u16 = EP_DTOG_RX | EP_DTOG_TX | EP_STAT_TX;
pub const EP_TOGGLE_ALL: u16 = EP_DTOG_RX | EP_DTOG_TX | EP_STAT_TX | EP_STAT_RX;
pub const EP_TOGGLE_STALL: u16 = EP_DTOG_RX | EP_DTOG_TX;

pub const EP_RX_VALID: u16 = (usb_types::STATTX_Status::VALID as u16) << usb_types::USBEPnR::STAT_RX as u8;
pub const EP_TX_VALID: u16 = (usb_types::STATTX_Status::VALID as u16) << usb_types::USBEPnR::STAT_TX as u8;
pub const EP_TX_NAK:   u16 = (usb_types::STATTX_Status::NAK as u16)   << usb_types::USBEPnR::STAT_TX as u8;
pub const EP_TX_STALL: u16 = (usb_types::STATTX_Status::STALL as u16) << usb_types::USBEPnR::STAT_TX as u8;
pub const EP_RX_STALL: u16 = (usb_types::STATRX_Status::STALL as u16) << usb_types::USBEPnR::STAT_RX as u8;

// ========================
// LOW LEVEL USB FUNCTIONS
// ========================
pub fn enable_usb_peripheral()
{
    utils::set_bit16(mcu::USB_DADDR as *mut u16, usb_types::USBDADDR::EF as u8);
}

pub fn get_ep_register(epn: usize) -> *mut u16
{
    let ep = match epn
    {
        0 => mcu::USB_EP0R as *mut u16,
        1 => mcu::USB_EP1R as *mut u16,
        2 => mcu::USB_EP2R as *mut u16,
        3 => mcu::USB_EP3R as *mut u16,
        4 => mcu::USB_EP4R as *mut u16,
        5 => mcu::USB_EP5R as *mut u16,
        6 => mcu::USB_EP6R as *mut u16,
        7 => mcu::USB_EP7R as *mut u16,
        _ => panic!("ep num invalid")
    };

    ep
}

#[inline(always)]
unsafe fn read_ep(epn: usize) -> u16
{
    let epr = get_ep_register(epn);
    core::ptr::read_volatile(epr)
}

#[inline(always)]
unsafe fn write_ep(epn: usize, val: u16)
{
    let epr = get_ep_register(epn);
    core::ptr::write_volatile(epr, val);
}

pub fn clear_ctr_rx(epn: usize)
{
    unsafe
    {
        let mut val = read_ep(epn);

        val &= !EP_CTR_RX;      // Clean CTR_RX
        val &= !EP_TOGGLE_ALL;  // Save toggles

        write_ep(epn, val);
    }
}

pub fn clear_ctr_tx(epn: usize)
{
    unsafe
    {
        let mut val = read_ep(epn);

        val &= !EP_CTR_TX;   // Clean CTR_TX
        val |= EP_CTR_RX;    // Save CTR_RX
        val &= !EP_TOGGLE_ALL;

        write_ep(epn, val);
    }
}

pub fn set_stat_rx_valid(epn: usize)
{
    unsafe
    {
        let mut val = read_ep(epn);

        val &= !EP_CTR_RX;
        val |= EP_CTR_TX;
        val &= !EP_TOGGLE_RX;

        val ^= EP_RX_VALID; // toggle until VALID

        write_ep(epn, val);
    }
}

pub fn set_stat_tx_valid(epn: usize)
{
    unsafe
    {
        let mut val = read_ep(epn);

        val &= !EP_CTR_TX;
        val |= EP_CTR_RX;
        val &= !EP_TOGGLE_TX;

        val ^= EP_TX_VALID;

        write_ep(epn, val);
    }
}

pub fn set_stat_tx_nak(epn: usize)
{
    unsafe
    {
        let mut val = read_ep(epn);

        val &= !EP_CTR_TX;
        val |= EP_CTR_RX;
        val &= !EP_TOGGLE_TX;

        val ^= EP_TX_NAK;

        write_ep(epn, val);
    }
}

pub fn stall_ep(epn: usize)
{
    unsafe
    {
        let mut val = read_ep(epn);

        val &= !(EP_CTR_RX | EP_CTR_TX);
        val &= !EP_TOGGLE_RX;
        val ^= EP_TX_STALL;
        val ^= EP_RX_STALL;

        write_ep(epn, val);
    }
}

// PMA
// Reads data from Packet Memory Area (PMA) into a buffer
// Note: PMA is 16-bit wide, so we handle byte packing manually
pub fn pma_read(addr: u16, buffer: &mut [u8], len: usize)
{
    unsafe
    {
        let n_bytes = (len + 1) >> 1;
        let mut pma = usb_types::PMA_BASE as *mut u16;
        pma = pma.add(addr as usize);
        
        for i in 0..n_bytes
        {
            let word     = core::ptr::read_volatile(pma);
            buffer[i * 2]     = (word & 0xff) as u8;
            buffer[i * 2 + 1] = (word >> 8) as u8;
            pma = pma.add(2);
        }
    }
}

pub fn pma_write(addr: u16, buffer: &[u8], len: usize)
{
    unsafe
    {
        let n_bytes = (len + 1) >> 1;
        let mut pma = usb_types::PMA_BASE as *mut u16;
        pma = pma.add(addr as usize);

        for i in 0..n_bytes
        {
            let mut word: u16 = 0;
            // LSB
            word |= buffer[i * 2] as u16;
            // MSB
            if i * 2 + 1 < buffer.len()
            {
                word |= (buffer[i * 2 + 1] as u16) << 8;
            }
            core::ptr::write_volatile(pma, word);
            pma = pma.add(2);
        }
    }
}

pub fn read_rx_count(epn: usize) -> usize
{
    unsafe
    {
        let pma = usb_types::PMA_BASE as *const u16;
        let addr = match epn
        {
            0 => pma.add(usb_types::BTABLE_ADDRESS::EP0_COUNT_RX as usize) as *const u16,
            1 => pma.add(usb_types::BTABLE_ADDRESS::EP1_COUNT_RX as usize) as *const u16,
            2 => pma.add(usb_types::BTABLE_ADDRESS::EP2_COUNT_RX as usize) as *const u16,
            3 => pma.add(usb_types::BTABLE_ADDRESS::EP3_COUNT_RX as usize) as *const u16,
            4 => pma.add(usb_types::BTABLE_ADDRESS::EP4_COUNT_RX as usize) as *mut u16,
            5 => pma.add(usb_types::BTABLE_ADDRESS::EP5_COUNT_RX as usize) as *mut u16,
            6 => pma.add(usb_types::BTABLE_ADDRESS::EP6_COUNT_RX as usize) as *mut u16,
            7 => pma.add(usb_types::BTABLE_ADDRESS::EP7_COUNT_RX as usize) as *mut u16,
            _ => return 0
        };
        (core::ptr::read_volatile(addr) & 0x03FF) as usize
    }
}

/// Writes the TX byte count for Endpoint 0 into PMA
pub fn write_tx_count(epn: usize, count: u16)
{
    unsafe
    {
        let pma = usb_types::PMA_BASE as *mut u16;
        // GET COUNT_TX BY ENDPOINT
        let addr = match epn
        {
            0 => pma.add(usb_types::BTABLE_ADDRESS::EP0_COUNT_TX as usize) as *mut u16,
            1 => pma.add(usb_types::BTABLE_ADDRESS::EP1_COUNT_TX as usize) as *mut u16,
            2 => pma.add(usb_types::BTABLE_ADDRESS::EP2_COUNT_TX as usize) as *mut u16,
            3 => pma.add(usb_types::BTABLE_ADDRESS::EP3_COUNT_TX as usize) as *mut u16,
            4 => pma.add(usb_types::BTABLE_ADDRESS::EP4_COUNT_TX as usize) as *mut u16,
            5 => pma.add(usb_types::BTABLE_ADDRESS::EP5_COUNT_TX as usize) as *mut u16,
            6 => pma.add(usb_types::BTABLE_ADDRESS::EP6_COUNT_TX as usize) as *mut u16,
            7 => pma.add(usb_types::BTABLE_ADDRESS::EP7_COUNT_TX as usize) as *mut u16,
            _ => return
        };
        core::ptr::write_volatile(addr, count & 0x03FF);
    }
}

/// Sends the next chunk of data during a Data IN stage
pub fn send_next_packet(epn: usize, addr_tx: u16, len: usize, pos: &mut usize, data: &[u8])
{
    let chunk =
    {
        let remaining = len - *pos;
        remaining.min(64)
    };
    
    // Copy data to PMA
    pma_write(addr_tx, &data[*pos..*pos + chunk], chunk);
    *pos += chunk;
    // Update TX count and set TX status to VALID
    write_tx_count(epn, chunk as u16);
    //set_stat_rx_nak(epn);
    for _ in 0..1000 { core::hint::spin_loop(); }
    set_stat_tx_valid(epn);
}

pub fn configure_ep(ep: &mut usb_endpoint::Endpoint, ep_type: usb_types::EndpointType)
{
    let mut btable_rx_count: usize = 0;
    let mut btable_rx_addr: usize  = 0;
    let mut btable_tx_count: usize = 0;
    let mut btable_tx_addr: usize  = 0;

    match ep.number
    {
        // Configures Endpoint 0 buffers and registers
        usb_types::Endpoints::EP0 =>
        {
            btable_rx_count = usb_types::BTABLE_ADDRESS::EP0_COUNT_RX as usize;
            btable_rx_addr  = usb_types::BTABLE_ADDRESS::EP0_ADDR_RX as usize;
            btable_tx_count = usb_types::BTABLE_ADDRESS::EP0_COUNT_TX as usize;
            btable_tx_addr  = usb_types::BTABLE_ADDRESS::EP0_ADDR_TX as usize;
        },
        // Configures Endpoint 1 buffers and registers
        usb_types::Endpoints::EP1 =>
        {
            btable_rx_count = usb_types::BTABLE_ADDRESS::EP1_COUNT_RX as usize;
            btable_rx_addr  = usb_types::BTABLE_ADDRESS::EP1_ADDR_RX as usize;
            btable_tx_count = usb_types::BTABLE_ADDRESS::EP1_COUNT_TX as usize;
            btable_tx_addr  = usb_types::BTABLE_ADDRESS::EP1_ADDR_TX as usize;
        },
        // Configures Endpoint 2 buffers and registers
        usb_types::Endpoints::EP2 =>
        {
            btable_rx_count = usb_types::BTABLE_ADDRESS::EP2_COUNT_RX as usize;
            btable_rx_addr  = usb_types::BTABLE_ADDRESS::EP2_ADDR_RX as usize;
            btable_tx_count = usb_types::BTABLE_ADDRESS::EP2_COUNT_TX as usize;
            btable_tx_addr  = usb_types::BTABLE_ADDRESS::EP2_ADDR_TX as usize;
        },
        // Configures Endpoint 3 buffers and registers
        usb_types::Endpoints::EP3 =>
        {
            btable_rx_count = usb_types::BTABLE_ADDRESS::EP3_COUNT_RX as usize;
            btable_rx_addr  = usb_types::BTABLE_ADDRESS::EP3_ADDR_RX as usize;
            btable_tx_count = usb_types::BTABLE_ADDRESS::EP3_COUNT_TX as usize;
            btable_tx_addr  = usb_types::BTABLE_ADDRESS::EP3_ADDR_TX as usize;
        },
        // Configures Endpoint 4 buffers and registers
        usb_types::Endpoints::EP4 =>
        {
            btable_rx_count = usb_types::BTABLE_ADDRESS::EP4_COUNT_RX as usize;
            btable_rx_addr  = usb_types::BTABLE_ADDRESS::EP4_ADDR_RX as usize;
            btable_tx_count = usb_types::BTABLE_ADDRESS::EP4_COUNT_TX as usize;
            btable_tx_addr  = usb_types::BTABLE_ADDRESS::EP4_ADDR_TX as usize;
        },
        // Configures Endpoint 5 buffers and registers
        usb_types::Endpoints::EP5 =>
        {
            btable_rx_count = usb_types::BTABLE_ADDRESS::EP5_COUNT_RX as usize;
            btable_rx_addr  = usb_types::BTABLE_ADDRESS::EP5_ADDR_RX as usize;
            btable_tx_count = usb_types::BTABLE_ADDRESS::EP5_COUNT_TX as usize;
            btable_tx_addr  = usb_types::BTABLE_ADDRESS::EP5_ADDR_TX as usize;
        },
        // Configures Endpoint 6 buffers and registers
        usb_types::Endpoints::EP6 =>
        {
            btable_rx_count = usb_types::BTABLE_ADDRESS::EP6_COUNT_RX as usize;
            btable_rx_addr  = usb_types::BTABLE_ADDRESS::EP6_ADDR_RX as usize;
            btable_tx_count = usb_types::BTABLE_ADDRESS::EP6_COUNT_TX as usize;
            btable_tx_addr  = usb_types::BTABLE_ADDRESS::EP6_ADDR_TX as usize;
        },
        // Configures Endpoint 7 buffers and registers
        usb_types::Endpoints::EP7 =>
        {
            btable_rx_count = usb_types::BTABLE_ADDRESS::EP7_COUNT_RX as usize;
            btable_rx_addr  = usb_types::BTABLE_ADDRESS::EP7_ADDR_RX as usize;
            btable_tx_count = usb_types::BTABLE_ADDRESS::EP7_COUNT_TX as usize;
            btable_tx_addr  = usb_types::BTABLE_ADDRESS::EP7_ADDR_TX as usize;
        }
    }

    unsafe
    {
        // === Configure EP0R Register ===
        let pma = usb_types::PMA_BASE as *mut u16;
        // COUNT_RX
        core::ptr::write_volatile(pma.add(btable_rx_count), ep.rx_count);
        // ADDR_RX
        core::ptr::write_volatile(pma.add(btable_rx_addr), ep.rx_addr);
        // COUNT_TX
        core::ptr::write_volatile(pma.add(btable_tx_count), 0);
        // ADDR_TX
        core::ptr::write_volatile(pma.add(btable_tx_addr), ep.tx_addr);
        
        // === Clean PMA ===
        let dummy = [0u8; 64];
        pma_write(ep.rx_addr, &dummy, 64);
        pma_write(ep.tx_addr, &dummy, 64);
        
        // Bits [3:0]  = EA[3:0]  → Endpoint Address = 0
        // Bits [8:9]  = EP_TYPE  → 01 = Control
        let epr = get_ep_register(0);
        *epr ^= (ep_type as u16) << (usb_types::USBEPnR::EP_TYPE as u8) |
                (usb_types::STATTX_Status::NAK as u16) << (usb_types::USBEPnR::STAT_TX as u8) |
                (usb_types::STATTX_Status::VALID as u16) << (usb_types::USBEPnR::STAT_RX as u8);
    }

    // // Enable USB peripheral
    // enable_usb_peripheral();
}